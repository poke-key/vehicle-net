// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "forge-std/Script.sol";
import "../src/VehicleRegistry.sol";
import "../src/DataMarketplace.sol";
import "../src/AccessControl.sol";

contract DeploySystemScript is Script {
    VehicleRegistry public vehicleRegistry;
    DataMarketplace public dataMarketplace;
    AccessControl public accessControl;
    
    function run() external {
        // Get the private key from environment variable
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        
        // Start broadcasting transactions
        vm.startBroadcast(deployerPrivateKey);
        
        // Deploy contracts
        console.log("Deploying Vehicle Registry...");
        vehicleRegistry = new VehicleRegistry();
        console.log("VehicleRegistry deployed at:", address(vehicleRegistry));
        
        console.log("Deploying Data Marketplace...");
        dataMarketplace = new DataMarketplace(address(vehicleRegistry));
        console.log("DataMarketplace deployed at:", address(dataMarketplace));
        
        console.log("Deploying Access Control...");
        accessControl = new AccessControl(
            address(dataMarketplace),
            address(vehicleRegistry)
        );
        console.log("AccessControl deployed at:", address(accessControl));
        
        // Stop broadcasting
        vm.stopBroadcast();
        
        // Log deployment summary
        console.log("\n=== DEPLOYMENT SUMMARY ===");
        console.log("VehicleRegistry:   ", address(vehicleRegistry));
        console.log("DataMarketplace:   ", address(dataMarketplace));
        console.log("AccessControl:     ", address(accessControl));
        console.log("============================");
        
        // Verify basic functionality
        console.log("\n=== VERIFICATION ===");
        console.log("Registry fee:      ", vehicleRegistry.registrationFee());
        console.log("Platform fee rate: ", dataMarketplace.platformFeeRate());
        console.log("Max sessions:      ", accessControl.maxConcurrentSessions());
        console.log("=====================");
    }
}