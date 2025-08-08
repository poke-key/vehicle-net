'use client';

import React, { useState } from 'react';
import { Card } from './ui/card';
import { Button } from './ui/button';
import { Badge } from './ui/badge';
import { useAllVehicles, type VehicleWithMetadata } from '@/hooks/useMockVehicleData';
import { formatEther } from 'viem';

export const VehicleListings: React.FC = () => {
  const { vehicles, isLoading, error, refetch } = useAllVehicles();
  const [selectedVehicle, setSelectedVehicle] = useState<VehicleWithMetadata | null>(null);

  const formatTimestamp = (timestamp: number) => {
    return new Date(timestamp * 1000).toLocaleString();
  };

  if (isLoading) {
    return (
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        <div className="flex items-center justify-center min-h-[400px]">
          <div className="w-8 h-8 border-2 border-blue-600 border-t-transparent rounded-full animate-spin" />
          <span className="ml-2 text-gray-600">Loading vehicles...</span>
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        <div className="bg-red-50 border border-red-200 rounded-md p-4">
          <p className="text-red-600">Error loading vehicles: {error.message}</p>
          <Button onClick={refetch} className="mt-2" variant="outline" size="sm">
            Retry
          </Button>
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

      {vehicles.length === 0 ? (
        <div className="text-center py-12">
          <div className="max-w-md mx-auto">
            <h3 className="text-lg font-medium text-gray-900 mb-2">No vehicles registered yet</h3>
            <p className="text-gray-500 mb-4">
              No vehicle data available at the moment. Check back later for new listings.
            </p>
          </div>
        </div>
      ) : (
        <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
          {vehicles.map((vehicle) => (
            <Card key={vehicle.id} className="hover:shadow-md transition-shadow">
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
                  <span className="text-sm text-gray-600">Price per hour:</span>
                  <span className="text-sm font-medium">{vehicle.price} ETH</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-sm text-gray-600">Owner:</span>
                  <span className="text-sm font-mono">{vehicle.owner.slice(0, 8)}...</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-sm text-gray-600">Wallet:</span>
                  <span className="text-sm font-mono">{vehicle.wallet.slice(0, 8)}...</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-sm text-gray-600">Location:</span>
                  <span className="text-sm font-medium">{vehicle.location}</span>
                </div>
                {vehicle.batteryHealth && (
                  <div className="flex justify-between">
                    <span className="text-sm text-gray-600">Battery Health:</span>
                    <span className="text-sm font-medium">{vehicle.batteryHealth}%</span>
                  </div>
                )}
                <div className="flex justify-between">
                  <span className="text-sm text-gray-600">Mileage:</span>
                  <span className="text-sm font-medium">{vehicle.mileage?.toLocaleString()} mi</span>
                </div>
              </div>

              {vehicle.dataTypes.length > 0 && (
                <div className="mb-4">
                  <span className="text-sm text-gray-600 block mb-2">Available Data:</span>
                  <div className="flex flex-wrap gap-1">
                    {vehicle.dataTypes.map((dataType, index) => (
                      <Badge key={index} variant="outline" className="text-xs">
                        {dataType}
                      </Badge>
                    ))}
                  </div>
                </div>
              )}

              {vehicle.ipfsHash && (
                <div className="mb-4">
                  <span className="text-sm text-gray-600">IPFS Hash:</span>
                  <p className="text-xs font-mono text-gray-500 break-all">
                    {vehicle.ipfsHash}
                  </p>
                </div>
              )}

              <div className="flex items-center justify-between pt-4 border-t border-gray-200">
                <span className="text-sm text-gray-600">
                  Last Update: {formatTimestamp(vehicle.lastUpdate)}
                </span>
                <Button 
                  size="sm" 
                  variant="outline"
                  onClick={() => setSelectedVehicle(vehicle)}
                >
                  View Details
                </Button>
              </div>
            </Card>
          ))}
        </div>
      )}

      {selectedVehicle && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4 z-50">
          <div className="bg-white rounded-lg p-6 max-w-md w-full">
            <h3 className="text-lg font-bold mb-4">Vehicle Details</h3>
            <div className="space-y-2">
              <p><strong>VIN:</strong> {selectedVehicle.vin}</p>
              <p><strong>Manufacturer:</strong> {selectedVehicle.manufacturer}</p>
              <p><strong>Model:</strong> {selectedVehicle.model}</p>
              <p><strong>Year:</strong> {selectedVehicle.year}</p>
              <p><strong>Owner:</strong> {selectedVehicle.owner}</p>
              <p><strong>Wallet:</strong> {selectedVehicle.wallet}</p>
              <p><strong>Status:</strong> {selectedVehicle.isActive ? "Active" : "Inactive"}</p>
            </div>
            <Button 
              className="mt-4 w-full" 
              onClick={() => setSelectedVehicle(null)}
            >
              Close
            </Button>
          </div>
        </div>
      )}
    </div>
  );
};