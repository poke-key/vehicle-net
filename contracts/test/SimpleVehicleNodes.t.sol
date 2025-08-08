// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "forge-std/Test.sol";
import "../src/SimpleVehicleNodes.sol";

contract SimpleVehicleNodesTest is Test {
    SimpleVehicleNodes public vehicleNodes;
    address public vehicle1 = address(0x1);
    address public vehicle2 = address(0x2);

    function setUp() public {
        vehicleNodes = new SimpleVehicleNodes();
    }

    function testAddVehicleNode() public {
        string memory vin = "1HGBH41JXMN109186";
        
        vehicleNodes.addVehicleNode(vin, vehicle1);
        
        SimpleVehicleNodes.VehicleNode memory vehicle = vehicleNodes.getVehicle(1);
        assertEq(vehicle.vin, vin);
        assertEq(vehicle.nodeAddress, vehicle1);
        assertTrue(vehicle.isActive);
        assertEq(vehicleNodes.getTotalVehicles(), 1);
    }

    function testUpdateVehicleReport() public {
        string memory vin = "1HGBH41JXMN109186";
        vehicleNodes.addVehicleNode(vin, vehicle1);

        vm.prank(vehicle1);
        vehicleNodes.updateVehicleReport(15000, 85);

        SimpleVehicleNodes.VehicleNode memory vehicle = vehicleNodes.getVehicle(1);
        assertEq(vehicle.mileage, 15000);
        assertEq(vehicle.batteryHealth, 85);
    }

    function testGetAllActiveVehicles() public {
        vehicleNodes.addVehicleNode("VIN1", vehicle1);
        vehicleNodes.addVehicleNode("VIN2", vehicle2);

        SimpleVehicleNodes.VehicleNode[] memory activeVehicles = vehicleNodes.getAllActiveVehicles();
        assertEq(activeVehicles.length, 2);
    }
}