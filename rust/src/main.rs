mod signer;
mod contract_client;
mod contracts;

use clap::{Parser, Subcommand};
use ethers::types::{Address, U256};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{info, Level};
use tracing_subscriber;

use crate::contract_client::{ContractConfig, VehicleNetworkClient};
use crate::signer::{VehicleConditionReport, VehicleSigner};

#[derive(Parser)]
#[command(name = "vehicle-net")]
#[command(about = "A decentralized vehicle condition reporting system")]
struct Args {
    #[arg(long, help = "Vehicle index for HD wallet derivation")]
    index: u32,
    
    #[arg(long, help = "RPC URL for blockchain connection", default_value = "http://127.0.0.1:8545")]
    rpc_url: String,
    
    #[arg(long, help = "VehicleRegistry contract address")]
    registry_address: Option<String>,
    
    #[arg(long, help = "DataMarketplace contract address")]
    marketplace_address: Option<String>,
    
    #[arg(long, help = "AccessControl contract address")]
    access_control_address: Option<String>,
    
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate and sign a vehicle condition report
    SignReport {
        #[arg(long, help = "Vehicle VIN")]
        vin: Option<String>,
        #[arg(long, help = "Mileage")]
        mileage: Option<u64>,
        #[arg(long, help = "Battery health percentage")]
        battery_health: Option<u8>,
    },
    /// Register vehicle on blockchain
    RegisterVehicle {
        #[arg(long, help = "Vehicle VIN")]
        vin: String,
        #[arg(long, help = "Manufacturer")]
        manufacturer: String,
        #[arg(long, help = "Model")]
        model: String,
        #[arg(long, help = "Year")]
        year: u64,
        #[arg(long, help = "Registration fee in ETH", default_value = "0.01")]
        fee: f64,
    },
    /// Submit condition report to blockchain
    SubmitReport {
        #[arg(long, help = "Vehicle VIN")]
        vin: Option<String>,
        #[arg(long, help = "Mileage")]
        mileage: Option<u64>,
        #[arg(long, help = "Battery health percentage")]
        battery_health: Option<u8>,
    },
    /// List data product on marketplace
    ListData {
        #[arg(long, help = "Data type (GPS, diagnostics, etc.)")]
        data_type: String,
        #[arg(long, help = "Price per hour in ETH")]
        price: f64,
        #[arg(long, help = "Minimum duration in seconds", default_value = "3600")]
        min_duration: u64,
        #[arg(long, help = "Maximum duration in seconds", default_value = "86400")]
        max_duration: u64,
        #[arg(long, help = "Description")]
        description: String,
    },
    /// Get vehicle information
    GetVehicle {
        #[arg(long, help = "Vehicle ID")]
        id: Option<u64>,
        #[arg(long, help = "Vehicle wallet address")]
        address: Option<String>,
    },
    /// Get account balance
    Balance,
    /// Get blockchain info
    Info,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    println!("🚗 Vehicle Network - Decentralized Vehicle System");
    println!("================================================");
    
    // Create contract configuration
    let mut config = ContractConfig::localhost();
    config.rpc_url = args.rpc_url;
    
    // Parse contract addresses if provided
    if let Some(addr) = args.registry_address {
        config.vehicle_registry_address = addr.parse()
            .map_err(|_| "Invalid registry address format")?;
    }
    if let Some(addr) = args.marketplace_address {
        config.data_marketplace_address = addr.parse()
            .map_err(|_| "Invalid marketplace address format")?;
    }
    if let Some(addr) = args.access_control_address {
        config.access_control_address = addr.parse()
            .map_err(|_| "Invalid access control address format")?;
    }

    // Handle different commands
    match args.command {
        Commands::SignReport { vin, mileage, battery_health } => {
            sign_report_command(args.index, vin, mileage, battery_health).await?;
        }
        
        Commands::RegisterVehicle { vin, manufacturer, model, year, fee } => {
            register_vehicle_command(config, args.index, vin, manufacturer, model, year, fee).await?;
        }
        
        Commands::SubmitReport { vin, mileage, battery_health } => {
            submit_report_command(config, args.index, vin, mileage, battery_health).await?;
        }
        
        Commands::ListData { data_type, price, min_duration, max_duration, description } => {
            list_data_command(config, args.index, data_type, price, min_duration, max_duration, description).await?;
        }
        
        Commands::GetVehicle { id, address } => {
            get_vehicle_command(config, args.index, id, address).await?;
        }
        
        Commands::Balance => {
            balance_command(config, args.index).await?;
        }
        
        Commands::Info => {
            info_command(config, args.index).await?;
        }
    }
    
    Ok(())
}

async fn sign_report_command(
    vehicle_index: u32,
    vin: Option<String>,
    mileage: Option<u64>,
    battery_health: Option<u8>,
) -> Result<(), Box<dyn std::error::Error>> {
    let signer = VehicleSigner::new(vehicle_index)?;
    
    println!("Vehicle Index: {}", signer.get_vehicle_index());
    println!("Derived Address: {}", signer.get_address());
    
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_secs();
    
    let condition_report = VehicleConditionReport {
        vin: vin.unwrap_or_else(|| "1HGCM82633A004352".to_string()),
        mileage: mileage.unwrap_or(120000),
        battery_health: battery_health.unwrap_or(92),
        timestamp,
    };
    
    let message_json = serde_json::to_string_pretty(&condition_report)?;
    println!("\n📋 Vehicle Condition Report:");
    println!("{}", message_json);
    
    let signature = signer.sign_condition_report(&condition_report)?;
    
    println!("\n✍️  Signature:");
    println!("{}", signature);
    
    println!("\n✅ Successfully signed condition report for vehicle #{}", vehicle_index);
    Ok(())
}

