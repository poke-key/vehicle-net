'use client';

import React, { useState } from 'react';
import { Card } from './ui/card';
import { Button } from './ui/button';
import { Badge } from './ui/badge';
import { useTotalVehicles, useVehicleData } from '@/hooks/useVehicleContract';

interface VehicleListingsRealProps {
  showMockData?: boolean;
}

export const VehicleListingsReal: React.FC<VehicleListingsRealProps> = ({ showMockData = false }) => {
  const { totalVehicles, isLoading: totalLoading, error: totalError, refetch } = useTotalVehicles();
  const [selectedVehicleId, setSelectedVehicleId] = useState<number | null>(null);

  const formatTimestamp = (timestamp: number) => {
    return new Date(timestamp * 1000).toLocaleString();
  };

  const formatAddress = (address: string) => {
    return `${address.slice(0, 6)}...${address.slice(-4)}`;
  };

  if (totalLoading) {
    return (
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        <div className="flex items-center justify-center min-h-[400px]">
          <div className="w-8 h-8 border-2 border-blue-600 border-t-transparent rounded-full animate-spin" />
          <span className="ml-2 text-gray-600">Loading vehicles from blockchain...</span>
        </div>
      </div>
    );
  }

  if (totalError) {
    return (
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        <div className="bg-red-50 border border-red-200 rounded-md p-4">
          <p className="text-red-600">Error loading vehicles: {totalError.message}</p>
          <Button onClick={refetch} className="mt-2" variant="outline" size="sm">
            Retry
          </Button>
        </div>
      </div>
    );
  }

  if (totalVehicles === 0) {
    return (
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        <div className="mb-8">
          <div className="flex items-center justify-between">
            <div>
              <h2 className="text-2xl font-bold text-gray-900 mb-2">
                Registered Vehicles
              </h2>
              <p className="text-gray-600">
                Browse available vehicle data streams in the marketplace
              </p>
            </div>
            <Button onClick={refetch} variant="outline" size="sm">
              Refresh
            </Button>
          </div>
        </div>

        <div className="text-center py-12">
          <div className="max-w-md mx-auto">
            <h3 className="text-lg font-medium text-gray-900 mb-2">No vehicles registered yet</h3>
            <p className="text-gray-500 mb-4">
              No vehicle data available on the blockchain. Register a vehicle using the Rust CLI to see it here.
            </p>
            <div className="mt-6 p-4 bg-blue-50 rounded-lg">
              <p className="text-sm text-blue-600 font-medium mb-2">To register a vehicle:</p>
              <code className="block text-xs bg-gray-100 p-2 rounded text-left">
                # From the rust directory:<br/>
                cargo run -- --index 0
              </code>
            </div>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
      <div className="mb-8">
        <div className="flex items-center justify-between">
          <div>
            <h2 className="text-2xl font-bold text-gray-900 mb-2">
              Registered Vehicles ({totalVehicles})
            </h2>
            <p className="text-gray-600">
              Browse available vehicle data streams in the marketplace
            </p>
          </div>
          <Button onClick={refetch} variant="outline" size="sm">
            Refresh
          </Button>
        </div>
      </div>

      <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
        {Array.from({ length: totalVehicles }, (_, index) => (
          <VehicleCard 
            key={index + 1} 
            vehicleId={index + 1} 
            onSelect={setSelectedVehicleId}
          />
        ))}
      </div>

      {selectedVehicleId && (
        <VehicleModal 
          vehicleId={selectedVehicleId}
          onClose={() => setSelectedVehicleId(null)}
        />
      )}
    </div>
  );
};

interface VehicleCardProps {
  vehicleId: number;
  onSelect: (id: number) => void;
}

const VehicleCard: React.FC<VehicleCardProps> = ({ vehicleId, onSelect }) => {
  const { vehicle, isLoading, error } = useVehicleData(vehicleId);

  if (isLoading) {
    return (
      <Card className="animate-pulse">
        <div className="h-32 bg-gray-200 rounded mb-4"></div>
        <div className="h-4 bg-gray-200 rounded mb-2"></div>
        <div className="h-4 bg-gray-200 rounded w-3/4"></div>
      </Card>
    );
  }

  if (error || !vehicle) {
    return (
      <Card className="border-red-200">
        <div className="text-red-600 text-sm p-4">
          Failed to load vehicle #{vehicleId}
        </div>
      </Card>
    );
  }

  const formatTimestamp = (timestamp: number) => {
    return new Date(timestamp * 1000).toLocaleString();
  };

  const formatAddress = (address: string) => {
    return `${address.slice(0, 6)}...${address.slice(-4)}`;
  };

  return (
    <Card className="hover:shadow-md transition-shadow">
      <div className="flex justify-between items-start mb-4">
        <div>
          <h3 className="text-lg font-semibold text-gray-900">
            {vehicle.manufacturer} {vehicle.model}
          </h3>
          <p className="text-gray-600 text-sm">
            {vehicle.year} • VIN: {vehicle.vin}
          </p>
        </div>
        <Badge variant={vehicle.isActive ? "default" : "secondary"}>
          {vehicle.isActive ? "Active" : "Inactive"}
        </Badge>
      </div>

      <div className="space-y-2 mb-4">
        <div className="flex justify-between">
          <span className="text-sm text-gray-600">Vehicle ID:</span>
          <span className="text-sm font-medium">#{vehicleId}</span>
        </div>
        <div className="flex justify-between">
          <span className="text-sm text-gray-600">Owner:</span>
          <span className="text-sm font-mono">{formatAddress(vehicle.owner)}</span>
        </div>
        <div className="flex justify-between">
          <span className="text-sm text-gray-600">Wallet:</span>
          <span className="text-sm font-mono">{formatAddress(vehicle.wallet)}</span>
        </div>
        <div className="flex justify-between">
          <span className="text-sm text-gray-600">Registered:</span>
          <span className="text-sm font-medium">
            {formatTimestamp(vehicle.registrationTimestamp)}
          </span>
        </div>
      </div>

      <div className="flex items-center justify-between pt-4 border-t border-gray-200">
        <span className="text-xs text-gray-500">
          From blockchain
        </span>
        <Button 
          size="sm" 
          variant="outline"
          onClick={() => onSelect(vehicleId)}
        >
          View Details
        </Button>
      </div>
    </Card>
  );
};

interface VehicleModalProps {
  vehicleId: number;
  onClose: () => void;
}

const VehicleModal: React.FC<VehicleModalProps> = ({ vehicleId, onClose }) => {
  const { vehicle, isLoading, error } = useVehicleData(vehicleId);

  if (isLoading) {
    return (
      <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4 z-50">
        <div className="bg-white rounded-lg p-6 max-w-md w-full">
          <div className="animate-pulse">
            <div className="h-6 bg-gray-200 rounded mb-4"></div>
            <div className="space-y-2">
              {Array.from({ length: 6 }).map((_, i) => (
                <div key={i} className="h-4 bg-gray-200 rounded"></div>
              ))}
            </div>
          </div>
        </div>
      </div>
    );
  }

  if (error || !vehicle) {
    return (
      <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4 z-50">
        <div className="bg-white rounded-lg p-6 max-w-md w-full">
          <h3 className="text-lg font-bold mb-4">Error</h3>
          <p className="text-red-600">Failed to load vehicle details</p>
          <Button className="mt-4 w-full" onClick={onClose}>
            Close
          </Button>
        </div>
      </div>
    );
  }

  const formatTimestamp = (timestamp: number) => {
    return new Date(timestamp * 1000).toLocaleString();
  };

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4 z-50">
      <div className="bg-white rounded-lg p-6 max-w-md w-full max-h-[80vh] overflow-y-auto">
        <h3 className="text-lg font-bold mb-4">Vehicle Details #{vehicleId}</h3>
        <div className="space-y-3">
          <div>
            <strong className="text-sm text-gray-600">VIN:</strong>
            <p className="font-mono text-sm break-all">{vehicle.vin}</p>
          </div>
          <div>
            <strong className="text-sm text-gray-600">Manufacturer:</strong>
            <p>{vehicle.manufacturer}</p>
          </div>
          <div>
            <strong className="text-sm text-gray-600">Model:</strong>
            <p>{vehicle.model}</p>
          </div>
          <div>
            <strong className="text-sm text-gray-600">Year:</strong>
            <p>{vehicle.year}</p>
          </div>
          <div>
            <strong className="text-sm text-gray-600">Owner:</strong>
            <p className="font-mono text-xs break-all">{vehicle.owner}</p>
          </div>
          <div>
            <strong className="text-sm text-gray-600">Vehicle Wallet:</strong>
            <p className="font-mono text-xs break-all">{vehicle.wallet}</p>
          </div>
          <div>
            <strong className="text-sm text-gray-600">Status:</strong>
            <p>{vehicle.isActive ? "Active" : "Inactive"}</p>
          </div>
          <div>
            <strong className="text-sm text-gray-600">Registration Time:</strong>
            <p className="text-sm">{formatTimestamp(vehicle.registrationTimestamp)}</p>
          </div>
        </div>
        <Button 
          className="mt-6 w-full" 
          onClick={onClose}
        >
          Close
        </Button>
      </div>
    </div>
  );
};