export interface VehicleData {
  id: string;
  make: string;
  model: string;
  year: number;
  mileage: number;
  price: string; // In wei
  owner: string;
  description: string;
  images: string[];
  vin?: string;
}

export interface TelemetryData {
  timestamp: number;
  gps: {
    latitude: number;
    longitude: number;
    speed: number;
  };
  diagnostics: {
    engineTemp: number;
    fuelLevel: number;
    batteryVoltage: number;
    errorCodes: string[];
  };
}

export interface DataListing {
  id: string;
  vehicleId: string;
  dataType: 'gps' | 'diagnostics' | 'full';
  price: string; // In wei
  duration: number; // In seconds
  seller: string;
  isActive: boolean;
  createdAt: number;
}