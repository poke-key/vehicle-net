// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "forge-std/Test.sol";
import "../src/VehicleRegistry.sol";
import "../src/DataMarketplace.sol";

contract DataMarketplaceTest is Test {
    VehicleRegistry public registry;
    DataMarketplace public marketplace;
    
    receive() external payable {}
    
    address public owner = address(this);
    address public vehicleOwner = address(0x1);
    address public buyer1 = address(0x2);
    address public buyer2 = address(0x3);
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
        
        dataTypes.push("GPS");
        dataTypes.push("Diagnostics");
        
        vm.deal(vehicleOwner, 1 ether);
        vm.deal(buyer1, 1 ether);
        vm.deal(buyer2, 1 ether);
        
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
    }

    function testListDataProduct() public {
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
        
        DataMarketplace.DataProduct memory product = marketplace.getDataProduct(1);
        
        assertEq(product.vehicleId, 1);
        assertEq(product.dataType, DATA_TYPE);
        assertEq(product.pricePerHour, PRICE_PER_HOUR);
        assertEq(product.minDuration, MIN_DURATION);
        assertEq(product.maxDuration, MAX_DURATION);
        assertEq(product.description, DESCRIPTION);
        assertEq(product.apiEndpoint, API_ENDPOINT);
        assertTrue(product.isActive);
    }

    function testCannotListProductAsNonOwner() public {
        vm.prank(buyer1);
        vm.expectRevert(DataMarketplace.UnauthorizedSeller.selector);
        
        marketplace.listDataProduct(
            1,
            DATA_TYPE,
            PRICE_PER_HOUR,
            MIN_DURATION,
            MAX_DURATION,
            DESCRIPTION,
            API_ENDPOINT
        );
    }

    function testCannotListProductWithZeroPrice() public {
        vm.prank(vehicleOwner);
        vm.expectRevert(DataMarketplace.InvalidPrice.selector);
        
        marketplace.listDataProduct(
            1,
            DATA_TYPE,
            0,
            MIN_DURATION,
            MAX_DURATION,
            DESCRIPTION,
            API_ENDPOINT
        );
    }

    function testCannotListProductWithInvalidDuration() public {
        vm.prank(vehicleOwner);
        vm.expectRevert(DataMarketplace.InvalidDuration.selector);
        
        marketplace.listDataProduct(
            1,
            DATA_TYPE,
            PRICE_PER_HOUR,
            MAX_DURATION,
            MIN_DURATION, // min > max
            DESCRIPTION,
            API_ENDPOINT
        );
    }

    function testUpdateDataProduct() public {
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
        
        uint256 newPrice = 0.002 ether;
        string memory newDescription = "Updated GPS tracking";
        string memory newEndpoint = "https://api.example.com/gps-v2";
        
        vm.prank(vehicleOwner);
        marketplace.updateDataProduct(1, newPrice, true, newDescription, newEndpoint);
        
        DataMarketplace.DataProduct memory product = marketplace.getDataProduct(1);
        assertEq(product.pricePerHour, newPrice);
        assertEq(product.description, newDescription);
        assertEq(product.apiEndpoint, newEndpoint);
        assertTrue(product.isActive);
    }

    function testPurchaseDataAccess() public {
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
        
        uint256 duration = 7200; // 2 hours
        uint256 expectedCost = PRICE_PER_HOUR * 2; // Rounded up to 2 hours
        uint256 platformFee = (expectedCost * marketplace.platformFeeRate()) / 10000;
        uint256 vehiclePayment = expectedCost - platformFee;
        
        uint256 initialVehicleBalance = vehicleWallet.balance;
        
        vm.prank(buyer1);
        marketplace.purchaseDataAccess{value: expectedCost}(1, duration);
        
        assertEq(vehicleWallet.balance, initialVehicleBalance + vehiclePayment);
        
        (bool hasAccess, uint256 endTime) = marketplace.hasValidAccess(buyer1, 1);
        assertTrue(hasAccess);
        assertGt(endTime, block.timestamp);
    }

    function testCannotPurchaseInactiveProduct() public {
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
        
        vm.prank(vehicleOwner);
        marketplace.updateDataProduct(1, 0, false, "", "");
        
        vm.prank(buyer1);
        vm.expectRevert(DataMarketplace.ProductNotActive.selector);
        marketplace.purchaseDataAccess{value: PRICE_PER_HOUR}(1, MIN_DURATION);
    }

    function testCannotPurchaseWithInvalidDuration() public {
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
        
        vm.prank(buyer1);
        vm.expectRevert(DataMarketplace.InvalidDuration.selector);
        marketplace.purchaseDataAccess{value: PRICE_PER_HOUR}(1, MIN_DURATION - 1);
    }

    function testCannotPurchaseWithInsufficientPayment() public {
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
        
        vm.prank(buyer1);
        vm.expectRevert(DataMarketplace.InsufficientPayment.selector);
        marketplace.purchaseDataAccess{value: PRICE_PER_HOUR - 1}(1, MIN_DURATION);
    }

    function testStartStreamingPayment() public {
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
        
        uint256 ratePerSecond = 1e12; // 0.000001 ether per second
        uint256 prepaidAmount = 0.1 ether;
        
        vm.prank(buyer1);
        marketplace.startStreamingPayment{value: prepaidAmount}(1, ratePerSecond);
        
        DataMarketplace.StreamingPayment memory streaming = marketplace.getStreamingPayment(1);
        
        assertEq(streaming.productId, 1);
        assertEq(streaming.buyer, buyer1);
        assertEq(streaming.ratePerSecond, ratePerSecond);
        assertEq(streaming.balance, prepaidAmount);
        assertTrue(streaming.isActive);
    }

    function testWithdrawStreamingPayment() public {
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
        
        uint256 ratePerSecond = 1e12; // 0.000001 ether per second
        uint256 prepaidAmount = 0.1 ether;
        
        vm.prank(buyer1);
        marketplace.startStreamingPayment{value: prepaidAmount}(1, ratePerSecond);
        
        // Fast forward 1000 seconds
        vm.warp(block.timestamp + 1000);
        
        uint256 initialBalance = vehicleWallet.balance;
        
        vm.prank(vehicleOwner);
        marketplace.withdrawStreamingPayment(1);
        
        uint256 expectedWithdraw = 1000 * ratePerSecond;
        uint256 platformFee = (expectedWithdraw * marketplace.platformFeeRate()) / 10000;
        uint256 vehiclePayment = expectedWithdraw - platformFee;
        
        assertEq(vehicleWallet.balance, initialBalance + vehiclePayment);
    }

    function testCannotWithdrawStreamingAsNonOwner() public {
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
        
        uint256 ratePerSecond = 1e12;
        uint256 prepaidAmount = 0.1 ether;
        
        vm.prank(buyer1);
        marketplace.startStreamingPayment{value: prepaidAmount}(1, ratePerSecond);
        
        vm.prank(buyer2);
        vm.expectRevert(DataMarketplace.UnauthorizedSeller.selector);
        marketplace.withdrawStreamingPayment(1);
    }

    function testSetPlatformFeeRate() public {
        uint256 newFeeRate = 500; // 5%
        marketplace.setPlatformFeeRate(newFeeRate);
        assertEq(marketplace.platformFeeRate(), newFeeRate);
    }

    function testCannotSetExcessivePlatformFee() public {
        uint256 excessiveFee = 1500; // 15%, above 10% max
        vm.expectRevert(DataMarketplace.InvalidFeeRate.selector);
        marketplace.setPlatformFeeRate(excessiveFee);
    }

    function testPauseAndUnpause() public {
        marketplace.pause();
        
        vm.prank(vehicleOwner);
        vm.expectRevert();
        marketplace.listDataProduct(
            1,
            DATA_TYPE,
            PRICE_PER_HOUR,
            MIN_DURATION,
            MAX_DURATION,
            DESCRIPTION,
            API_ENDPOINT
        );
        
        marketplace.unpause();
        
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
    }

    function testWithdrawPlatformFees() public {
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
        
        uint256 duration = MIN_DURATION;
        uint256 cost = PRICE_PER_HOUR;
        
        vm.prank(buyer1);
        marketplace.purchaseDataAccess{value: cost}(1, duration);
        
        uint256 initialBalance = owner.balance;
        marketplace.withdrawPlatformFees();
        
        uint256 platformFee = (cost * marketplace.platformFeeRate()) / 10000;
        assertEq(owner.balance, initialBalance + platformFee);
    }

    function testGetTotalProducts() public {
        assertEq(marketplace.getTotalProducts(), 0);
        
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
        
        assertEq(marketplace.getTotalProducts(), 1);
    }
}