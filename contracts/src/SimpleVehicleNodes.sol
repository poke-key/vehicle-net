// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

contract SimpleVehicleNodes {
    struct VehicleNode {
        string vin;
        address nodeAddress;
        uint256 mileage;
        uint256 batteryHealth;
        uint256 lastReportTimestamp;
        bool isActive;
    }

    mapping(uint256 => VehicleNode) public vehicles;
    mapping(address => uint256) public addressToVehicleId;
    mapping(string => uint256) public vinToVehicleId;
    
    uint256 public vehicleCount;

    event VehicleNodeAdded(uint256 indexed vehicleId, string vin, address indexed nodeAddress);
    event VehicleReportUpdated(uint256 indexed vehicleId, uint256 mileage, uint256 batteryHealth);

    function addVehicleNode(string memory vin, address nodeAddress) external {
        require(bytes(vin).length > 0, "Invalid VIN");
        require(nodeAddress != address(0), "Invalid address");
        require(vinToVehicleId[vin] == 0, "VIN already exists");
        require(addressToVehicleId[nodeAddress] == 0, "Address already registered");

        vehicleCount++;
        uint256 vehicleId = vehicleCount;

        vehicles[vehicleId] = VehicleNode({
            vin: vin,
            nodeAddress: nodeAddress,
            mileage: 0,
            batteryHealth: 100,
            lastReportTimestamp: block.timestamp,
            isActive: true
        });

        addressToVehicleId[nodeAddress] = vehicleId;
        vinToVehicleId[vin] = vehicleId;

        emit VehicleNodeAdded(vehicleId, vin, nodeAddress);
    }

    function updateVehicleReport(uint256 mileage, uint256 batteryHealth) external {
        uint256 vehicleId = addressToVehicleId[msg.sender];
        require(vehicleId > 0, "Vehicle not found");
        require(vehicles[vehicleId].isActive, "Vehicle not active");

        vehicles[vehicleId].mileage = mileage;
        vehicles[vehicleId].batteryHealth = batteryHealth;
        vehicles[vehicleId].lastReportTimestamp = block.timestamp;

        emit VehicleReportUpdated(vehicleId, mileage, batteryHealth);
    }

    function getVehicle(uint256 vehicleId) external view returns (VehicleNode memory) {
        require(vehicleId > 0 && vehicleId <= vehicleCount, "Invalid vehicle ID");
        return vehicles[vehicleId];
    }

    function getVehicleByAddress(address nodeAddress) external view returns (VehicleNode memory) {
        uint256 vehicleId = addressToVehicleId[nodeAddress];
        require(vehicleId > 0, "Vehicle not found");
        return vehicles[vehicleId];
    }

    function getAllActiveVehicles() external view returns (VehicleNode[] memory) {
        uint256 activeCount = 0;
        for (uint256 i = 1; i <= vehicleCount; i++) {
            if (vehicles[i].isActive) {
                activeCount++;
            }
        }

        VehicleNode[] memory activeVehicles = new VehicleNode[](activeCount);
        uint256 currentIndex = 0;
        for (uint256 i = 1; i <= vehicleCount; i++) {
            if (vehicles[i].isActive) {
                activeVehicles[currentIndex] = vehicles[i];
                currentIndex++;
            }
        }

        return activeVehicles;
    }

    function getTotalVehicles() external view returns (uint256) {
        return vehicleCount;
    }
}