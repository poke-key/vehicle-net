# Vehicle Data Marketplace Smart Contracts

A modular blockchain-based vehicle data marketplace where vehicles have dedicated Ethereum wallets and can sell access to their telemetry data through smart contracts.

## 🏗️ Architecture

The system consists of three core smart contracts:

1. **VehicleRegistry.sol** - Vehicle registration and on-chain identity management
2. **DataMarketplace.sol** - Data product listings, purchases, and payments
3. **AccessControl.sol** - Session management and access control

## 📋 Features

### Vehicle Registry
- **Vehicle Registration**: Each vehicle gets a unique on-chain identity linked to a dedicated wallet
- **Metadata Management**: Store vehicle details, data types, and IPFS metadata hashes
- **Access Control**: Only vehicle owners can update their vehicle information
- **Registration Fees**: Configurable registration fees paid to the platform

### Data Marketplace
- **Product Listings**: Vehicles can list data products with pricing and duration limits
- **Purchase Logic**: Buyers pay in ETH for time-limited access to data streams
- **Automated Payouts**: Payments automatically routed to vehicle wallets minus platform fees
- **Streaming Payments**: Support for continuous payment streams based on usage
- **Platform Fees**: Configurable platform fee percentage (max 10%)

### Access Control
- **Session Management**: Create and manage time-limited access sessions
- **Request Tracking**: Monitor data access requests and usage patterns
- **Concurrent Limits**: Configurable maximum concurrent sessions per user
- **Session Extension**: Extend sessions when additional access is purchased

## 🚀 Getting Started

### Prerequisites

