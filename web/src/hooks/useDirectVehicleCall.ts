import { useState, useEffect } from 'react';
import { createPublicClient, http } from 'viem';
import { defineChain } from 'viem';

const anvil = defineChain({
  id: 31337,
  name: 'Anvil',
  network: 'anvil',
  nativeCurrency: {
    decimals: 18,
    name: 'Ether',
    symbol: 'ETH',
  },
  rpcUrls: {
    public: { http: ['http://localhost:8545'] },
    default: { http: ['http://localhost:8545'] },
  },
});

const VEHICLE_REGISTRY_ABI = [
  {
    inputs: [],
    name: 'getTotalVehicles',
    outputs: [{ name: '', type: 'uint256' }],
    stateMutability: 'view',
    type: 'function'
  }
];

export function useDirectTotalVehicles() {
  const [totalVehicles, setTotalVehicles] = useState<number | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<Error | null>(null);

  const fetchTotalVehicles = async () => {
    try {
      setIsLoading(true);
      setError(null);

      const client = createPublicClient({
        chain: anvil,
        transport: http('http://localhost:8545'),
      });

      const result = await client.readContract({
        address: '0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9',
        abi: VEHICLE_REGISTRY_ABI,
        functionName: 'getTotalVehicles',
      });

      const totalCount = Number(result);
      setTotalVehicles(totalCount);
      console.log('Direct contract call result:', totalCount);
    } catch (err) {
      setError(err instanceof Error ? err : new Error('Unknown error occurred'));
      console.error('Error fetching total vehicles directly:', err);
    } finally {
      setIsLoading(false);
    }
  };

  useEffect(() => {
    fetchTotalVehicles();
  }, []);

  return { totalVehicles, isLoading, error, refetch: fetchTotalVehicles };
}