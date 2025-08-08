// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/ReentrancyGuard.sol";

contract VehicleRegistry is Ownable, ReentrancyGuard {
    struct Vehicle {
        string vin; // Vehicle Identification Number
        address wallet; // Dedicated wallet for this vehicle
        string manufacturer;
        string model;
        uint256 year;
        bool isActive;
        uint256 registrationTimestamp;
        address owner; // Vehicle owner (different from wallet)
    }

    struct VehicleMetadata {
        string[] dataTypes; // GPS, diagnostics, fuel, etc.
        string ipfsHash; // IPFS hash for additional metadata
        uint256 lastUpdate;
    }

    struct ConditionReport {
        string vin;
        uint256 mileage;
        uint8 batteryHealth;
        uint256 timestamp;
        bytes signature;
        bool verified;
    }

    mapping(uint256 => Vehicle) public vehicles;
    mapping(address => uint256) public walletToVehicleId;
    mapping(string => uint256) public vinToVehicleId;
    mapping(uint256 => VehicleMetadata) public vehicleMetadata;
    mapping(uint256 => ConditionReport[]) public vehicleReports;
    mapping(uint256 => uint256) public lastReportTimestamp;
    
    uint256 public vehicleCounter;
    uint256 public registrationFee = 0.01 ether;

    event VehicleRegistered(
        uint256 indexed vehicleId,
        string vin,
        address indexed wallet,
        address indexed owner
    );
    
    event VehicleUpdated(uint256 indexed vehicleId, string ipfsHash);
    event VehicleDeactivated(uint256 indexed vehicleId);
    event RegistrationFeeUpdated(uint256 newFee);
    event ConditionReportSubmitted(
        uint256 indexed vehicleId,
        uint256 mileage,
        uint8 batteryHealth,
        uint256 timestamp,
        bool verified
    );

    error VehicleNotFound();
    error VehicleAlreadyExists();
    error UnauthorizedAccess();
    error InvalidVehicleData();
    error InsufficientFee();

    modifier onlyVehicleOwner(uint256 vehicleId) {
        if (vehicles[vehicleId].owner != msg.sender) {
            revert UnauthorizedAccess();
        }
        _;
    }

    modifier validVehicleId(uint256 vehicleId) {
        if (vehicleId == 0 || vehicleId > vehicleCounter) {
            revert VehicleNotFound();
        }
        _;
    }

    constructor() Ownable(msg.sender) {}

    function registerVehicle(
        string memory vin,
        address vehicleWallet,
        string memory manufacturer,
        string memory model,
        uint256 year,
        string[] memory dataTypes,
        string memory ipfsHash
    ) external payable nonReentrant {
        if (msg.value < registrationFee) {
            revert InsufficientFee();
        }

        if (bytes(vin).length == 0 || vehicleWallet == address(0)) {
            revert InvalidVehicleData();
        }

        if (vinToVehicleId[vin] != 0) {
            revert VehicleAlreadyExists();
        }

        if (walletToVehicleId[vehicleWallet] != 0) {
            revert VehicleAlreadyExists();
        }

        vehicleCounter++;
        uint256 vehicleId = vehicleCounter;

        vehicles[vehicleId] = Vehicle({
            vin: vin,
            wallet: vehicleWallet,
            manufacturer: manufacturer,
            model: model,
            year: year,
            isActive: true,
            registrationTimestamp: block.timestamp,
            owner: msg.sender
        });

        vehicleMetadata[vehicleId] = VehicleMetadata({
            dataTypes: dataTypes,
            ipfsHash: ipfsHash,
            lastUpdate: block.timestamp
        });

        vinToVehicleId[vin] = vehicleId;
        walletToVehicleId[vehicleWallet] = vehicleId;

        emit VehicleRegistered(vehicleId, vin, vehicleWallet, msg.sender);
    }

    function updateVehicleMetadata(
        uint256 vehicleId,
        string[] memory dataTypes,
        string memory ipfsHash
    ) external validVehicleId(vehicleId) onlyVehicleOwner(vehicleId) {
        vehicleMetadata[vehicleId].dataTypes = dataTypes;
        vehicleMetadata[vehicleId].ipfsHash = ipfsHash;
        vehicleMetadata[vehicleId].lastUpdate = block.timestamp;

        emit VehicleUpdated(vehicleId, ipfsHash);
    }

    function deactivateVehicle(uint256 vehicleId) 
        external 
        validVehicleId(vehicleId) 
        onlyVehicleOwner(vehicleId) 
    {
        vehicles[vehicleId].isActive = false;
        emit VehicleDeactivated(vehicleId);
    }

    function getVehicle(uint256 vehicleId) 
        external 
        view 
        validVehicleId(vehicleId) 
        returns (Vehicle memory) 
    {
        return vehicles[vehicleId];
    }

    function getVehicleMetadata(uint256 vehicleId) 
        external 
        view 
        validVehicleId(vehicleId) 
        returns (VehicleMetadata memory) 
    {
        return vehicleMetadata[vehicleId];
    }

    function getVehicleIdByWallet(address wallet) external view returns (uint256) {
        return walletToVehicleId[wallet];
    }

    function getVehicleIdByVin(string memory vin) external view returns (uint256) {
        return vinToVehicleId[vin];
    }

    function isVehicleActive(uint256 vehicleId) external view returns (bool) {
        return vehicleId <= vehicleCounter && vehicles[vehicleId].isActive;
    }

    function setRegistrationFee(uint256 newFee) external onlyOwner {
        registrationFee = newFee;
        emit RegistrationFeeUpdated(newFee);
    }

    function withdrawFees() external onlyOwner {
        (bool success, ) = payable(owner()).call{value: address(this).balance}("");
        require(success, "Withdrawal failed");
    }

    function submitConditionReport(
        uint256 vehicleId,
        string memory vin,
        uint256 mileage,
        uint8 batteryHealth,
        uint256 timestamp,
        bytes memory signature
    ) external validVehicleId(vehicleId) {
        Vehicle storage vehicle = vehicles[vehicleId];
        
        // Only the vehicle's dedicated wallet can submit reports
        require(msg.sender == vehicle.wallet, "Only vehicle wallet can submit reports");
        require(keccak256(abi.encodePacked(vin)) == keccak256(abi.encodePacked(vehicle.vin)), "VIN mismatch");
        
        // Verify signature (simplified for demo - in production use proper ECDSA verification)
        bool verified = signature.length > 0;
        
        ConditionReport memory report = ConditionReport({
            vin: vin,
            mileage: mileage,
            batteryHealth: batteryHealth,
            timestamp: timestamp,
            signature: signature,
            verified: verified
        });
        
        vehicleReports[vehicleId].push(report);
        lastReportTimestamp[vehicleId] = block.timestamp;
        
        emit ConditionReportSubmitted(vehicleId, mileage, batteryHealth, timestamp, verified);
    }
    
    function getLatestConditionReport(uint256 vehicleId) 
        external 
        view 
        validVehicleId(vehicleId) 
        returns (ConditionReport memory) 
    {
        ConditionReport[] storage reports = vehicleReports[vehicleId];
        require(reports.length > 0, "No reports available");
        return reports[reports.length - 1];
    }
    
    function getConditionReports(uint256 vehicleId) 
        external 
        view 
        validVehicleId(vehicleId) 
        returns (ConditionReport[] memory) 
    {
        return vehicleReports[vehicleId];
    }
    
    function getReportCount(uint256 vehicleId) 
        external 
        view 
        validVehicleId(vehicleId) 
        returns (uint256) 
    {
        return vehicleReports[vehicleId].length;
    }

    function getTotalVehicles() external view returns (uint256) {
        return vehicleCounter;
    }
}