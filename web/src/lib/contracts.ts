export const VEHICLE_REGISTRY_ADDRESS = process.env.NEXT_PUBLIC_REGISTRY_ADDRESS as `0x${string}`
export const DATA_MARKETPLACE_ADDRESS = process.env.NEXT_PUBLIC_MARKETPLACE_ADDRESS as `0x${string}`
export const ACCESS_CONTROL_ADDRESS = process.env.NEXT_PUBLIC_ACCESS_CONTROL_ADDRESS as `0x${string}`

export const CONTRACT_ADDRESSES = {
  vehicleRegistry: VEHICLE_REGISTRY_ADDRESS,
  dataMarketplace: DATA_MARKETPLACE_ADDRESS,
  accessControl: ACCESS_CONTROL_ADDRESS,
}

export const VEHICLE_REGISTRY_ABI = [
  {
    inputs: [
      { name: 'vehicleId', type: 'uint256' }
    ],
    name: 'getVehicle',
    outputs: [
      {
        components: [
          { name: 'vin', type: 'string' },
          { name: 'wallet', type: 'address' },
          { name: 'manufacturer', type: 'string' },
          { name: 'model', type: 'string' },
          { name: 'year', type: 'uint256' },
          { name: 'isActive', type: 'bool' },
          { name: 'registrationTimestamp', type: 'uint256' },
          { name: 'owner', type: 'address' }
        ],
        name: '',
        type: 'tuple'
      }
    ],
    stateMutability: 'view',
    type: 'function'
  },
  {
    inputs: [],
    name: 'getTotalVehicles',
    outputs: [{ name: '', type: 'uint256' }],
    stateMutability: 'view',
    type: 'function'
  },
  {
    inputs: [
      { name: 'vehicleId', type: 'uint256' }
    ],
    name: 'getVehicleMetadata',
    outputs: [
      {
        components: [
          { name: 'dataTypes', type: 'string[]' },
          { name: 'ipfsHash', type: 'string' },
          { name: 'lastUpdate', type: 'uint256' }
        ],
        name: '',
        type: 'tuple'
      }
    ],
    stateMutability: 'view',
    type: 'function'
  },
  {
    inputs: [
      { name: 'wallet', type: 'address' }
    ],
    name: 'getVehicleIdByWallet',
    outputs: [{ name: '', type: 'uint256' }],
    stateMutability: 'view',
    type: 'function'
  },
  {
    inputs: [
      { name: 'vin', type: 'string' }
    ],
    name: 'getVehicleIdByVin',
    outputs: [{ name: '', type: 'uint256' }],
    stateMutability: 'view',
    type: 'function'
  },
  {
    inputs: [
      { name: 'vehicleId', type: 'uint256' }
    ],
    name: 'isVehicleActive',
    outputs: [{ name: '', type: 'bool' }],
    stateMutability: 'view',
    type: 'function'
  },
  {
    inputs: [
      { name: 'vin', type: 'string' },
      { name: 'vehicleWallet', type: 'address' },
      { name: 'manufacturer', type: 'string' },
      { name: 'model', type: 'string' },
      { name: 'year', type: 'uint256' },
      { name: 'dataTypes', type: 'string[]' },
      { name: 'ipfsHash', type: 'string' }
    ],
    name: 'registerVehicle',
    outputs: [],
    stateMutability: 'payable',
    type: 'function'
  }
] as const

export const DATA_MARKETPLACE_ABI = [
  // Add relevant marketplace functions here based on your DataMarketplace contract
] as const