async fn register_vehicle_command(
    config: ContractConfig,
    vehicle_index: u32,
    vin: String,
    manufacturer: String,
    model: String,
    year: u64,
    fee: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Connecting to blockchain...");
    let client = VehicleNetworkClient::new(config, vehicle_index).await?;
    
    println!("Vehicle Address: {}", client.get_vehicle_address());
    
    let registration_fee = ethers::utils::parse_ether(fee)?;
    let data_types = vec!["GPS".to_string(), "Diagnostics".to_string(), "Fuel".to_string()];
    let ipfs_hash = "QmExampleHash123";
    
    let receipt = client.register_vehicle(
        &vin,
        &manufacturer,
        &model,
        year,
        data_types,
        ipfs_hash,
        registration_fee,
    ).await?;
    
    println!("✅ Vehicle registered successfully!");
    println!("Transaction Hash: {:?}", receipt.transaction_hash);
    println!("Gas Used: {:?}", receipt.gas_used);
    
    Ok(())
}

async fn submit_report_command(
    config: ContractConfig,
    vehicle_index: u32,
    vin: Option<String>,
    mileage: Option<u64>,
    battery_health: Option<u8>,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Connecting to blockchain...");
    let client = VehicleNetworkClient::new(config, vehicle_index).await?;
    
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_secs();
    
    let condition_report = VehicleConditionReport {
        vin: vin.unwrap_or_else(|| "1HGCM82633A004352".to_string()),
        mileage: mileage.unwrap_or(120000),
        battery_health: battery_health.unwrap_or(92),
        timestamp,
    };
    
    let signature = client.submit_condition_report(&condition_report).await?;
    
    println!("✅ Condition report submitted successfully!");
    println!("Signature: {}", signature);
    
    Ok(())
}

async fn list_data_command(
    config: ContractConfig,
    vehicle_index: u32,
    data_type: String,
    price: f64,
    min_duration: u64,
    max_duration: u64,
    description: String,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Connecting to blockchain...");
    let client = VehicleNetworkClient::new(config, vehicle_index).await?;
    
    let vehicle_address: Address = client.get_vehicle_address().parse()?;
    let vehicle_id = client.get_vehicle_id_by_wallet(vehicle_address).await?;
    
    if vehicle_id == U256::zero() {
        return Err("Vehicle not registered".into());
    }
    
    let price_per_hour = ethers::utils::parse_ether(price)?;
    let api_endpoint = format!("https://api.vehicle-net.com/data/{}", vehicle_index);
    
    let receipt = client.list_data_product(
        vehicle_id,
        &data_type,
        price_per_hour,
        min_duration,
        max_duration,
        &description,
        &api_endpoint,
    ).await?;
    
    println!("✅ Data product listed successfully!");
    println!("Transaction Hash: {:?}", receipt.transaction_hash);
    
    Ok(())
}

async fn get_vehicle_command(
    config: ContractConfig,
    vehicle_index: u32,
    id: Option<u64>,
    address: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Connecting to blockchain...");
    let client = VehicleNetworkClient::new(config, vehicle_index).await?;
    
    let vehicle_id = if let Some(id) = id {
        U256::from(id)
    } else if let Some(addr) = address {
        let address: Address = addr.parse()?;
        client.get_vehicle_id_by_wallet(address).await?
    } else {
        let vehicle_address: Address = client.get_vehicle_address().parse()?;
        client.get_vehicle_id_by_wallet(vehicle_address).await?
    };
    
    if vehicle_id == U256::zero() {
        println!("❌ Vehicle not found");
        return Ok(());
    }
    
    let vehicle = client.get_vehicle_by_id(vehicle_id).await?;
    
    println!("🚗 Vehicle Information:");
    println!("ID: {}", vehicle_id);
    println!("VIN: {}", vehicle.vin);
    println!("Wallet: {:?}", vehicle.wallet);
    println!("Manufacturer: {}", vehicle.manufacturer);
    println!("Model: {}", vehicle.model);
    println!("Year: {}", vehicle.year);
    println!("Active: {}", vehicle.is_active);
    println!("Registration Time: {}", vehicle.registration_timestamp);
    println!("Owner: {:?}", vehicle.owner);
    
    Ok(())
}

async fn balance_command(
    config: ContractConfig,
    vehicle_index: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Connecting to blockchain...");
    let client = VehicleNetworkClient::new(config, vehicle_index).await?;
    
    let balance = client.get_balance().await?;
    let balance_eth = ethers::utils::format_ether(balance);
    
    println!("💰 Account Balance:");
    println!("Address: {}", client.get_vehicle_address());
    println!("Balance: {} ETH", balance_eth);
    
    Ok(())
}

async fn info_command(
    config: ContractConfig,
    vehicle_index: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Connecting to blockchain...");
    let client = VehicleNetworkClient::new(config.clone(), vehicle_index).await?;
    
    let chain_id = client.get_chain_id().await?;
    let block_number = client.get_block_number().await?;
    
    println!("ℹ️  Blockchain Information:");
    println!("RPC URL: {}", config.rpc_url);
    println!("Chain ID: {}", chain_id);
    println!("Block Number: {}", block_number);
    println!("Vehicle Address: {}", client.get_vehicle_address());
    println!("Vehicle Index: {}", client.get_vehicle_index());
    
    Ok(())
}