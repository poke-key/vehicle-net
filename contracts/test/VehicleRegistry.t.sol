// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "forge-std/Test.sol";
import "../src/VehicleRegistry.sol";

contract VehicleRegistryTest is Test {
    VehicleRegistry public registry;
    
    receive() external payable {}
    
    address public owner = address(this);
    address public vehicleOwner1 = address(0x1);
    address public vehicleOwner2 = address(0x2);
    address public vehicleWallet1 = address(0x101);
    address public vehicleWallet2 = address(0x102);
    
    string constant VIN1 = "1HGCM82633A004352";
    string constant VIN2 = "WBAPK5G59BNB29457";
    string constant MANUFACTURER = "Toyota";
    string constant MODEL = "Camry";
    uint256 constant YEAR = 2023;
    
    string[] dataTypes;
    string constant IPFS_HASH = "QmTestHash123";
    
    event VehicleRegistered(
        uint256 indexed vehicleId,
        string vin,
        address indexed wallet,
        address indexed owner
    );

    function setUp() public {
        registry = new VehicleRegistry();
        
        dataTypes.push("GPS");
        dataTypes.push("Diagnostics");
        dataTypes.push("Fuel");
        
        vm.deal(vehicleOwner1, 1 ether);
        vm.deal(vehicleOwner2, 1 ether);
    }

    function testRegisterVehicle() public {
        vm.prank(vehicleOwner1);
        vm.expectEmit(true, true, true, false);
        emit VehicleRegistered(1, VIN1, vehicleWallet1, vehicleOwner1);
        
        registry.registerVehicle{value: 0.01 ether}(
            VIN1,
            vehicleWallet1,
            MANUFACTURER,
            MODEL,
            YEAR,
            dataTypes,
            IPFS_HASH
        );
        
        VehicleRegistry.Vehicle memory vehicle = registry.getVehicle(1);
        
        assertEq(vehicle.vin, VIN1);
        assertEq(vehicle.wallet, vehicleWallet1);
        assertEq(vehicle.manufacturer, MANUFACTURER);
        assertEq(vehicle.model, MODEL);
        assertEq(vehicle.year, YEAR);
        assertEq(vehicle.owner, vehicleOwner1);
        assertTrue(vehicle.isActive);
        
        assertEq(registry.getVehicleIdByVin(VIN1), 1);
        assertEq(registry.getVehicleIdByWallet(vehicleWallet1), 1);
        assertTrue(registry.isVehicleActive(1));
    }

    function testCannotRegisterVehicleWithInsufficientFee() public {
        vm.prank(vehicleOwner1);
        vm.expectRevert(VehicleRegistry.InsufficientFee.selector);
        
        registry.registerVehicle{value: 0.005 ether}(
            VIN1,
            vehicleWallet1,
            MANUFACTURER,
            MODEL,
            YEAR,
            dataTypes,
            IPFS_HASH
        );
    }

    function testCannotRegisterVehicleWithEmptyVin() public {
        vm.prank(vehicleOwner1);
        vm.expectRevert(VehicleRegistry.InvalidVehicleData.selector);
        
        registry.registerVehicle{value: 0.01 ether}(
            "",
            vehicleWallet1,
            MANUFACTURER,
            MODEL,
            YEAR,
            dataTypes,
            IPFS_HASH
        );
    }

    function testCannotRegisterVehicleWithZeroWallet() public {
        vm.prank(vehicleOwner1);
        vm.expectRevert(VehicleRegistry.InvalidVehicleData.selector);
        
        registry.registerVehicle{value: 0.01 ether}(
            VIN1,
            address(0),
            MANUFACTURER,
            MODEL,
            YEAR,
            dataTypes,
            IPFS_HASH
        );
    }

    function testCannotRegisterDuplicateVin() public {
        vm.prank(vehicleOwner1);
        registry.registerVehicle{value: 0.01 ether}(
            VIN1,
            vehicleWallet1,
            MANUFACTURER,
            MODEL,
            YEAR,
            dataTypes,
            IPFS_HASH
        );
        
        vm.prank(vehicleOwner2);
        vm.expectRevert(VehicleRegistry.VehicleAlreadyExists.selector);
        
        registry.registerVehicle{value: 0.01 ether}(
            VIN1,
            vehicleWallet2,
            MANUFACTURER,
            MODEL,
            YEAR,
            dataTypes,
            IPFS_HASH
        );
    }

    function testCannotRegisterDuplicateWallet() public {
        vm.prank(vehicleOwner1);
        registry.registerVehicle{value: 0.01 ether}(
            VIN1,
            vehicleWallet1,
            MANUFACTURER,
            MODEL,
            YEAR,
            dataTypes,
            IPFS_HASH
        );
        
        vm.prank(vehicleOwner2);
        vm.expectRevert(VehicleRegistry.VehicleAlreadyExists.selector);
        
        registry.registerVehicle{value: 0.01 ether}(
            VIN2,
            vehicleWallet1,
            MANUFACTURER,
            MODEL,
            YEAR,
            dataTypes,
            IPFS_HASH
        );
    }

    function testUpdateVehicleMetadata() public {
        vm.prank(vehicleOwner1);
        registry.registerVehicle{value: 0.01 ether}(
            VIN1,
            vehicleWallet1,
            MANUFACTURER,
            MODEL,
            YEAR,
            dataTypes,
            IPFS_HASH
        );
        
        string[] memory newDataTypes = new string[](2);
        newDataTypes[0] = "GPS";
        newDataTypes[1] = "Temperature";
        string memory newIpfsHash = "QmNewHash456";
        
        vm.prank(vehicleOwner1);
        registry.updateVehicleMetadata(1, newDataTypes, newIpfsHash);
        
        VehicleRegistry.VehicleMetadata memory metadata = registry.getVehicleMetadata(1);
        assertEq(metadata.dataTypes.length, 2);
        assertEq(metadata.dataTypes[0], "GPS");
        assertEq(metadata.dataTypes[1], "Temperature");
        assertEq(metadata.ipfsHash, newIpfsHash);
    }

    function testCannotUpdateMetadataAsNonOwner() public {
        vm.prank(vehicleOwner1);
        registry.registerVehicle{value: 0.01 ether}(
            VIN1,
            vehicleWallet1,
            MANUFACTURER,
            MODEL,
            YEAR,
            dataTypes,
            IPFS_HASH
        );
        
        string[] memory newDataTypes = new string[](1);
        newDataTypes[0] = "GPS";
        
        vm.prank(vehicleOwner2);
        vm.expectRevert(VehicleRegistry.UnauthorizedAccess.selector);
        registry.updateVehicleMetadata(1, newDataTypes, "NewHash");
    }

    function testDeactivateVehicle() public {
        vm.prank(vehicleOwner1);
        registry.registerVehicle{value: 0.01 ether}(
            VIN1,
            vehicleWallet1,
            MANUFACTURER,
            MODEL,
            YEAR,
            dataTypes,
            IPFS_HASH
        );
        
        assertTrue(registry.isVehicleActive(1));
        
        vm.prank(vehicleOwner1);
        registry.deactivateVehicle(1);
        
        assertFalse(registry.isVehicleActive(1));
    }

    function testCannotDeactivateAsNonOwner() public {
        vm.prank(vehicleOwner1);
        registry.registerVehicle{value: 0.01 ether}(
            VIN1,
            vehicleWallet1,
            MANUFACTURER,
            MODEL,
            YEAR,
            dataTypes,
            IPFS_HASH
        );
        
        vm.prank(vehicleOwner2);
        vm.expectRevert(VehicleRegistry.UnauthorizedAccess.selector);
        registry.deactivateVehicle(1);
    }

    function testSetRegistrationFee() public {
        uint256 newFee = 0.02 ether;
        registry.setRegistrationFee(newFee);
        assertEq(registry.registrationFee(), newFee);
    }

    function testWithdrawFees() public {
        vm.prank(vehicleOwner1);
        registry.registerVehicle{value: 0.01 ether}(
            VIN1,
            vehicleWallet1,
            MANUFACTURER,
            MODEL,
            YEAR,
            dataTypes,
            IPFS_HASH
        );
        
        uint256 initialBalance = owner.balance;
        registry.withdrawFees();
        assertEq(owner.balance, initialBalance + 0.01 ether);
    }

    function testGetTotalVehicles() public {
        assertEq(registry.getTotalVehicles(), 0);
        
        vm.prank(vehicleOwner1);
        registry.registerVehicle{value: 0.01 ether}(
            VIN1,
            vehicleWallet1,
            MANUFACTURER,
            MODEL,
            YEAR,
            dataTypes,
            IPFS_HASH
        );
        
        assertEq(registry.getTotalVehicles(), 1);
    }

    function testCannotGetNonExistentVehicle() public {
        vm.expectRevert(VehicleRegistry.VehicleNotFound.selector);
        registry.getVehicle(1);
    }
}