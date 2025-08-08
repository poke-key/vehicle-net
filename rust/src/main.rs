use clap::Parser;
use ethers::prelude::*;
use std::sync::Arc;

mod signer;
mod simple_contract_client;

use signer::VehicleSigner;
use simple_contract_client::SimpleContractClient;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 0)]
    index: u32,

    #[arg(long, default_value = "http://localhost:8545")]
    rpc_url: String,

    #[arg(long)]
    contract_address: Option<String>,

    #[arg(long)]
    add_node: bool,

    #[arg(long)]
    update_report: bool,

    #[arg(long)]
    list_vehicles: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    
    let args = Args::parse();
    
    // Initialize signer with vehicle index
    let signer = VehicleSigner::new(args.index)?;
    let vehicle_address = signer.get_address();
    
    println!("Vehicle {} Address: {:#x}", args.index, vehicle_address);
    
    // Connect to blockchain
    let provider = Provider::<Http>::try_from(&args.rpc_url)?;
    let client = Arc::new(SignerMiddleware::new(provider, signer.wallet.clone()));
    
    // Initialize contract client
    let contract_client = SimpleContractClient::new(client, args.contract_address).await?;
    
    if args.add_node {
        let vin = format!("VIN{:06}", args.index);
        println!("Adding vehicle node with VIN: {}", vin);
        
        match contract_client.add_vehicle_node(&vin, vehicle_address).await {
            Ok(tx_hash) => println!("Vehicle node added! Transaction: {:#x}", tx_hash),
            Err(e) => eprintln!("Failed to add vehicle node: {}", e),
        }
    }
    
    if args.update_report {
        let mileage = 10000 + (args.index * 5000) as u64;
        let battery_health = 100 - (args.index * 5) as u64;
        
        println!("Updating vehicle report - Mileage: {}, Battery: {}%", mileage, battery_health);
        
        match contract_client.update_vehicle_report(mileage, battery_health).await {
            Ok(tx_hash) => println!("Vehicle report updated! Transaction: {:#x}", tx_hash),
            Err(e) => eprintln!("Failed to update vehicle report: {}", e),
        }
    }
    
    if args.list_vehicles {
        println!("Fetching all active vehicles...");
        match contract_client.get_all_active_vehicles().await {
            Ok(vehicles) => {
                println!("Found {} active vehicles:", vehicles.len());
                for (i, vehicle) in vehicles.iter().enumerate() {
                    println!("  {}. VIN: {}, Address: {:#x}, Mileage: {}, Battery: {}%", 
                        i + 1, vehicle.vin, vehicle.node_address, vehicle.mileage, vehicle.battery_health);
                }
            },
            Err(e) => eprintln!("Failed to fetch vehicles: {}", e),
        }
    }
    
    // If no specific action, just show vehicle info
    if !args.add_node && !args.update_report && !args.list_vehicles {
        match contract_client.get_vehicle_by_address(vehicle_address).await {
            Ok(vehicle) => {
                println!("Vehicle Info:");
                println!("  VIN: {}", vehicle.vin);
                println!("  Address: {:#x}", vehicle.node_address);
                println!("  Mileage: {}", vehicle.mileage);
                println!("  Battery Health: {}%", vehicle.battery_health);
                println!("  Last Report: {}", vehicle.last_report_timestamp);
                println!("  Active: {}", vehicle.is_active);
            },
            Err(e) => println!("Vehicle not registered yet: {}", e),
        }
    }
    
    Ok(())
}