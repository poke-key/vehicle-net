use ethers::core::types::{H160, H256};
use ethers::signers::{LocalWallet, Signer};
use ethers::utils::keccak256;
use serde::{Deserialize, Serialize};

// Master mnemonic for HD wallet derivation
// This will be used for generating deterministic wallets for each vehicle
const MASTER_MNEMONIC: &str = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

#[derive(Debug, Serialize, Deserialize)]
pub struct VehicleConditionReport {
    pub vin: String,
    pub mileage: u64,
    pub battery_health: u8,
    pub timestamp: u64,
}

#[derive(Debug)]
pub struct VehicleSigner {
    pub wallet: LocalWallet,
    pub vehicle_index: u32,
    pub address: H160,
}

impl VehicleSigner {
    pub fn new(vehicle_index: u32) -> Result<Self, Box<dyn std::error::Error>> {
        // Generate deterministic private key from master mnemonic and vehicle index
        // This simulates HD wallet derivation path: m/44'/60'/0'/0/{vehicle_index}
        let seed = Self::mnemonic_to_seed(MASTER_MNEMONIC)?;
        let private_key = Self::derive_private_key(&seed, vehicle_index)?;
        
        let wallet = private_key.parse::<LocalWallet>()?;
        let address = wallet.address();

        Ok(VehicleSigner {
            wallet,
            vehicle_index,
            address,
        })
    }

    /// Convert mnemonic to seed (simplified implementation)
    fn mnemonic_to_seed(mnemonic: &str) -> Result<[u8; 64], Box<dyn std::error::Error>> {
        // In a real implementation, you'd use proper BIP39 seed generation
        // For now, we'll use a deterministic hash of the mnemonic
        let mnemonic_bytes = mnemonic.as_bytes();
        let mut seed = [0u8; 64];
        
        // Simple deterministic seed generation
        for (i, byte) in mnemonic_bytes.iter().enumerate() {
            seed[i % 64] ^= byte;
        }
        
        Ok(seed)
    }

    /// Derive private key from seed and index (simplified HD derivation)
    fn derive_private_key(seed: &[u8; 64], index: u32) -> Result<String, Box<dyn std::error::Error>> {
        // In a real implementation, you'd use proper BIP32/BIP44 derivation
        // For now, we'll create a deterministic private key based on seed + index
        let mut derived_seed = [0u8; 64];
        
        // Mix the seed with the index
        for i in 0..64 {
            derived_seed[i] = seed[i] ^ ((index >> (i % 32)) as u8);
        }
        
        // Convert to hex string (64 characters for 32 bytes)
        let private_key = hex::encode(&derived_seed[..32]);
        Ok(private_key)
    }

    pub fn sign_condition_report(&self, report: &VehicleConditionReport) -> Result<String, Box<dyn std::error::Error>> {
        let message_json = serde_json::to_string(report)?;
        let message_bytes = message_json.as_bytes();
        
        let message_hash = keccak256(message_bytes);
        let signature = self.wallet.sign_hash(H256::from(message_hash))?;
        
        Ok(format!("0x{}", hex::encode(signature.to_vec())))
    }

    pub fn get_address(&self) -> H160 {
        self.address
    }

    pub fn get_address_string(&self) -> String {
        format!("0x{:x}", self.address)
    }

    pub fn get_vehicle_index(&self) -> u32 {
        self.vehicle_index
    }

    /// Get the master mnemonic (useful for future blockchain integration)
    #[allow(dead_code)]
    pub fn get_master_mnemonic() -> &'static str {
        MASTER_MNEMONIC
    }

    /// Get the derivation path for a vehicle index
    #[allow(dead_code)]
    pub fn get_derivation_path(vehicle_index: u32) -> String {
        format!("m/44'/60'/0'/0/{}", vehicle_index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wallet_derivation() {
        let signer1 = VehicleSigner::new(0).unwrap();
        let signer2 = VehicleSigner::new(1).unwrap();
        
        assert_ne!(signer1.get_address_string(), signer2.get_address_string());
        assert_eq!(signer1.get_vehicle_index(), 0);
        assert_eq!(signer2.get_vehicle_index(), 1);
    }

    #[test]
    fn test_message_signing() {
        let signer = VehicleSigner::new(0).unwrap();
        let report = VehicleConditionReport {
            vin: "1HGCM82633A004352".to_string(),
            mileage: 120000,
            battery_health: 92,
            timestamp: 1699999999,
        };

        let signature = signer.sign_condition_report(&report).unwrap();
        assert!(signature.starts_with("0x"));
        assert!(signature.len() > 10);
    }

    #[test]
    fn test_deterministic_wallets() {
        // Same index should always produce same wallet
        let signer1 = VehicleSigner::new(5).unwrap();
        let signer2 = VehicleSigner::new(5).unwrap();
        
        assert_eq!(signer1.get_address_string(), signer2.get_address_string());
        assert_eq!(signer1.get_vehicle_index(), signer2.get_vehicle_index());
    }

    #[test]
    fn test_master_mnemonic() {
        let mnemonic = VehicleSigner::get_master_mnemonic();
        assert_eq!(mnemonic, MASTER_MNEMONIC);
        assert!(mnemonic.contains("abandon"));
    }

    #[test]
    fn test_derivation_path() {
        let path = VehicleSigner::get_derivation_path(42);
        assert_eq!(path, "m/44'/60'/0'/0/42");
    }
}