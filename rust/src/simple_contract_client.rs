use ethers::prelude::*;
use std::sync::Arc;

// ABI for SimpleVehicleNodes contract
abigen!(
    SimpleVehicleNodes,
    r#"[
        function addVehicleNode(string memory vin, address nodeAddress) external
        function updateVehicleReport(uint256 mileage, uint256 batteryHealth) external  
        function getVehicle(uint256 vehicleId) external view returns ((string,address,uint256,uint256,uint256,bool))
        function getVehicleByAddress(address nodeAddress) external view returns ((string,address,uint256,uint256,uint256,bool))
        function getTotalVehicles() external view returns (uint256)
        event VehicleNodeAdded(uint256 indexed vehicleId, string vin, address indexed nodeAddress)
        event VehicleReportUpdated(uint256 indexed vehicleId, uint256 mileage, uint256 batteryHealth)
    ]"#,
);

#[derive(Clone, Debug)]
pub struct VehicleNode {
    pub vin: String,
    pub node_address: Address,
    pub mileage: u64,
    pub battery_health: u64,
    pub last_report_timestamp: u64,
    pub is_active: bool,
}

pub struct SimpleContractClient<M> {
    contract: SimpleVehicleNodes<M>,
    client: Arc<M>,
}

impl<M: Middleware + 'static> SimpleContractClient<M> {
    pub async fn new(client: Arc<M>, contract_address: Option<String>) -> Result<Self, Box<dyn std::error::Error>> {
        let address = if let Some(addr_str) = contract_address {
            addr_str.parse::<Address>()?
        } else {
            // Default contract address for local deployment
            "0x5FbDB2315678afecb367f032d93F642f64180aa3".parse::<Address>()?
        };

        let contract = SimpleVehicleNodes::new(address, client.clone());
        
        Ok(Self {
            contract,
            client,
        })
    }

    pub async fn add_vehicle_node(&self, vin: &str, node_address: Address) -> Result<H256, Box<dyn std::error::Error>> {
        let call = self.contract.add_vehicle_node(vin.to_string(), node_address);
        let pending_tx = call.send().await?;
        let receipt = pending_tx.await?;
        
        Ok(receipt.unwrap().transaction_hash)
    }

    pub async fn update_vehicle_report(&self, mileage: u64, battery_health: u64) -> Result<H256, Box<dyn std::error::Error>> {
        let call = self.contract.update_vehicle_report(mileage.into(), battery_health.into());
        let pending_tx = call.send().await?;
        let receipt = pending_tx.await?;
        
        Ok(receipt.unwrap().transaction_hash)
    }

    pub async fn get_vehicle(&self, vehicle_id: u64) -> Result<VehicleNode, Box<dyn std::error::Error>> {
        let result = self.contract
            .get_vehicle(vehicle_id.into())
            .call()
            .await?;
        
        Ok(VehicleNode {
            vin: result.0,
            node_address: result.1,
            mileage: result.2.as_u64(),
            battery_health: result.3.as_u64(),
            last_report_timestamp: result.4.as_u64(),
            is_active: result.5,
        })
    }

    pub async fn get_vehicle_by_address(&self, node_address: Address) -> Result<VehicleNode, Box<dyn std::error::Error>> {
        let result = self.contract
            .get_vehicle_by_address(node_address)
            .call()
            .await?;
        
        Ok(VehicleNode {
            vin: result.0,
            node_address: result.1,
            mileage: result.2.as_u64(),
            battery_health: result.3.as_u64(),
            last_report_timestamp: result.4.as_u64(),
            is_active: result.5,
        })
    }

    pub async fn get_all_active_vehicles(&self) -> Result<Vec<VehicleNode>, Box<dyn std::error::Error>> {
        let total = self.get_total_vehicles().await?;
        let mut vehicles = Vec::new();
        
        for i in 1..=total {
            match self.get_vehicle(i).await {
                Ok(vehicle) => {
                    if vehicle.is_active {
                        vehicles.push(vehicle);
                    }
                },
                Err(_) => continue,
            }
        }
        
        Ok(vehicles)
    }

    pub async fn get_total_vehicles(&self) -> Result<u64, Box<dyn std::error::Error>> {
        let result = self.contract
            .get_total_vehicles()
            .call()
            .await?;
        
        Ok(result.as_u64())
    }
}