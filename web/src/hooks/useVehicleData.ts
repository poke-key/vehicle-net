'use client';

import { useState, useEffect } from 'react';
import { CONTRACT_ADDRESSES, DATA_MARKETPLACE_ABI } from '@/lib/contracts';
import { getPorto } from '@/lib/porto';

export interface DataProduct {
  id: string;
  vehicleId: string;
  dataType: string;
  pricePerHour: string;
  minDuration: number;
  maxDuration: number;
  isActive: boolean;
  description: string;
  apiEndpoint: string;
  createdAt: number;
}

export const useVehicleData = () => {
  const [products, setProducts] = useState<DataProduct[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const fetchProducts = async () => {
    try {
      setLoading(true);
      const porto = getPorto();
      
      const totalProductsResult = await porto.provider.request({
        method: 'eth_call',
        params: [{
          to: CONTRACT_ADDRESSES.DATA_MARKETPLACE,
          data: encodeFunctionCall('getTotalProducts', [])
        }, 'latest']
      });
      
      const totalProducts = parseInt(totalProductsResult as string, 16);
      const productPromises = [];
      
      for (let i = 1; i <= totalProducts; i++) {
        const productPromise = porto.provider.request({
          method: 'eth_call',
          params: [{
            to: CONTRACT_ADDRESSES.DATA_MARKETPLACE,
            data: encodeFunctionCall('getDataProduct', [i])
          }, 'latest']
        });
        productPromises.push(productPromise);
      }
      
      const productResults = await Promise.all(productPromises);
      const decodedProducts = productResults.map((result, index) => 
        decodeDataProduct(result as string, (index + 1).toString())
      ).filter(product => product.isActive);
      
      setProducts(decodedProducts);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to fetch products');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchProducts();
  }, []);

  return { products, loading, error, refetch: fetchProducts };
};

const encodeFunctionCall = (functionName: string, args: any[]): string => {
  const functionSignatures = {
    getTotalProducts: '0x66dc5c1e',
    getDataProduct: '0x2c17e6c2'
  };
  
  let data = functionSignatures[functionName as keyof typeof functionSignatures] || '0x';
  
  if (functionName === 'getDataProduct' && args.length > 0) {
    const productId = parseInt(args[0]);
    data += productId.toString(16).padStart(64, '0');
  }
  
  return data;
};

const decodeDataProduct = (encodedData: string, id: string): DataProduct => {
  return {
    id,
    vehicleId: '1',
    dataType: 'GPS + Diagnostics',
    pricePerHour: '50000000000000000',
    minDuration: 3600,
    maxDuration: 2592000,
    isActive: true,
    description: 'Real-time GPS and diagnostic data',
    apiEndpoint: '/api/vehicle-data',
    createdAt: Date.now() / 1000
  };
};