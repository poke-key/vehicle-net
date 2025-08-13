use ethers::{
    prelude::*,
    providers::{Http, Provider},
    signers::LocalWallet,
    types::{Address, U256},
};
use std::sync::Arc;
use anyhow::{Context, Result};
use tracing::info;

use crate::contracts::{VehicleRegistry, DataMarketplace, AccessControl, vehicle_registry::Vehicle};
use crate::signer::{VehicleConditionReport, VehicleSigner};

#[derive(thiserror::Error, Debug)]
pub enum ContractError {
    #[error("Provider error: {0}")]
    Provider(#[from] ethers::providers::ProviderError),
    #[error("Contract call failed: {0}")]
    Contract(String),
    #[error("Invalid address: {0}")]
    InvalidAddress(String),
}

#[derive(Clone)]
pub struct ContractConfig {
    pub rpc_url: String,
    pub vehicle_registry_address: Address,
    pub data_marketplace_address: Address,
    pub access_control_address: Address,
    pub chain_id: u64,
}

impl ContractConfig {
    pub fn localhost() -> Self {
        Self {
            rpc_url: "http://127.0.0.1:8545".to_string(),
            vehicle_registry_address: Address::zero(), // Will be set after deployment
            data_marketplace_address: Address::zero(),
            access_control_address: Address::zero(),
            chain_id: 31337, // Anvil default chain ID
        }
    }

}

pub struct VehicleNetworkClient {
    provider: Arc<SignerMiddleware<Provider<Http>, LocalWallet>>,
    vehicle_registry: VehicleRegistry<SignerMiddleware<Provider<Http>, LocalWallet>>,
    data_marketplace: DataMarketplace<SignerMiddleware<Provider<Http>, LocalWallet>>,
    #[allow(dead_code)]
    access_control: AccessControl<SignerMiddleware<Provider<Http>, LocalWallet>>,
    vehicle_signer: VehicleSigner,
    #[allow(dead_code)]
    config: ContractConfig,
}

impl VehicleNetworkClient {
    pub async fn new(config: ContractConfig, vehicle_index: u32) -> Result<Self> {
        info!("Connecting to blockchain at {}", config.rpc_url);
        
        // Create provider
        let provider = Provider::<Http>::try_from(&config.rpc_url)
            .context("Failed to create provider")?
            .interval(std::time::Duration::from_millis(10u64));

        // Create vehicle signer
        let vehicle_signer = VehicleSigner::new(vehicle_index)
            .map_err(|e| anyhow::anyhow!("Failed to create vehicle signer: {}", e))?;

        // Parse wallet and connect to provider
        let wallet: LocalWallet = vehicle_signer.wallet.clone().with_chain_id(config.chain_id);
        let provider = Arc::new(SignerMiddleware::new(provider, wallet));

        // Create contract instances
        let vehicle_registry = VehicleRegistry::new(config.vehicle_registry_address, provider.clone());
        let data_marketplace = DataMarketplace::new(config.data_marketplace_address, provider.clone());
        let access_control = AccessControl::new(config.access_control_address, provider.clone());

        info!("Initialized VehicleNetworkClient for vehicle index {}", vehicle_index);
        info!("Vehicle address: {}", vehicle_signer.get_address());

        Ok(Self {
            provider,
            vehicle_registry,
            data_marketplace,
            access_control,
            vehicle_signer,
            config,
        })
    }

    pub fn get_vehicle_address(&self) -> String {
        self.vehicle_signer.get_address()
    }

    pub fn get_vehicle_index(&self) -> u32 {
        self.vehicle_signer.get_vehicle_index()
    }

    pub async fn get_balance(&self) -> Result<U256> {
        let address: Address = self.vehicle_signer.get_address().parse()
            .map_err(|e| ContractError::InvalidAddress(format!("Invalid address format: {}", e)))?;
        
        let balance = self.provider.get_balance(address, None).await?;
        Ok(balance)
    }

    pub async fn register_vehicle(
        &self,
        vin: &str,
        manufacturer: &str,
        model: &str,
        year: u64,
        data_types: Vec<String>,
        ipfs_hash: &str,
        registration_fee: U256,
    ) -> Result<TransactionReceipt> {
        info!("Registering vehicle with VIN: {}", vin);
        
        let vehicle_address: Address = self.vehicle_signer.get_address().parse()
            .map_err(|e| ContractError::InvalidAddress(format!("Invalid vehicle address: {}", e)))?;

        let tx = self.vehicle_registry
            .register_vehicle(
                vin.to_string(),
                vehicle_address,
                manufacturer.to_string(),
                model.to_string(),
                U256::from(year),
                data_types,
                ipfs_hash.to_string(),
            )
            .value(registration_fee);

        let pending_tx = tx.send().await
            .map_err(|e| ContractError::Contract(format!("Failed to send registration transaction: {}", e)))?;

        let receipt = pending_tx.await?
            .ok_or_else(|| ContractError::Contract("Transaction receipt not found".to_string()))?;

        info!("Vehicle registered successfully. Transaction hash: {:?}", receipt.transaction_hash);
        Ok(receipt)
    }

