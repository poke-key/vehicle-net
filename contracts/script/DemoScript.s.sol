// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "forge-std/Script.sol";
import "../src/VehicleRegistry.sol";
import "../src/DataMarketplace.sol";
import "../src/AccessControl.sol";

contract DemoScript is Script {
    VehicleRegistry public vehicleRegistry;
    DataMarketplace public dataMarketplace;
    AccessControl public accessControl;
    
    // Demo addresses
    address public vehicleOwner = 0x70997970C51812dc3A010C7d01b50e0d17dc79C8;
    address public buyer = 0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC;
    address public vehicleWallet = 0x90F79bf6EB2c4f870365E785982E1f101E93b906;
    
    function run() external {
        // Load deployed contract addresses from environment
        address registryAddr = vm.envAddress("VEHICLE_REGISTRY_ADDRESS");
        address marketplaceAddr = vm.envAddress("DATA_MARKETPLACE_ADDRESS");
        address accessControlAddr = vm.envAddress("ACCESS_CONTROL_ADDRESS");
        
        vehicleRegistry = VehicleRegistry(registryAddr);
        dataMarketplace = DataMarketplace(marketplaceAddr);
        accessControl = AccessControl(accessControlAddr);
        
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        
        vm.startBroadcast(deployerPrivateKey);
        
        console.log("=== VEHICLE DATA MARKETPLACE DEMO ===");
        
        // Demo: Register a vehicle
        console.log("\n1. Registering vehicle...");
        
        // Fund the vehicle owner
        vm.deal(vehicleOwner, 1 ether);
        
        string[] memory dataTypes = new string[](3);
        dataTypes[0] = "GPS";
        dataTypes[1] = "Diagnostics";
        dataTypes[2] = "Fuel";
        
        vm.stopBroadcast();
        vm.startBroadcast(vm.addr(vm.deriveKey(vm.envString("MNEMONIC"), 1))); // Vehicle owner
        
        vehicleRegistry.registerVehicle{value: 0.01 ether}(
            "1HGCM82633A004352",
            vehicleWallet,
            "Tesla",
            "Model 3",
            2023,
            dataTypes,
            "QmTestVehicleMetadata123"
        );
        
        uint256 vehicleId = vehicleRegistry.getVehicleIdByVin("1HGCM82633A004352");
        console.log("Vehicle registered with ID:", vehicleId);
        
        // Demo: List a data product
        console.log("\n2. Listing data product...");
        
        dataMarketplace.listDataProduct(
            vehicleId,
            "GPS",
            0.001 ether, // 0.001 ETH per hour
            3600, // 1 hour minimum
            86400, // 24 hours maximum
            "Real-time GPS tracking data with 1-second precision",
            "https://api.vehiclenet.com/gps"
        );
        
        uint256 productId = dataMarketplace.getTotalProducts();
        console.log("Data product listed with ID:", productId);
        
        vm.stopBroadcast();
        vm.startBroadcast(vm.addr(vm.deriveKey(vm.envString("MNEMONIC"), 2))); // Buyer
        
        // Demo: Purchase data access
        console.log("\n3. Purchasing data access...");
        
        vm.deal(buyer, 1 ether);
        
        dataMarketplace.purchaseDataAccess{value: 0.002 ether}(
            productId,
            7200 // 2 hours
        );
        
        (bool hasAccess, uint256 endTime) = dataMarketplace.hasValidAccess(buyer, productId);
        console.log("Access purchased. Valid until timestamp:", endTime);
        console.log("Current timestamp:", block.timestamp);
        
        vm.stopBroadcast();
        vm.startBroadcast(deployerPrivateKey);
        
        // Demo: Create access session
        console.log("\n4. Creating access session...");
        
        bytes32 sessionKey = accessControl.createAccessSession(productId, buyer);
        console.log("Access session created with key:");
        console.logBytes32(sessionKey);
        
        // Demo: Validate access
        console.log("\n5. Validating access...");
        
        (bool isValid, uint256 timeRemaining) = accessControl.validateAccess(sessionKey, buyer);
        console.log("Session valid:", isValid);
        console.log("Time remaining (seconds):", timeRemaining);
        
        vm.stopBroadcast();
        
        console.log("\n=== DEMO COMPLETE ===");
        console.log("Vehicle registered, data product listed, access purchased, and session created!");
        console.log("The system is ready for vehicle data marketplace operations.");
    }
}