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
  },
  {
    id: "4",
    vin: "1HGCM82633A123459",
    make: "Chevrolet",
    model: "Bolt EV",
    year: 2022,
    mileage: 12000,
    batteryHealth: 96,
    condition: "Excellent",
    price: "0.3",
    owner: "0xdef0...1234",
    verified: true,
    title: "Chevrolet Bolt EV",
    streamType: "gps",
    location: "Denver, CO",
    isActive: true,
    frequency: "2s",
    billingPeriod: "hour",
    lastUpdated: new Date().toISOString(),
    description: "Affordable electric vehicle with reliable GPS tracking",
    features: ["Real-time location", "Speed monitoring", "Trip history"],
    totalSales: 33,
    rating: 4.6
  },
  {
    id: "5",
    vin: "1HGCM82633A123460",
    make: "BMW",
    model: "iX",
    year: 2023,
    mileage: 5000,
    batteryHealth: 99,
    condition: "Like New",
    price: "1.2",
    owner: "0x2468...acef",
    verified: true,
    title: "BMW iX Luxury SUV",
    streamType: "telemetry",
    location: "Miami, FL",
    isActive: true,
    frequency: "500ms",
    billingPeriod: "hour",
    lastUpdated: new Date().toISOString(),
    description: "Luxury electric SUV with premium telemetry package",
    features: ["Advanced driver assistance", "Luxury interior monitoring", "Climate control data", "Charging analytics"],
    totalSales: 18,
    rating: 4.9
  },
  {
    id: "6",
    vin: "1HGCM82633A123461",
    make: "Rivian",
    model: "R1T",
    year: 2022,
    mileage: 18000,
    batteryHealth: 94,
    condition: "Good",
    price: "0.7",
    owner: "0x1357...9bdf",
    verified: true,
    title: "Rivian R1T Electric Truck",
    streamType: "diagnostics",
    location: "Seattle, WA",
    isActive: true,
    frequency: "3s",
    billingPeriod: "hour",
    lastUpdated: new Date().toISOString(),
    description: "Electric pickup truck with comprehensive diagnostic monitoring",
    features: ["Truck-specific diagnostics", "Towing analytics", "Off-road metrics", "Battery thermal management"],
    totalSales: 22,
    rating: 4.7
  }
]

export const mockVehicles = mockVehicleListings

// Mock Data Products for the marketplace
export const mockDataProducts = [
  {
    id: "dp-1",
    vehicleId: "1",
    dataType: "telemetry",
    pricePerHour: "1000000000000000", // 0.001 ETH in wei
    minDuration: 3600, // 1 hour
    maxDuration: 2592000, // 30 days
    description: "Real-time Tesla Model 3 telemetry including battery health, charging data, and performance metrics",
    apiEndpoint: "/api/mock/vehicles/1/telemetry",
    isActive: true
  },
  {
    id: "dp-2",
    vehicleId: "2",
    dataType: "gps",
    pricePerHour: "800000000000000", // 0.0008 ETH in wei
    minDuration: 1800, // 30 minutes
    maxDuration: 1209600, // 14 days
    description: "Precise GPS tracking data from Tesla Model Y with sub-meter accuracy",
    apiEndpoint: "/api/mock/vehicles/2/gps",
    isActive: true
  },
  {
    id: "dp-3",
    vehicleId: "4",
    dataType: "gps",
    pricePerHour: "600000000000000", // 0.0006 ETH in wei
    minDuration: 3600,
    maxDuration: 604800, // 7 days
    description: "Affordable GPS data stream from Chevrolet Bolt EV for fleet tracking applications",
    apiEndpoint: "/api/mock/vehicles/4/gps",
    isActive: true
  },
  {
    id: "dp-4",
    vehicleId: "5",
    dataType: "telemetry",
    pricePerHour: "2000000000000000", // 0.002 ETH in wei
    minDuration: 7200, // 2 hours
    maxDuration: 2592000,
    description: "Premium BMW iX telemetry with luxury features monitoring and advanced driver assistance data",
    apiEndpoint: "/api/mock/vehicles/5/telemetry",
    isActive: true
  },
  {
    id: "dp-5",
    vehicleId: "6",
    dataType: "diagnostics",
    pricePerHour: "1200000000000000", // 0.0012 ETH in wei
    minDuration: 3600,
    maxDuration: 1728000, // 20 days
    description: "Comprehensive Rivian R1T diagnostic data including towing metrics and off-road performance",
    apiEndpoint: "/api/mock/vehicles/6/diagnostics",
    isActive: true
  }
];

// Mock telemetry data for real-time display
export const mockTelemetryData = {
  "1": {
    timestamp: Date.now(),
    gps: { latitude: 37.7749, longitude: -122.4194, speed: 65 },
    diagnostics: { engineTemp: 190, fuelLevel: 85, batteryVoltage: 12.6, errorCodes: [] },
    batteryHealth: 95,
    chargingStatus: "not_charging",
    range: 280
  },
  "2": {
    timestamp: Date.now(),
    gps: { latitude: 34.0522, longitude: -118.2437, speed: 45 },
    diagnostics: { engineTemp: 185, fuelLevel: 92, batteryVoltage: 12.8, errorCodes: [] },
    batteryHealth: 98,
    chargingStatus: "not_charging", 
    range: 310
  },
  "4": {
    timestamp: Date.now(),
    gps: { latitude: 39.7392, longitude: -104.9903, speed: 55 },
    diagnostics: { engineTemp: 180, fuelLevel: 78, batteryVoltage: 12.4, errorCodes: [] },
    batteryHealth: 96,
    chargingStatus: "charging",
    range: 220
  },
  "5": {
    timestamp: Date.now(),
    gps: { latitude: 25.7617, longitude: -80.1918, speed: 35 },
    diagnostics: { engineTemp: 175, fuelLevel: 88, batteryVoltage: 13.1, errorCodes: [] },
    batteryHealth: 99,
    chargingStatus: "not_charging",
    range: 380
  },
  "6": {
    timestamp: Date.now(),
    gps: { latitude: 47.6062, longitude: -122.3321, speed: 70 },
    diagnostics: { engineTemp: 195, fuelLevel: 65, batteryVoltage: 12.9, errorCodes: ["P0420"] },
    batteryHealth: 94,
    chargingStatus: "not_charging",
    range: 290
  }
};

// Mock marketplace statistics
export const mockMarketplaceStats = {
  totalVehicles: mockVehicleListings.length,
  activeListings: mockVehicleListings.filter(v => v.isActive).length,
  totalSales: mockVehicleListings.reduce((acc, v) => acc + (v.totalSales || 0), 0),
  averageRating: mockVehicleListings.reduce((acc, v) => acc + (v.rating || 0), 0) / mockVehicleListings.length,
  totalValueLocked: "15.8", // ETH
  recentTransactions: [
    { id: "tx-1", vehicleId: "1", amount: "0.5", timestamp: Date.now() - 3600000 },
    { id: "tx-2", vehicleId: "2", amount: "0.8", timestamp: Date.now() - 7200000 },
    { id: "tx-3", vehicleId: "4", amount: "0.3", timestamp: Date.now() - 10800000 },
  ]
};

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