// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@openzeppelin/contracts/utils/ReentrancyGuard.sol";
import "@openzeppelin/contracts/utils/Pausable.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "./VehicleRegistry.sol";

contract DataMarketplace is ReentrancyGuard, Pausable, Ownable {
    VehicleRegistry public immutable vehicleRegistry;

    struct DataProduct {
        uint256 vehicleId;
        string dataType; // GPS, diagnostics, fuel, etc.
        uint256 pricePerHour; // Price in wei per hour of access
        uint256 minDuration; // Minimum access duration in seconds
        uint256 maxDuration; // Maximum access duration in seconds
        bool isActive;
        string description;
        string apiEndpoint; // Off-chain API endpoint for data access
        uint256 createdAt;
    }

    struct Purchase {
        uint256 productId;
        address buyer;
        uint256 startTime;
        uint256 endTime;
        uint256 amountPaid;
        bool isActive;
        string accessToken; // Off-chain access token
    }

    struct StreamingPayment {
        uint256 productId;
        address buyer;
        uint256 ratePerSecond; // Payment rate in wei per second
        uint256 balance; // Prepaid balance
        uint256 lastWithdrawal;
        uint256 startTime;
        bool isActive;
    }

    mapping(uint256 => DataProduct) public dataProducts;
    mapping(uint256 => Purchase[]) public productPurchases;
    mapping(address => Purchase[]) public buyerPurchases;
    mapping(uint256 => StreamingPayment) public streamingPayments;
    
    uint256 public productCounter;
    uint256 public streamingCounter;
    uint256 public platformFeeRate = 250; // 2.5% (basis points)
    uint256 public constant MAX_PLATFORM_FEE = 1000; // 10% max
    
    event DataProductListed(
        uint256 indexed productId,
        uint256 indexed vehicleId,
        string dataType,
        uint256 pricePerHour
    );
    
    event DataProductUpdated(uint256 indexed productId, uint256 newPrice, bool isActive);
    
    event DataPurchased(
        uint256 indexed productId,
        address indexed buyer,
        uint256 duration,
        uint256 amountPaid,
        uint256 purchaseIndex
    );
    
    event StreamingPaymentStarted(
        uint256 indexed streamingId,
        uint256 indexed productId,
        address indexed buyer,
        uint256 ratePerSecond
    );
    
    event StreamingPaymentWithdrawn(
        uint256 indexed streamingId,
        uint256 amount,
        address indexed vehicleWallet
    );
    
    event PlatformFeeUpdated(uint256 newFeeRate);

    error ProductNotFound();
    error ProductNotActive();
    error UnauthorizedSeller();
    error InvalidDuration();
    error InvalidPrice();
    error InsufficientPayment();
    error AccessExpired();
    error StreamingNotActive();
    error InvalidFeeRate();

    modifier onlyVehicleOwner(uint256 vehicleId) {
        (, , , , , , , address vehicleOwner) = vehicleRegistry.vehicles(vehicleId);
        if (vehicleOwner != msg.sender) {
            revert UnauthorizedSeller();
        }
        _;
    }

    modifier validProduct(uint256 productId) {
        if (productId == 0 || productId > productCounter) {
            revert ProductNotFound();
        }
        _;
    }

    constructor(address _vehicleRegistry) Ownable(msg.sender) {
        vehicleRegistry = VehicleRegistry(_vehicleRegistry);
    }

    function listDataProduct(
        uint256 vehicleId,
        string memory dataType,
        uint256 pricePerHour,
        uint256 minDuration,
        uint256 maxDuration,
        string memory description,
        string memory apiEndpoint
    ) external onlyVehicleOwner(vehicleId) whenNotPaused {
        if (pricePerHour == 0) {
            revert InvalidPrice();
        }
        
        if (minDuration == 0 || maxDuration == 0 || minDuration > maxDuration) {
            revert InvalidDuration();
        }

        if (!vehicleRegistry.isVehicleActive(vehicleId)) {
            revert ProductNotActive();
        }

        productCounter++;
        uint256 productId = productCounter;

        dataProducts[productId] = DataProduct({
            vehicleId: vehicleId,
            dataType: dataType,
            pricePerHour: pricePerHour,
            minDuration: minDuration,
            maxDuration: maxDuration,
            isActive: true,
            description: description,
            apiEndpoint: apiEndpoint,
            createdAt: block.timestamp
        });

        emit DataProductListed(productId, vehicleId, dataType, pricePerHour);
    }

    function updateDataProduct(
        uint256 productId,
        uint256 newPricePerHour,
        bool isActive,
        string memory newDescription,
        string memory newApiEndpoint
    ) external validProduct(productId) whenNotPaused {
        DataProduct storage product = dataProducts[productId];
        
        uint256 vehicleId = product.vehicleId;
        (, , , , , , , address vehicleOwner) = vehicleRegistry.vehicles(vehicleId);
        if (vehicleOwner != msg.sender) {
            revert UnauthorizedSeller();
        }

        if (newPricePerHour > 0) {
            product.pricePerHour = newPricePerHour;
        }
        
        product.isActive = isActive;
        product.description = newDescription;
        product.apiEndpoint = newApiEndpoint;

        emit DataProductUpdated(productId, newPricePerHour, isActive);
    }

    function purchaseDataAccess(
        uint256 productId,
        uint256 durationInSeconds
    ) external payable validProduct(productId) nonReentrant whenNotPaused {
        DataProduct memory product = dataProducts[productId];
        
        if (!product.isActive) {
            revert ProductNotActive();
        }
        
        if (durationInSeconds < product.minDuration || durationInSeconds > product.maxDuration) {
            revert InvalidDuration();
        }

        uint256 durationInHours = (durationInSeconds + 3599) / 3600; // Round up to nearest hour
        uint256 totalCost = product.pricePerHour * durationInHours;
        
        if (msg.value < totalCost) {
            revert InsufficientPayment();
        }

        uint256 platformFee = (totalCost * platformFeeRate) / 10000;
        uint256 vehiclePayment = totalCost - platformFee;

        (string memory vin, address vehicleWallet, , , , , , ) = vehicleRegistry.vehicles(product.vehicleId);
        
        (bool success, ) = vehicleWallet.call{value: vehiclePayment}("");
        require(success, "Payment to vehicle failed");

        uint256 purchaseIndex = productPurchases[productId].length;
        
        Purchase memory newPurchase = Purchase({
            productId: productId,
            buyer: msg.sender,
            startTime: block.timestamp,
            endTime: block.timestamp + durationInSeconds,
            amountPaid: totalCost,
            isActive: true,
            accessToken: "" // To be set off-chain
        });

        productPurchases[productId].push(newPurchase);
        buyerPurchases[msg.sender].push(newPurchase);

        if (msg.value > totalCost) {
            (bool refundSuccess, ) = payable(msg.sender).call{value: msg.value - totalCost}("");
            require(refundSuccess, "Refund failed");
        }

        emit DataPurchased(productId, msg.sender, durationInSeconds, totalCost, purchaseIndex);
    }

    function startStreamingPayment(
        uint256 productId,
        uint256 ratePerSecond
    ) external payable validProduct(productId) nonReentrant whenNotPaused {
        DataProduct memory product = dataProducts[productId];
        
        if (!product.isActive) {
            revert ProductNotActive();
        }

        if (msg.value == 0) {
            revert InsufficientPayment();
        }

        streamingCounter++;
        uint256 streamingId = streamingCounter;

        streamingPayments[streamingId] = StreamingPayment({
            productId: productId,
            buyer: msg.sender,
            ratePerSecond: ratePerSecond,
            balance: msg.value,
            lastWithdrawal: block.timestamp,
            startTime: block.timestamp,
            isActive: true
        });

        emit StreamingPaymentStarted(streamingId, productId, msg.sender, ratePerSecond);
    }

    function withdrawStreamingPayment(uint256 streamingId) 
        external 
        nonReentrant 
        whenNotPaused 
    {
        StreamingPayment storage streaming = streamingPayments[streamingId];
        
        if (!streaming.isActive) {
            revert StreamingNotActive();
        }

        DataProduct memory product = dataProducts[streaming.productId];
        (, address vehicleWallet, , , , , , address vehicleOwner) = vehicleRegistry.vehicles(product.vehicleId);
        
        if (msg.sender != vehicleOwner) {
            revert UnauthorizedSeller();
        }

        uint256 timeElapsed = block.timestamp - streaming.lastWithdrawal;
        uint256 withdrawAmount = timeElapsed * streaming.ratePerSecond;
        
        if (withdrawAmount > streaming.balance) {
            withdrawAmount = streaming.balance;
            streaming.isActive = false;
        }

        streaming.balance -= withdrawAmount;
        streaming.lastWithdrawal = block.timestamp;

        uint256 platformFee = (withdrawAmount * platformFeeRate) / 10000;
        uint256 vehiclePayment = withdrawAmount - platformFee;

        (bool success, ) = vehicleWallet.call{value: vehiclePayment}("");
        require(success, "Streaming payment failed");

        emit StreamingPaymentWithdrawn(streamingId, withdrawAmount, vehicleWallet);
    }

    function hasValidAccess(address buyer, uint256 productId) 
        external 
        view 
        returns (bool, uint256) 
    {
        Purchase[] memory purchases = buyerPurchases[buyer];
        uint256 latestEndTime = 0;
        bool hasAccess = false;
        
        for (uint256 i = 0; i < purchases.length; i++) {
            if (purchases[i].productId == productId && 
                purchases[i].isActive && 
                block.timestamp <= purchases[i].endTime) {
                hasAccess = true;
                if (purchases[i].endTime > latestEndTime) {
                    latestEndTime = purchases[i].endTime;
                }
            }
        }
        
        return (hasAccess, latestEndTime);
    }

    function getDataProduct(uint256 productId) 
        external 
        view 
        validProduct(productId) 
        returns (DataProduct memory) 
    {
        return dataProducts[productId];
    }

    function getProductPurchases(uint256 productId) 
        external 
        view 
        returns (Purchase[] memory) 
    {
        return productPurchases[productId];
    }

    function getBuyerPurchases(address buyer) 
        external 
        view 
        returns (Purchase[] memory) 
    {
        return buyerPurchases[buyer];
    }

    function getStreamingPayment(uint256 streamingId) 
        external 
        view 
        returns (StreamingPayment memory) 
    {
        return streamingPayments[streamingId];
    }

    function setPlatformFeeRate(uint256 newFeeRate) external onlyOwner {
        if (newFeeRate > MAX_PLATFORM_FEE) {
            revert InvalidFeeRate();
        }
        
        platformFeeRate = newFeeRate;
        emit PlatformFeeUpdated(newFeeRate);
    }

    function pause() external onlyOwner {
        _pause();
    }

    function unpause() external onlyOwner {
        _unpause();
    }

    function withdrawPlatformFees() external onlyOwner {
        (bool success, ) = payable(owner()).call{value: address(this).balance}("");
        require(success, "Platform fee withdrawal failed");
    }

    function getTotalProducts() external view returns (uint256) {
        return productCounter;
    }

    function getTotalStreamingPayments() external view returns (uint256) {
        return streamingCounter;
    }
}