- [Foundry](https://book.getfoundry.sh/getting-started/installation)
- Node.js and npm (for additional tooling)

### Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd vehicle-net/contracts
```

2. Install dependencies:
```bash
forge install
```

3. Build contracts:
```bash
forge build
```

4. Run tests:
```bash
forge test
```

### Deployment

1. Set up environment variables:
```bash
export PRIVATE_KEY="0x..."
export RPC_URL="https://..."
```

2. Deploy the system:
```bash
forge script script/DeploySystem.s.sol --rpc-url $RPC_URL --broadcast --verify
```

3. Run the demo (optional):
```bash
export VEHICLE_REGISTRY_ADDRESS="0x..."
export DATA_MARKETPLACE_ADDRESS="0x..."
export ACCESS_CONTROL_ADDRESS="0x..."
export MNEMONIC="your twelve word mnemonic here..."

forge script script/DemoScript.s.sol --rpc-url $RPC_URL --broadcast
```

## 📚 Contract Details

### VehicleRegistry

Main functions:
- `registerVehicle(...)` - Register a new vehicle with metadata
- `updateVehicleMetadata(...)` - Update vehicle data types and IPFS hash
- `deactivateVehicle(uint256)` - Deactivate a vehicle
- `getVehicle(uint256)` - Get vehicle information
- `isVehicleActive(uint256)` - Check if vehicle is active

Events:
- `VehicleRegistered` - Vehicle successfully registered
- `VehicleUpdated` - Vehicle metadata updated
- `VehicleDeactivated` - Vehicle deactivated

### DataMarketplace

Main functions:
- `listDataProduct(...)` - List a new data product
- `updateDataProduct(...)` - Update existing product details
- `purchaseDataAccess(...)` - Purchase time-limited access
- `startStreamingPayment(...)` - Start streaming payment for usage-based access
- `withdrawStreamingPayment(uint256)` - Withdraw accumulated streaming payments
- `hasValidAccess(address, uint256)` - Check if user has valid access

Events:
- `DataProductListed` - New data product listed
- `DataPurchased` - Data access purchased
- `StreamingPaymentStarted` - Streaming payment initiated
- `StreamingPaymentWithdrawn` - Streaming payment withdrawn

### AccessControl

Main functions:
- `createAccessSession(uint256, address)` - Create new access session
- `requestDataAccess(bytes32, string)` - Request data access in session
- `terminateSession(bytes32, string)` - Terminate an active session
- `extendSession(bytes32)` - Extend session with additional purchases
- `validateAccess(bytes32, address)` - Validate session and get remaining time
- `getUserActiveSessions(address)` - Get all active sessions for user

Events:
- `AccessSessionCreated` - New session created
- `DataAccessRequested` - Data access requested
- `AccessSessionTerminated` - Session terminated

## 🛡️ Security Features

- **Reentrancy Protection**: All external calls protected with ReentrancyGuard
- **Access Control**: Proper ownership and authorization checks
- **Input Validation**: Comprehensive input validation and custom errors
- **Pausable Operations**: Emergency pause functionality for marketplace
- **Fee Limits**: Platform fees capped at 10% maximum
- **Session Limits**: Configurable concurrent session limits

## 🧪 Testing

The project includes comprehensive test coverage:

```bash
# Run all tests
forge test

# Run tests with verbosity
forge test -vv

# Run specific test file
forge test --match-path test/VehicleRegistry.t.sol

# Run with gas reporting
forge test --gas-report
```

Test files:
- `VehicleRegistry.t.sol` - Vehicle registration and management tests
- `DataMarketplace.t.sol` - Marketplace functionality and payment tests
- `AccessControl.t.sol` - Session management and access control tests

## 💡 Usage Examples

### Register a Vehicle

```solidity
string[] memory dataTypes = ["GPS", "Diagnostics", "Fuel"];
vehicleRegistry.registerVehicle{value: 0.01 ether}(
    "1HGCM82633A004352", // VIN
    vehicleWallet,       // Dedicated wallet address
    "Tesla",            // Manufacturer
    "Model 3",          // Model
    2023,              // Year
    dataTypes,         // Available data types
    "QmIPFSHash..."    // IPFS metadata hash
);
```

### List a Data Product

```solidity
dataMarketplace.listDataProduct(
    vehicleId,
    "GPS",                    // Data type
    0.001 ether,             // Price per hour
    3600,                    // Minimum duration (1 hour)
    86400,                   // Maximum duration (24 hours)
    "Real-time GPS data",    // Description
    "https://api.example.com/gps" // API endpoint
);
```

### Purchase Data Access

```solidity
// Purchase 2 hours of access
dataMarketplace.purchaseDataAccess{value: 0.002 ether}(
    productId,
    7200  // 2 hours in seconds
);
```

### Create Access Session

```solidity
bytes32 sessionKey = accessControl.createAccessSession(productId, buyer);
```

## 🔧 Configuration

### Platform Settings

- **Registration Fee**: Default 0.01 ETH (configurable by owner)
- **Platform Fee Rate**: Default 2.5% (max 10%, configurable by owner)
- **Max Concurrent Sessions**: Default 3 (configurable by owner)
- **Session Timeout Buffer**: Default 5 minutes (configurable by owner)

### Gas Optimization

The contracts are optimized for gas efficiency:
- Packed structs for storage efficiency
- Batch operations where possible
- Efficient access pattern checking
- Minimal external calls

## 🚨 Important Notes

1. **Vehicle Wallets**: Each vehicle must have a dedicated wallet address that can receive ETH payments
2. **IPFS Integration**: Vehicle metadata should be stored on IPFS with hashes stored on-chain
3. **API Endpoints**: Off-chain data access APIs should validate session keys before serving data
4. **Time-based Access**: All access durations are in seconds, with timestamps used for validation
5. **Payment Flow**: Payments are automatically split between vehicle wallets and platform fees

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Write comprehensive tests
4. Ensure all tests pass
5. Submit a pull request

## 📄 License

This project is licensed under the MIT License.

## 🔗 Additional Resources

- [Foundry Documentation](https://book.getfoundry.sh/)
- [OpenZeppelin Contracts](https://docs.openzeppelin.com/contracts/)
- [Solidity Documentation](https://docs.soliditylang.org/)

---

**Built for the future of connected vehicles and data monetization** 🚗💎