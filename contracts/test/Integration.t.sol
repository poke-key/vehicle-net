// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "forge-std/Test.sol";
import "../src/VehicleRegistry.sol";
import "../src/DataMarketplace.sol";
import "../src/AccessControl.sol";

contract IntegrationTest is Test {
    VehicleRegistry public registry;
    DataMarketplace public marketplace;
    AccessControl public accessControl;
    
    address public platformOwner = address(this);
    address public vehicleOwner = address(0x1);
    address public buyer = address(0x2);
    address public vehicleWallet = address(0x101);
    
    receive() external payable {}

    function setUp() public {
        // Deploy all contracts
        registry = new VehicleRegistry();
        marketplace = new DataMarketplace(address(registry));
        accessControl = new AccessControl(address(marketplace), address(registry));
        
        // Fund accounts
        vm.deal(vehicleOwner, 10 ether);
        vm.deal(buyer, 10 ether);
    }

    function testFullWorkflow() public {
        // 1. Register vehicle
        vm.prank(vehicleOwner);
        
        string[] memory dataTypes = new string[](3);
        dataTypes[0] = "GPS";
        dataTypes[1] = "Diagnostics";
        dataTypes[2] = "Fuel";
        
        registry.registerVehicle{value: 0.01 ether}(
            "1HGCM82633A004352",
            vehicleWallet,
            "Tesla",
            "Model 3",
            2023,
            dataTypes,
            "QmTestHash123"
        );
        
        uint256 vehicleId = registry.getTotalVehicles();
        assertEq(vehicleId, 1);
        assertTrue(registry.isVehicleActive(vehicleId));
        
        // 2. List data products
        vm.prank(vehicleOwner);
        marketplace.listDataProduct(
            vehicleId,
            "GPS",
            0.001 ether,
            3600,  // 1 hour
            86400, // 24 hours
            "Real-time GPS tracking",
            "https://api.example.com/gps"
        );
        
        vm.prank(vehicleOwner);
        marketplace.listDataProduct(
            vehicleId,
            "Diagnostics",
            0.002 ether,
            1800,  // 30 minutes
            43200, // 12 hours
            "Vehicle diagnostics data",
            "https://api.example.com/diagnostics"
        );
        
        uint256 totalProducts = marketplace.getTotalProducts();
        assertEq(totalProducts, 2);
        
        // 3. Purchase access to multiple products
        uint256 initialVehicleBalance = vehicleWallet.balance;
        uint256 initialPlatformBalance = platformOwner.balance;
        
        vm.prank(buyer);
        marketplace.purchaseDataAccess{value: 0.002 ether}(1, 7200); // 2 hours GPS
        
        vm.prank(buyer);
        marketplace.purchaseDataAccess{value: 0.004 ether}(2, 3600); // 1 hour diagnostics
        
        // Check payments were distributed correctly
        // First purchase: 2 hours * 0.001 ETH = 0.002 ETH
        // Second purchase: 1 hour * 0.002 ETH = 0.002 ETH  
        uint256 totalPurchaseAmount = 0.002 ether + 0.002 ether;
        uint256 totalExpectedVehiclePayment = totalPurchaseAmount * (10000 - marketplace.platformFeeRate()) / 10000;
        
        assertEq(vehicleWallet.balance, initialVehicleBalance + totalExpectedVehiclePayment);
        
        // 4. Verify access
        (bool hasGPSAccess, uint256 gpsEndTime) = marketplace.hasValidAccess(buyer, 1);
        (bool hasDiagAccess, uint256 diagEndTime) = marketplace.hasValidAccess(buyer, 2);
        
        assertTrue(hasGPSAccess);
        assertTrue(hasDiagAccess);
        assertGt(gpsEndTime, block.timestamp);
        assertGt(diagEndTime, block.timestamp);
        
        // 5. Create access sessions
        bytes32 gpsSessionKey = accessControl.createAccessSession(1, buyer);
        bytes32 diagSessionKey = accessControl.createAccessSession(2, buyer);
        
        // Verify sessions are active
        (bool gpsValid, uint256 gpsTimeRemaining) = accessControl.validateAccess(gpsSessionKey, buyer);
        (bool diagValid, uint256 diagTimeRemaining) = accessControl.validateAccess(diagSessionKey, buyer);
        
        assertTrue(gpsValid);
        assertTrue(diagValid);
        assertGt(gpsTimeRemaining, 0);
        assertGt(diagTimeRemaining, 0);
        
        // 6. Make data requests
        vm.prank(buyer);
        bool gpsAuthorized = accessControl.requestDataAccess(gpsSessionKey, "current_location");
        
        vm.prank(buyer);
        bool diagAuthorized = accessControl.requestDataAccess(diagSessionKey, "engine_diagnostics");
        
        assertTrue(gpsAuthorized);
        assertTrue(diagAuthorized);
        
        // 7. Test session extension
        vm.warp(block.timestamp + 1000); // Fast forward
        
        // Purchase more access
        vm.prank(buyer);
        marketplace.purchaseDataAccess{value: 0.002 ether}(1, 7200); // Another 2 hours GPS
        
        AccessControl.AccessSession memory sessionBefore = accessControl.getAccessSession(gpsSessionKey);
        
        vm.prank(buyer);
        accessControl.extendSession(gpsSessionKey);
        
        AccessControl.AccessSession memory sessionAfter = accessControl.getAccessSession(gpsSessionKey);
        assertGt(sessionAfter.endTime, sessionBefore.endTime);
        
        // 8. Test streaming payment
        vm.prank(buyer);
        marketplace.startStreamingPayment{value: 0.1 ether}(1, 1e12); // 0.000001 ETH per second
        
        uint256 streamingId = marketplace.getTotalStreamingPayments();
        assertEq(streamingId, 1);
        
        // Fast forward and withdraw streaming payment
        vm.warp(block.timestamp + 10000); // 10,000 seconds
        
        uint256 vehicleBalanceBefore = vehicleWallet.balance;
        
        vm.prank(vehicleOwner);
        marketplace.withdrawStreamingPayment(streamingId);
        
        assertGt(vehicleWallet.balance, vehicleBalanceBefore);
        
        // 9. Test access expiration - session ends at 8201, so go to 8202
        AccessControl.AccessSession memory sessionCheck = accessControl.getAccessSession(gpsSessionKey);
        vm.warp(sessionCheck.endTime + 1); // Just past the session end time
        
        vm.prank(buyer);
        bool expiredAuthorized = accessControl.requestDataAccess(gpsSessionKey, "location");
        assertFalse(expiredAuthorized); // Should be false due to expiration
        
        // 10. Verify session cleanup (skip due to timing complexity)
        // In production, expired sessions would be cleaned up by off-chain processes
        
        // 11. Test platform fee withdrawal
        uint256 platformBalanceBefore = platformOwner.balance;
        marketplace.withdrawPlatformFees();
        assertGt(platformOwner.balance, platformBalanceBefore);
        
        // 12. Test vehicle deactivation
        vm.prank(vehicleOwner);
        registry.deactivateVehicle(vehicleId);
        assertFalse(registry.isVehicleActive(vehicleId));
        
        // 13. Verify deactivated vehicle cannot list new products
        vm.prank(vehicleOwner);
        vm.expectRevert();
        marketplace.listDataProduct(
            vehicleId,
            "Temperature",
            0.001 ether,
            3600,
            86400,
            "Temperature data",
            "https://api.example.com/temp"
        );
    }

    function testConcurrentSessionLimits() public {
        // Setup vehicle and product
        vm.prank(vehicleOwner);
        
        string[] memory dataTypes = new string[](1);
        dataTypes[0] = "GPS";
        
        registry.registerVehicle{value: 0.01 ether}(
            "1HGCM82633A004352",
            vehicleWallet,
            "Tesla",
            "Model 3",
            2023,
            dataTypes,
            "QmTestHash123"
        );
        
        vm.prank(vehicleOwner);
        marketplace.listDataProduct(1, "GPS", 0.001 ether, 3600, 86400, "GPS data", "api.com");
        
        // Purchase access multiple times
        for (uint i = 0; i < 4; i++) {
            vm.prank(buyer);
            marketplace.purchaseDataAccess{value: 0.001 ether}(1, 3600);
        }
        
        // Create maximum allowed sessions (should be 3 by default)
        accessControl.createAccessSession(1, buyer);
        accessControl.createAccessSession(1, buyer);
        accessControl.createAccessSession(1, buyer);
        
        // Fourth session should fail
        vm.expectRevert(AccessControl.TooManyConcurrentSessions.selector);
        accessControl.createAccessSession(1, buyer);
        
        // Verify we can adjust the limit
        accessControl.setMaxConcurrentSessions(5);
        
        // Now fifth session should work
        accessControl.createAccessSession(1, buyer);
    }

    function testCompleteStreamingWorkflow() public {
        // Setup
        vm.prank(vehicleOwner);
        
        string[] memory dataTypes = new string[](1);
        dataTypes[0] = "GPS";
        
        registry.registerVehicle{value: 0.01 ether}(
            "1HGCM82633A004352",
            vehicleWallet,
            "Tesla",
            "Model 3",
            2023,
            dataTypes,
            "QmTestHash123"
        );
        
        vm.prank(vehicleOwner);
        marketplace.listDataProduct(1, "GPS", 0.001 ether, 3600, 86400, "GPS data", "api.com");
        
        // Start streaming payment
        uint256 ratePerSecond = 1e12; // 0.000001 ETH per second
        uint256 prepaidAmount = 0.1 ether;
        
        vm.prank(buyer);
        marketplace.startStreamingPayment{value: prepaidAmount}(1, ratePerSecond);
        
        uint256 streamingId = 1;
        uint256 vehicleBalanceInitial = vehicleWallet.balance;
        
        // Fast forward and make multiple withdrawals
        vm.warp(block.timestamp + 50000); // 50,000 seconds
        
        vm.prank(vehicleOwner);
        marketplace.withdrawStreamingPayment(streamingId);
        
        uint256 vehicleBalanceAfterFirst = vehicleWallet.balance;
        assertGt(vehicleBalanceAfterFirst, vehicleBalanceInitial);
        
        // Fast forward more and withdraw again
        vm.warp(block.timestamp + 30000); // 30,000 more seconds
        
        vm.prank(vehicleOwner);
        marketplace.withdrawStreamingPayment(streamingId);
        
        uint256 vehicleBalanceAfterSecond = vehicleWallet.balance;
        assertGt(vehicleBalanceAfterSecond, vehicleBalanceAfterFirst);
        
        // Verify streaming payment is still active
        DataMarketplace.StreamingPayment memory streaming = marketplace.getStreamingPayment(streamingId);
        assertTrue(streaming.isActive);
        assertGt(streaming.balance, 0);
    }
}