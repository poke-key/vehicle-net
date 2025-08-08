'use client';

import React, { useState } from 'react';
import { Card } from './ui/Card';
import { Button } from './ui/Button';
import { purchaseDataAccessWithPorto, calculatePurchaseCost, formatEthValue } from '@/lib/porto-payments';
import { useWallet } from '@/components/WalletProvider';
import { DataProduct } from '@/hooks/useVehicleData';

interface PurchaseModalProps {
  product: DataProduct;
  isOpen: boolean;
  onClose: () => void;
  onSuccess: (txHash: string) => void;
}

const DURATION_OPTIONS = [
  { label: '1 Hour', value: 3600 },
  { label: '6 Hours', value: 21600 },
  { label: '24 Hours', value: 86400 },
  { label: '7 Days', value: 604800 },
  { label: '30 Days', value: 2592000 },
];

export const PurchaseModal: React.FC<PurchaseModalProps> = ({
  product,
  isOpen,
  onClose,
  onSuccess,
}) => {
  const [selectedDuration, setSelectedDuration] = useState(3600);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const { isPortoConnected, isConnected, connectWithPorto } = useWallet();

  const totalCost = calculatePurchaseCost(product.pricePerHour, selectedDuration);
  const costInEth = formatEthValue(totalCost);

  const handlePurchase = async () => {
    try {
      setLoading(true);
      setError(null);
      
      // Ensure Porto is connected for payments
      if (!isPortoConnected) {
        setError('Porto connection required for payments. Please connect with Porto first.');
        return;
      }
      
      const txHash = await purchaseDataAccessWithPorto({
        productId: product.id,
        durationInSeconds: selectedDuration,
        priceInWei: totalCost,
      });
      
      onSuccess(txHash);
      onClose();
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Purchase failed');
    } finally {
      setLoading(false);
    }
  };

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4 z-50">
      <Card className="w-full max-w-md">
        <div className="flex justify-between items-center mb-6">
          <h3 className="text-lg font-semibold">Purchase Data Access</h3>
          <button
            onClick={onClose}
            className="text-gray-400 hover:text-gray-600 text-xl"
          >
            ×
          </button>
        </div>

        <div className="space-y-4">
          <div>
            <h4 className="font-medium text-gray-900 mb-2">Vehicle Data</h4>
            <p className="text-sm text-gray-600">{product.description}</p>
            <p className="text-sm text-gray-500 mt-1">
              Type: {product.dataType} • Vehicle ID: {product.vehicleId}
            </p>
          </div>

          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              Access Duration
            </label>
            <select
              value={selectedDuration}
              onChange={(e) => setSelectedDuration(Number(e.target.value))}
              className="w-full p-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            >
              {DURATION_OPTIONS.map((option) => (
                <option key={option.value} value={option.value}>
                  {option.label}
                </option>
              ))}
            </select>
          </div>

          <div className="bg-gray-50 rounded-lg p-4">
            <div className="flex justify-between items-center">
              <span className="text-sm text-gray-600">Total Cost:</span>
              <span className="text-lg font-bold text-gray-900">
                {costInEth} ETH
              </span>
            </div>
            <p className="text-xs text-gray-500 mt-1">
              Includes platform fees • Paid via Porto
            </p>
          </div>

          {error && (
            <div className="bg-red-50 border border-red-200 rounded-md p-3">
              <p className="text-sm text-red-600">{error}</p>
            </div>
          )}

          <div className="flex gap-3 pt-4">
            <Button
              variant="outline"
              onClick={onClose}
              disabled={loading}
              className="flex-1"
            >
              Cancel
            </Button>
            {!isPortoConnected ? (
              <Button
                onClick={connectWithPorto}
                loading={loading}
                className="flex-1"
              >
                Connect Porto to Pay
              </Button>
            ) : (
              <Button
                onClick={handlePurchase}
                loading={loading}
                className="flex-1"
              >
                {loading ? 'Processing...' : `Pay ${costInEth} ETH`}
              </Button>
            )}
          </div>
        </div>
      </Card>
    </div>
  );
};