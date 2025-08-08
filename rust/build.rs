use ethers::prelude::Abigen;

fn main() {
    // Generate bindings for VehicleRegistry contract
    Abigen::new(
        "VehicleRegistry",
        "../contracts/out/VehicleRegistry.sol/VehicleRegistry.json",
    )
    .unwrap()
    .generate()
    .unwrap()
    .write_to_file("src/contracts/vehicle_registry.rs")
    .unwrap();

    // Generate bindings for DataMarketplace contract
    Abigen::new(
        "DataMarketplace",
        "../contracts/out/DataMarketplace.sol/DataMarketplace.json",
    )
    .unwrap()
    .generate()
    .unwrap()
    .write_to_file("src/contracts/data_marketplace.rs")
    .unwrap();

    // Generate bindings for AccessControl contract
    Abigen::new(
        "AccessControl",
        "../contracts/out/AccessControl.sol/AccessControl.json",
    )
    .unwrap()
    .generate()
    .unwrap()
    .write_to_file("src/contracts/access_control.rs")
    .unwrap();

    // Recompile if any of the contract files change
    println!("cargo:rerun-if-changed=../contracts/src/");
    println!("cargo:rerun-if-changed=../contracts/out/");
}