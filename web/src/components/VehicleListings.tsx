'use client';

import React, { useState } from 'react';
import { Card } from './ui/card';
import { Button } from './ui/button';
import { PurchaseModal } from './PurchaseModal';
import { useVehicleData, type DataProduct } from '@/hooks/useVehicleData';
import { formatEthValue } from '@/lib/porto-payments';

const getVehicleInfo = (vehicleId: string) => {
  const vehicles = {
    '1': { make: 'Tesla', model: 'Model 3', year: 2023 },
    '2': { make: 'BMW', model: 'i4', year: 2022 },
    '3': { make: 'Mercedes', model: 'EQS', year: 2023 },
  };
  return vehicles[vehicleId as keyof typeof vehicles] || { make: 'Unknown', model: 'Vehicle', year: 2023 };
};

export const VehicleListings: React.FC = () => {
  const { products, loading, error } = useVehicleData();
  const [selectedProduct, setSelectedProduct] = useState<DataProduct | null>(null);
  const [purchaseModalOpen, setPurchaseModalOpen] = useState(false);
  const [successMessage, setSuccessMessage] = useState<string | null>(null);

  const handlePurchaseClick = (product: DataProduct) => {
    setSelectedProduct(product);
    setPurchaseModalOpen(true);
  };

  const handlePurchaseSuccess = (txHash: string) => {
    setSuccessMessage(`Purchase successful! Transaction: ${txHash.slice(0, 10)}...`);
    setTimeout(() => setSuccessMessage(null), 5000);
  };

  if (loading) {
    return (
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        <div className="flex items-center justify-center min-h-[400px]">
          <div className="w-8 h-8 border-2 border-blue-600 border-t-transparent rounded-full animate-spin" />
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        <div className="bg-red-50 border border-red-200 rounded-md p-4">
          <p className="text-red-600">Error loading vehicle data: {error}</p>
        </div>
      </div>
    );
  }

  return (
    <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
      <div className="mb-8">
        <h2 className="text-2xl font-bold text-gray-900 dark:text-white mb-2">
          Available Vehicle Data
        </h2>
        <p className="text-gray-600 dark:text-gray-400">
          Browse and purchase access to real-time vehicle telemetry data
        </p>
      </div>

      {successMessage && (
        <div className="mb-6 bg-green-50 border border-green-200 rounded-md p-4">
          <p className="text-green-600">{successMessage}</p>
        </div>
      )}

      {products.length === 0 ? (
        <div className="text-center py-12">
          <p className="text-gray-500 dark:text-gray-400">No vehicle data products available</p>
        </div>
      ) : (
        <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
          {products.map((product) => {
            const vehicle = getVehicleInfo(product.vehicleId);
            const priceInEth = formatEthValue(product.pricePerHour);
            
            return (
              <Card key={product.id} className="hover:shadow-md transition-shadow">
                <div className="flex justify-between items-start mb-4">
                  <div>
                    <h3 className="text-lg font-semibold text-gray-900 dark:text-white">
                      {vehicle.make} {vehicle.model}
                    </h3>
                    <p className="text-gray-600 dark:text-gray-400 text-sm">
                      {vehicle.year} • ID: {product.vehicleId}
                    </p>
                  </div>
                  <span className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-green-100 text-green-800">
                    Active
                  </span>
                </div>

                <div className="space-y-2 mb-4">
                  <div className="flex justify-between">
                    <span className="text-sm text-gray-600 dark:text-gray-400">Data Type:</span>
                    <span className="text-sm font-medium text-gray-900 dark:text-white">{product.dataType}</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-sm text-gray-600 dark:text-gray-400">Min Duration:</span>
                    <span className="text-sm font-medium text-gray-900 dark:text-white">{Math.floor(product.minDuration / 3600)}h</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-sm text-gray-600 dark:text-gray-400">Max Duration:</span>
                    <span className="text-sm font-medium text-gray-900 dark:text-white">{Math.floor(product.maxDuration / 86400)}d</span>
                  </div>
                </div>

                <p className="text-sm text-gray-600 dark:text-gray-400 mb-4">{product.description}</p>

                <div className="flex items-center justify-between pt-4 border-t border-gray-200 dark:border-gray-600">
                  <span className="text-lg font-bold text-gray-900 dark:text-white">
                    {priceInEth} ETH/hr
                  </span>
                  <Button 
                    size="sm" 
                    onClick={() => handlePurchaseClick(product)}
                  >
                    Purchase Access
                  </Button>
                </div>
              </Card>
            );
          })}
        </div>
      )}

      {selectedProduct && (
        <PurchaseModal
          product={selectedProduct}
          isOpen={purchaseModalOpen}
          onClose={() => setPurchaseModalOpen(false)}
          onSuccess={handlePurchaseSuccess}
        />
      )}
    </div>
  );
};