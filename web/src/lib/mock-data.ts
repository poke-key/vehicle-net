export interface MockVehicle {
  id: string
  vin: string
  make: string
  model: string
  year: number
  mileage: number
  batteryHealth?: number
  condition: string
  price: string
  owner: string
  verified: boolean
  title: string
  streamType: string
  location: string
  isActive: boolean
  frequency: string
  billingPeriod: string
  lastUpdated?: string
  description?: string
  features?: string[]
  totalSales?: number
  rating?: number
}

export const mockVehicleListings: MockVehicle[] = [
  {
    id: "1",
    vin: "1HGCM82633A123456",
    make: "Tesla",
    model: "Model 3",
    year: 2022,
    mileage: 15000,
    batteryHealth: 95,
    condition: "Excellent",
    price: "0.5",
    owner: "0x1234...5678",
    verified: true,
    title: "Tesla Model 3",
    streamType: "telemetry",
    location: "San Francisco, CA",
    isActive: true,
    frequency: "1s",
    billingPeriod: "hour",
    lastUpdated: new Date().toISOString(),
    description: "High-performance electric vehicle with advanced telemetry",
    features: ["Real-time battery monitoring", "GPS tracking", "Performance metrics"],
    totalSales: 42,
    rating: 4.8
  },
  {
    id: "2", 
    vin: "1HGCM82633A123457",
    make: "Tesla",
    model: "Model Y",
    year: 2023,
    mileage: 8000,
    batteryHealth: 98,
    condition: "Like New",
    price: "0.8",
    owner: "0xabcd...efgh",
    verified: true,
    title: "Tesla Model Y",
    streamType: "gps",
    location: "Los Angeles, CA",
    isActive: true,
    frequency: "5s",
    billingPeriod: "hour",
    lastUpdated: new Date().toISOString(),
    description: "Premium SUV with comprehensive GPS tracking capabilities",
    features: ["Precise location data", "Route optimization", "Geofencing"],
    totalSales: 28,
    rating: 4.9
  },
  {
    id: "3",
    vin: "1HGCM82633A123458", 
    make: "Ford",
    model: "Mustang Mach-E",
    year: 2021,
    mileage: 25000,
    batteryHealth: 92,
    condition: "Good",
    price: "0.4",
    owner: "0x9876...5432",
    verified: false,
    title: "Ford Mustang Mach-E",
    streamType: "diagnostics",
    location: "Austin, TX",
    isActive: false,
    frequency: "10s",
    billingPeriod: "hour",
    lastUpdated: new Date().toISOString(),
    description: "Electric performance SUV with detailed diagnostic capabilities",
    features: ["Engine diagnostics", "Battery analysis", "System monitoring"],
    totalSales: 15,
    rating: 4.5
  }
]

export const mockVehicles = mockVehicleListings

export const mockContractABI = [
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
  }
] as const

export const mockContractAddress = '0x1234567890123456789012345678901234567890'