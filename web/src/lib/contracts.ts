// Contract addresses - these will be dynamic based on deployment
export const CONTRACT_ADDRESSES = {
  // Default localhost addresses (Anvil/Hardhat)
  SIMPLE_VEHICLE_NODES: '0x5FbDB2315678afecb367f032d93F642f64180aa3',
  
  // Add addresses for other networks as needed
  // SIMPLE_VEHICLE_NODES_SEPOLIA: '0x...',
  // SIMPLE_VEHICLE_NODES_MAINNET: '0x...',
} as const

// Contract ABIs
export const SIMPLE_VEHICLE_NODES_ABI = [
  {
    "type": "function",
    "name": "addVehicleNode",
    "inputs": [
      {"type": "string", "name": "vin", "internalType": "string"},
      {"type": "address", "name": "nodeAddress", "internalType": "address"}
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updateVehicleReport", 
    "inputs": [
      {"type": "uint256", "name": "mileage", "internalType": "uint256"},
      {"type": "uint256", "name": "batteryHealth", "internalType": "uint256"}
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getTotalVehicles",
    "inputs": [],
    "outputs": [{"type": "uint256", "name": "", "internalType": "uint256"}],
    "stateMutability": "view"
  },
  {
    "type": "function", 
    "name": "getVehicle",
    "inputs": [{"type": "uint256", "name": "vehicleId", "internalType": "uint256"}],
    "outputs": [{"type": "tuple", "name": "", "components": [
      {"type": "string", "name": "vin", "internalType": "string"},
      {"type": "address", "name": "nodeAddress", "internalType": "address"},
      {"type": "uint256", "name": "mileage", "internalType": "uint256"},
      {"type": "uint256", "name": "batteryHealth", "internalType": "uint256"},
      {"type": "uint256", "name": "lastReportTimestamp", "internalType": "uint256"},
      {"type": "bool", "name": "isActive", "internalType": "bool"}
    ], "internalType": "struct SimpleVehicleNodes.VehicleNode"}],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getVehicleByAddress",
    "inputs": [{"type": "address", "name": "nodeAddress", "internalType": "address"}],
    "outputs": [{"type": "tuple", "name": "", "components": [
      {"type": "string", "name": "vin", "internalType": "string"},
      {"type": "address", "name": "nodeAddress", "internalType": "address"},
      {"type": "uint256", "name": "mileage", "internalType": "uint256"},
      {"type": "uint256", "name": "batteryHealth", "internalType": "uint256"},
      {"type": "uint256", "name": "lastReportTimestamp", "internalType": "uint256"},
      {"type": "bool", "name": "isActive", "internalType": "bool"}
    ], "internalType": "struct SimpleVehicleNodes.VehicleNode"}],
    "stateMutability": "view"
  },
  {
    "type": "event",
    "name": "VehicleNodeAdded",
    "inputs": [
      {"type": "uint256", "name": "vehicleId", "indexed": true, "internalType": "uint256"},
      {"type": "string", "name": "vin", "indexed": false, "internalType": "string"},
      {"type": "address", "name": "nodeAddress", "indexed": true, "internalType": "address"}
    ]
  },
  {
    "type": "event",
    "name": "VehicleReportUpdated", 
    "inputs": [
      {"type": "uint256", "name": "vehicleId", "indexed": true, "internalType": "uint256"},
      {"type": "uint256", "name": "mileage", "indexed": false, "internalType": "uint256"},
      {"type": "uint256", "name": "batteryHealth", "indexed": false, "internalType": "uint256"}
    ]
  }
] as const

// Helper function to get contract address for current chain
export function getContractAddress(chainId: number): `0x${string}` {
  // For now, always return the localhost address
  // In production, you'd check chainId and return appropriate address
  return CONTRACT_ADDRESSES.SIMPLE_VEHICLE_NODES as `0x${string}`
}

export interface VehicleNode {
  vin: string
  nodeAddress: string
  mileage: bigint
  batteryHealth: bigint
  lastReportTimestamp: bigint
  isActive: boolean
}