"use client";

import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { Car, Clock } from "lucide-react";
import Link from "next/link";
import { useState, useEffect } from "react";
import { createPublicClient, http } from 'viem';
import { defineChain } from 'viem';

// Define local Anvil chain
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

export default function Dashboard() {
  const [totalVehicles, setTotalVehicles] = useState<number | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<Error | null>(null);

  useEffect(() => {
    async function fetchVehicles() {
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

        setTotalVehicles(Number(result));
      } catch (err) {
        setError(err instanceof Error ? err : new Error('Failed to fetch vehicles'));
      } finally {
        setIsLoading(false);
      }
    }

    fetchVehicles();
  }, []);

  if (isLoading) {
    return (
      <div className="container mx-auto px-4 py-8">
        <div className="text-center py-12">
          <Car className="mx-auto h-12 w-12 text-muted-foreground mb-4 animate-pulse" />
          <h3 className="text-lg font-semibold mb-2">Loading vehicles from blockchain...</h3>
          <p className="text-muted-foreground">Fetching vehicle data from smart contracts...</p>
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="container mx-auto px-4 py-8">
        <div className="text-center py-12">
          <h3 className="text-lg font-semibold mb-2 text-destructive">Failed to load vehicles</h3>
          <p className="text-muted-foreground mb-4">
            Could not connect to blockchain or load vehicle data: {error.message}
          </p>
          <Button onClick={() => window.location.reload()}>Try Again</Button>
        </div>
      </div>
    );
  }

  if (totalVehicles === 0) {
    return (
      <div className="container mx-auto px-4 py-8">
        <div className="text-center py-12">
          <Car className="mx-auto h-12 w-12 text-muted-foreground mb-4" />
          <h3 className="text-lg font-semibold mb-2">No vehicles registered yet</h3>
          <p className="text-muted-foreground mb-4">
            Be the first to register a vehicle on the decentralized marketplace.
          </p>
          <Link href="/list-vehicle">
            <Button>List Your Vehicle</Button>
          </Link>
        </div>
      </div>
    );
  }

  return (
    <div className="container mx-auto px-4 py-8">
      <div className="mb-8">
        <h1 className="text-3xl font-bold tracking-tight">Vehicle Marketplace</h1>
        <p className="text-muted-foreground">
          Browse verified vehicles and access real-time data streams
        </p>
      </div>

      <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
        {/* Show placeholder cards based on total vehicle count */}
        {Array.from({ length: totalVehicles || 0 }).map((_, index) => (
          <Card key={index} className="hover:shadow-lg transition-shadow">
            <CardHeader>
              <div className="flex items-start justify-between">
                <div className="flex items-center space-x-2">
                  <Car className="h-5 w-5 text-blue-600" />
                  <CardTitle className="text-lg">Vehicle #{index + 1}</CardTitle>
                </div>
                <Badge variant="secondary">Active</Badge>
              </div>
              <CardDescription>
                Vehicle registered on the blockchain
              </CardDescription>
            </CardHeader>
            <CardContent>
              <div className="space-y-2">
                <div className="flex items-center space-x-2 text-sm text-muted-foreground">
                  <Clock className="h-4 w-4" />
                  <span>Recently updated</span>
                </div>
                <div className="pt-4">
                  <Link href={`/vehicle/${index + 1}`}>
                    <Button size="sm" className="w-full">
                      View Details
                    </Button>
                  </Link>
                </div>
              </div>
            </CardContent>
          </Card>
        ))}
      </div>

      <div className="mt-8 text-center">
        <p className="text-sm text-muted-foreground">
          Showing {totalVehicles} vehicle{totalVehicles !== 1 ? 's' : ''} from the blockchain
        </p>
      </div>
    </div>
  );
}