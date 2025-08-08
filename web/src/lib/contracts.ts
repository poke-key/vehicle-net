export const VEHICLE_REGISTRY_ADDRESS = process.env.NEXT_PUBLIC_VEHICLE_REGISTRY_ADDRESS || '0x'
export const DATA_MARKETPLACE_ADDRESS = process.env.NEXT_PUBLIC_DATA_MARKETPLACE_ADDRESS || '0x'

export const CONTRACT_ADDRESSES = {
  vehicleRegistry: VEHICLE_REGISTRY_ADDRESS,
  dataMarketplace: DATA_MARKETPLACE_ADDRESS,
  DATA_MARKETPLACE: DATA_MARKETPLACE_ADDRESS
}

export const VEHICLE_REGISTRY_ABI = [
  {
    inputs: [{ name: 'vin', type: 'string' }],
    name: 'getVehicle',
    outputs: [
      { name: 'owner', type: 'address' },
      { name: 'verified', type: 'bool' },
      { name: 'metadata', type: 'string' }
    ],
    stateMutability: 'view',
    type: 'function'
  },
  {
    inputs: [
      { name: 'vin', type: 'string' },
      { name: 'metadata', type: 'string' }
    ],
    name: 'registerVehicle',
    outputs: [],
    stateMutability: 'nonpayable',
    type: 'function'
  }
] as const

export const DATA_MARKETPLACE_ABI = [
  {
    inputs: [{ name: 'vehicleId', type: 'string' }],
    name: 'getVehicleData',
    outputs: [{ name: 'data', type: 'string' }],
    stateMutability: 'view',
    type: 'function'
  },
  {
    inputs: [
      { name: 'vehicleId', type: 'string' },
      { name: 'data', type: 'string' },
      { name: 'price', type: 'uint256' }
    ],
    name: 'listVehicleData',
    outputs: [],
    stateMutability: 'nonpayable',
    type: 'function'
  }
] as const