    pub async fn get_vehicle_by_id(&self, vehicle_id: U256) -> Result<Vehicle> {
        let vehicle = self.vehicle_registry.get_vehicle(vehicle_id).call().await
            .map_err(|e| ContractError::Contract(format!("Failed to get vehicle: {}", e)))?;
        
        Ok(vehicle)
    }

    pub async fn get_vehicle_id_by_wallet(&self, wallet_address: Address) -> Result<U256> {
        let vehicle_id = self.vehicle_registry.get_vehicle_id_by_wallet(wallet_address).call().await
            .map_err(|e| ContractError::Contract(format!("Failed to get vehicle ID: {}", e)))?;
        
        Ok(vehicle_id)
    }

    pub async fn submit_condition_report(&self, report: &VehicleConditionReport) -> Result<String> {
        info!("Submitting condition report for VIN: {}", report.vin);
        
        // Sign the condition report
        let signature = self.vehicle_signer.sign_condition_report(report)
            .map_err(|e| anyhow::anyhow!("Failed to sign condition report: {}", e))?;

        // In a real implementation, you would submit this to an off-chain service
        // or store it in IPFS and update the vehicle metadata
        info!("Condition report signed successfully");
        info!("Report: {}", serde_json::to_string_pretty(report)?);
        info!("Signature: {}", signature);

        Ok(signature)
    }

    pub async fn list_data_product(
        &self,
        vehicle_id: U256,
        data_type: &str,
        price_per_hour: U256,
        min_duration: u64,
        max_duration: u64,
        description: &str,
        api_endpoint: &str,
    ) -> Result<TransactionReceipt> {
        info!("Listing data product for vehicle ID: {}", vehicle_id);

        let tx = self.data_marketplace.list_data_product(
            vehicle_id,
            data_type.to_string(),
            price_per_hour,
            U256::from(min_duration),
            U256::from(max_duration),
            description.to_string(),
            api_endpoint.to_string(),
        );

        let pending_tx = tx.send().await
            .map_err(|e| ContractError::Contract(format!("Failed to list data product: {}", e)))?;

        let receipt = pending_tx.await?
            .ok_or_else(|| ContractError::Contract("Transaction receipt not found".to_string()))?;

        info!("Data product listed successfully. Transaction hash: {:?}", receipt.transaction_hash);
        Ok(receipt)
    }

    #[allow(dead_code)]
    pub async fn purchase_data_access(
        &self,
        product_id: U256,
        duration_in_seconds: u64,
        payment_amount: U256,
    ) -> Result<TransactionReceipt> {
        info!("Purchasing data access for product ID: {}", product_id);

        let tx = self.data_marketplace
            .purchase_data_access(product_id, U256::from(duration_in_seconds))
            .value(payment_amount);

        let pending_tx = tx.send().await
            .map_err(|e| ContractError::Contract(format!("Failed to purchase data access: {}", e)))?;

        let receipt = pending_tx.await?
            .ok_or_else(|| ContractError::Contract("Transaction receipt not found".to_string()))?;

        info!("Data access purchased successfully. Transaction hash: {:?}", receipt.transaction_hash);
        Ok(receipt)
    }

    #[allow(dead_code)]
    pub async fn has_valid_access(&self, buyer: Address, product_id: U256) -> Result<(bool, U256)> {
        let access = self.data_marketplace.has_valid_access(buyer, product_id).call().await
            .map_err(|e| ContractError::Contract(format!("Failed to check access: {}", e)))?;
        
        Ok(access)
    }

    #[allow(dead_code)]
    pub async fn create_access_session(&self, product_id: U256, user: Address) -> Result<[u8; 32]> {
        info!("Creating access session for product ID: {}", product_id);

        let tx = self.access_control.create_access_session(product_id, user);
        let session_key = tx.call().await
            .map_err(|e| ContractError::Contract(format!("Failed to create access session: {}", e)))?;

        info!("Access session created successfully");
        Ok(session_key)
    }

    #[allow(dead_code)]
    pub async fn validate_access(&self, session_key: [u8; 32], user: Address) -> Result<(bool, U256)> {
        let validation = self.access_control.validate_access(session_key, user).call().await
            .map_err(|e| ContractError::Contract(format!("Failed to validate access: {}", e)))?;
        
        Ok(validation)
    }

    pub async fn get_chain_id(&self) -> Result<U256> {
        let chain_id = self.provider.get_chainid().await?;
        Ok(chain_id)
    }

    pub async fn get_block_number(&self) -> Result<U64> {
        let block_number = self.provider.get_block_number().await?;
        Ok(block_number)
    }

    #[allow(dead_code)]
    pub fn get_provider(&self) -> Arc<SignerMiddleware<Provider<Http>, LocalWallet>> {
        self.provider.clone()
    }
}