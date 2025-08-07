// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "forge-std/Test.sol";
import "../src/VehicleRegistry.sol";
import "../src/DataMarketplace.sol";
import "../src/AccessControl.sol";

contract AccessControlTest is Test {
    VehicleRegistry public registry;
    DataMarketplace public marketplace;
    AccessControl public accessControl;
    
    address public owner = address(this);
    address public vehicleOwner = address(0x1);
    address public buyer = address(0x2);
    address public vehicleWallet = address(0x101);
    
    string constant VIN = "1HGCM82633A004352";
    string constant MANUFACTURER = "Toyota";
    string constant MODEL = "Camry";
    uint256 constant YEAR = 2023;
    
    string[] dataTypes;
    string constant IPFS_HASH = "QmTestHash123";
    
    string constant DATA_TYPE = "GPS";
    uint256 constant PRICE_PER_HOUR = 0.001 ether;
    uint256 constant MIN_DURATION = 3600; // 1 hour
    uint256 constant MAX_DURATION = 86400; // 24 hours
    string constant DESCRIPTION = "Real-time GPS tracking data";
    string constant API_ENDPOINT = "https://api.example.com/gps";

    function setUp() public {
        registry = new VehicleRegistry();
        marketplace = new DataMarketplace(address(registry));
        accessControl = new AccessControl(address(marketplace), address(registry));
        
        dataTypes.push("GPS");
        dataTypes.push("Diagnostics");
        
        vm.deal(vehicleOwner, 1 ether);
        vm.deal(buyer, 1 ether);
        
        // Register a vehicle
        vm.prank(vehicleOwner);
        registry.registerVehicle{value: 0.01 ether}(
            VIN,
            vehicleWallet,
            MANUFACTURER,
            MODEL,
            YEAR,
            dataTypes,
            IPFS_HASH
        );
        
        // List a data product
        vm.prank(vehicleOwner);
        marketplace.listDataProduct(
            1,
            DATA_TYPE,
            PRICE_PER_HOUR,
            MIN_DURATION,
            MAX_DURATION,
            DESCRIPTION,
            API_ENDPOINT
        );
        
        // Purchase access
        vm.prank(buyer);
        marketplace.purchaseDataAccess{value: PRICE_PER_HOUR}(1, MIN_DURATION);
    }

    function testCreateAccessSession() public {
        bytes32 sessionKey = accessControl.createAccessSession(1, buyer);
        
        AccessControl.AccessSession memory session = accessControl.getAccessSession(sessionKey);
        
        assertEq(session.user, buyer);
        assertEq(session.productId, 1);
        assertTrue(session.isActive);
        assertGt(session.endTime, block.timestamp);
        assertEq(session.sessionKey, sessionKey);
    }

    function testCannotCreateSessionWithoutAccess() public {
        address unauthorizedUser = address(0x999);
        vm.expectRevert(AccessControl.UnauthorizedAccess.selector);
        accessControl.createAccessSession(1, unauthorizedUser);
    }

    function testRequestDataAccess() public {
        bytes32 sessionKey = accessControl.createAccessSession(1, buyer);
        
        vm.prank(buyer);
        bool authorized = accessControl.requestDataAccess(sessionKey, "location");
        
        assertTrue(authorized);
        
        AccessControl.AccessSession memory session = accessControl.getAccessSession(sessionKey);
        assertEq(session.dataRequests, 1);
    }

    function testCannotRequestDataAccessAsWrongUser() public {
        bytes32 sessionKey = accessControl.createAccessSession(1, buyer);
        
        address wrongUser = address(0x999);
        vm.prank(wrongUser);
        vm.expectRevert(AccessControl.UnauthorizedAccess.selector);
        accessControl.requestDataAccess(sessionKey, "location");
    }

    function testRequestDataAccessAfterExpiration() public {
        bytes32 sessionKey = accessControl.createAccessSession(1, buyer);
        
        // Fast forward past the access period
        vm.warp(block.timestamp + MIN_DURATION + 1);
        
        vm.prank(buyer);
        bool authorized = accessControl.requestDataAccess(sessionKey, "location");
        
        assertFalse(authorized);
    }

    function testTerminateSession() public {
        bytes32 sessionKey = accessControl.createAccessSession(1, buyer);
        
        vm.prank(buyer);
        accessControl.terminateSession(sessionKey, "User terminated");
        
        AccessControl.AccessSession memory session = accessControl.getAccessSession(sessionKey);
        assertFalse(session.isActive);
    }

    function testOwnerCanTerminateAnySession() public {
        bytes32 sessionKey = accessControl.createAccessSession(1, buyer);
        
        vm.prank(owner);
        accessControl.terminateSession(sessionKey, "Admin terminated");
        
        AccessControl.AccessSession memory session = accessControl.getAccessSession(sessionKey);
        assertFalse(session.isActive);
    }

    function testCannotTerminateSessionAsUnauthorizedUser() public {
        bytes32 sessionKey = accessControl.createAccessSession(1, buyer);
        
        address unauthorizedUser = address(0x999);
        vm.prank(unauthorizedUser);
        vm.expectRevert(AccessControl.UnauthorizedAccess.selector);
        accessControl.terminateSession(sessionKey, "Unauthorized attempt");
    }

    function testExtendSession() public {
        bytes32 sessionKey = accessControl.createAccessSession(1, buyer);
        
        AccessControl.AccessSession memory sessionBefore = accessControl.getAccessSession(sessionKey);
        
        // Wait 10 seconds and purchase additional access
        vm.warp(block.timestamp + 10);
        vm.prank(buyer);
        marketplace.purchaseDataAccess{value: PRICE_PER_HOUR}(1, MIN_DURATION);
        
        vm.prank(buyer);
        accessControl.extendSession(sessionKey);
        
        AccessControl.AccessSession memory sessionAfter = accessControl.getAccessSession(sessionKey);
        assertGt(sessionAfter.endTime, sessionBefore.endTime);
    }

    function testCannotExtendSessionWithoutValidAccess() public {
        bytes32 sessionKey = accessControl.createAccessSession(1, buyer);
        
        // Wait for access to expire
        vm.warp(block.timestamp + MIN_DURATION + 1);
        
        vm.prank(buyer);
        vm.expectRevert(AccessControl.UnauthorizedAccess.selector);
        accessControl.extendSession(sessionKey);
    }

    function testValidateAccess() public {
        bytes32 sessionKey = accessControl.createAccessSession(1, buyer);
        
        (bool isValid, uint256 timeRemaining) = accessControl.validateAccess(sessionKey, buyer);
        
        assertTrue(isValid);
        assertGt(timeRemaining, 0);
    }

    function testValidateAccessForInactiveSession() public {
        bytes32 sessionKey = accessControl.createAccessSession(1, buyer);
        
        vm.prank(buyer);
        accessControl.terminateSession(sessionKey, "Terminated");
        
        (bool isValid, uint256 timeRemaining) = accessControl.validateAccess(sessionKey, buyer);
        
        assertFalse(isValid);
        assertEq(timeRemaining, 0);
    }

    function testValidateAccessForExpiredSession() public {
        bytes32 sessionKey = accessControl.createAccessSession(1, buyer);
        
        // Wait for session to expire
        vm.warp(block.timestamp + MIN_DURATION + 1);
        
        (bool isValid, uint256 timeRemaining) = accessControl.validateAccess(sessionKey, buyer);
        
        assertFalse(isValid);
        assertEq(timeRemaining, 0);
    }

    function testGetUserActiveSessions() public {
        bytes32 sessionKey1 = accessControl.createAccessSession(1, buyer);
        
        // Purchase additional access for second session
        vm.prank(buyer);
        marketplace.purchaseDataAccess{value: PRICE_PER_HOUR}(1, MIN_DURATION);
        
        bytes32 sessionKey2 = accessControl.createAccessSession(1, buyer);
        
        bytes32[] memory activeSessions = accessControl.getUserActiveSessions(buyer);
        
        assertEq(activeSessions.length, 2);
        assertTrue(activeSessions[0] == sessionKey1 || activeSessions[0] == sessionKey2);
        assertTrue(activeSessions[1] == sessionKey1 || activeSessions[1] == sessionKey2);
        assertTrue(activeSessions[0] != activeSessions[1]);
    }

    function testMaxConcurrentSessions() public {
        // Create maximum allowed sessions
        for (uint256 i = 0; i < 3; i++) {
            vm.prank(buyer);
            marketplace.purchaseDataAccess{value: PRICE_PER_HOUR}(1, MIN_DURATION);
            accessControl.createAccessSession(1, buyer);
        }
        
        // Try to create one more session
        vm.prank(buyer);
        marketplace.purchaseDataAccess{value: PRICE_PER_HOUR}(1, MIN_DURATION);
        
        vm.expectRevert(AccessControl.TooManyConcurrentSessions.selector);
        accessControl.createAccessSession(1, buyer);
    }

    function testSetMaxConcurrentSessions() public {
        uint256 newLimit = 5;
        accessControl.setMaxConcurrentSessions(newLimit);
        assertEq(accessControl.maxConcurrentSessions(), newLimit);
    }

    function testCleanupExpiredSessions() public {
        bytes32 sessionKey = accessControl.createAccessSession(1, buyer);
        
        // Fast forward past expiration + buffer
        vm.warp(block.timestamp + MIN_DURATION + accessControl.sessionTimeoutBuffer() + 1);
        
        bytes32[] memory sessions = new bytes32[](1);
        sessions[0] = sessionKey;
        
        accessControl.cleanupExpiredSessions(sessions);
        
        AccessControl.AccessSession memory session = accessControl.getAccessSession(sessionKey);
        assertFalse(session.isActive);
    }

    function testGetSessionRequests() public {
        bytes32 sessionKey = accessControl.createAccessSession(1, buyer);
        
        vm.prank(buyer);
        accessControl.requestDataAccess(sessionKey, "location");
        
        vm.prank(buyer);
        accessControl.requestDataAccess(sessionKey, "speed");
        
        AccessControl.DataRequest[] memory requests = accessControl.getSessionRequests(sessionKey);
        
        assertEq(requests.length, 2);
        assertEq(requests[0].requestType, "location");
        assertEq(requests[1].requestType, "speed");
        assertTrue(requests[0].wasAuthorized);
        assertTrue(requests[1].wasAuthorized);
    }

    function testSetSessionTimeoutBuffer() public {
        uint256 newBuffer = 600; // 10 minutes
        accessControl.setSessionTimeoutBuffer(newBuffer);
        assertEq(accessControl.sessionTimeoutBuffer(), newBuffer);
    }
}