mod signer;

use clap::Parser;
use signer::{VehicleConditionReport, VehicleSigner};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Parser)]
#[command(name = "vehicle-net")]
#[command(about = "A decentralized vehicle condition reporting system")]
struct Args {
    #[arg(long, help = "Vehicle index for HD wallet derivation")]
    index: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    println!("🚗 Vehicle Network - Condition Report Signer");
    println!("============================================");
    
    let signer = VehicleSigner::new(args.index)?;
    
    println!("Vehicle Index: {}", signer.get_vehicle_index());
    println!("Derived Address: {}", signer.get_address());
    
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_secs();
    
    let condition_report = VehicleConditionReport {
        vin: "1HGCM82633A004352".to_string(),
        mileage: 120000,
        battery_health: 92,
        timestamp,
    };
    
    let message_json = serde_json::to_string_pretty(&condition_report)?;
    println!("\n📋 Vehicle Condition Report:");
    println!("{}", message_json);
    
    let signature = signer.sign_condition_report(&condition_report)?;
    
    println!("\n✍️  Signature:");
    println!("{}", signature);
    
    println!("\n✅ Successfully signed condition report for vehicle #{}", args.index);
    println!("💡 Try running with different --index values to simulate other vehicles");
    
    Ok(())
}