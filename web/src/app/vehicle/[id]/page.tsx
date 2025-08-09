"use client";

import { useState } from "react";
import { useParams } from "next/navigation";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { Separator } from "@/components/ui/separator";
import { Progress } from "@/components/ui/progress";
import { 
  Car, 
  MapPin, 
  Clock, 
  DollarSign, 
  Star,
  Shield,
  Activity,
  TrendingUp,
  User,
  Calendar
} from "lucide-react";
import { useAccount } from "wagmi";
import { ConnectWallet } from "@/components/connect-wallet";
import { useVehicleWithMetadata } from "@/hooks/useMockVehicleData";
import { useVehicleContract } from "@/hooks/useVehicleContract";
import { toast } from "sonner";
import Link from "next/link";
import { VehicleTelemetry } from "@/components/VehicleTelemetry";

export default function VehicleProfile() {
  const params = useParams();
  const { isConnected } = useAccount();
  const [isPurchasing, setIsPurchasing] = useState(false);
  const [purchaseComplete, setPurchaseComplete] = useState(false);
  
  const { registerVehicle, isWritePending, isConfirming, isConfirmed } = useVehicleContract();

  const vehicleId = params?.id as string;
  const { vehicle, isLoading, error } = useVehicleWithMetadata(vehicleId);

  if (isLoading) {
    return (
      <div className="container mx-auto px-4 py-16">
        <Card className="mx-auto max-w-md text-center">
          <CardHeader>
            <CardTitle>Loading Vehicle...</CardTitle>
            <CardDescription>
              Loading vehicle information...
            </CardDescription>
          </CardHeader>
          <CardContent>
            <div className="animate-pulse">
              <Car className="mx-auto h-12 w-12 text-muted-foreground mb-4" />
            </div>
          </CardContent>
        </Card>
      </div>
    );
  }

  if (error || !vehicle) {
    return (
      <div className="container mx-auto px-4 py-16">
        <Card className="mx-auto max-w-md text-center">
          <CardHeader>
            <CardTitle>Vehicle Not Found</CardTitle>
            <CardDescription>
              {error ? `Error loading vehicle: ${error.message}` : "The vehicle you're looking for doesn't exist."}
            </CardDescription>
          </CardHeader>
          <CardContent>
            <Link href="/dashboard">
              <Button>Back to Marketplace</Button>
            </Link>
          </CardContent>
        </Card>
      </div>
    );
  }

  const handlePurchase = async (duration: number) => {
    if (!isConnected) {
      toast.error("Please connect your wallet first");
      return;
    }

    if (!vehicle) return;

    setIsPurchasing(true);
    
    try {
      // Simulate purchase process
      await new Promise(resolve => setTimeout(resolve, 2000));
      toast.success(`Successfully purchased ${duration} hour(s) of data access!`);
      setPurchaseComplete(true);
      
    } catch (error: any) {
      console.error("Purchase failed:", error);
      toast.error(error?.message || "Failed to purchase data access. Please try again.");
    } finally {
      setIsPurchasing(false);
    }
  };

  return (
    <div className="container mx-auto px-4 py-8">
      <div className="mx-auto max-w-4xl">
        {/* Header */}
        <div className="mb-8">
          <Link href="/dashboard" className="text-sm text-muted-foreground hover:text-foreground">
            ← Back to Marketplace
          </Link>
          <h1 className="mt-2 text-3xl font-bold flex items-center gap-3">
            <Car className="h-8 w-8" />
            {vehicle.manufacturer} {vehicle.model} {vehicle.year}
          </h1>
          <div className="mt-2 flex items-center gap-4 text-muted-foreground">
            <span className="flex items-center gap-1">
              <Calendar className="h-4 w-4" />
              Registered: {new Date(vehicle.registrationTimestamp * 1000).toLocaleDateString()}
            </span>
            <Badge variant={vehicle.isActive ? "default" : "secondary"}>
              {vehicle.isActive ? "Live Stream" : "Offline"}
            </Badge>
          </div>
        </div>

        <div className="grid gap-8 lg:grid-cols-3">
          {/* Main Content */}
          <div className="lg:col-span-2 space-y-6">
            {/* Vehicle Information */}
            <Card>
              <CardHeader>
                <CardTitle>Vehicle Information</CardTitle>
              </CardHeader>
              <CardContent className="space-y-4">
                <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                  <div>
                    <label className="text-sm font-medium text-muted-foreground">VIN Number</label>
                    <p className="font-mono">{vehicle.vin}</p>
                  </div>
                  <div>
                    <label className="text-sm font-medium text-muted-foreground">Vehicle Wallet</label>
                    <p className="font-mono text-xs">{vehicle.wallet.slice(0, 6)}...{vehicle.wallet.slice(-4)}</p>
                  </div>
                  <div>
                    <label className="text-sm font-medium text-muted-foreground">Data Types</label>
                    <p className="capitalize">{vehicle.dataTypes.join(", ")}</p>
                  </div>
                  <div>
                    <label className="text-sm font-medium text-muted-foreground">Last Updated</label>
                    <p>{vehicle.lastUpdate > 0 ? new Date(vehicle.lastUpdate * 1000).toLocaleDateString() : 'Never'}</p>
                  </div>
                </div>
              </CardContent>
            </Card>

            {/* IPFS Data */}
            <Card>
              <CardHeader>
                <CardTitle>Data Storage</CardTitle>
              </CardHeader>
              <CardContent>
                <div className="space-y-2">
                  <label className="text-sm font-medium text-muted-foreground">IPFS Hash</label>
                  <p className="font-mono text-sm break-all">
                    {vehicle.ipfsHash || 'No data stored'}
                  </p>
                </div>
              </CardContent>
            </Card>

            {/* Available Data Types */}
            <Card>
              <CardHeader>
                <CardTitle>Available Data Types</CardTitle>
                <CardDescription>
                  Data types registered for this vehicle
                </CardDescription>
              </CardHeader>
              <CardContent>
                <div className="grid grid-cols-1 md:grid-cols-2 gap-2">
                  {vehicle.dataTypes.map((dataType) => (
                    <div key={dataType} className="flex items-center gap-2">
                      <Activity className="h-4 w-4 text-green-500" />
                      <span className="text-sm capitalize">{dataType}</span>
                    </div>
                  ))}
                </div>
              </CardContent>
            </Card>

            {/* Seller Information */}
            <Card>
              <CardHeader>
                <CardTitle>Seller Information</CardTitle>
              </CardHeader>
              <CardContent className="space-y-4">
                <div className="flex items-center justify-between">
                  <div className="flex items-center gap-2">
                    <User className="h-4 w-4" />
                    <span className="font-mono text-sm">
                      {vehicle.owner.slice(0, 6)}...{vehicle.owner.slice(-4)}
                    </span>
                  </div>
                  <Button variant="outline" size="sm">
                    View Profile
                  </Button>
                </div>
                
                <div className="grid grid-cols-1 md:grid-cols-2 gap-4 pt-4">
                  <div className="text-center">
                    <p className="text-2xl font-bold">
                      {new Date(vehicle.registrationTimestamp * 1000).toLocaleDateString()}
                    </p>
                    <p className="text-sm text-muted-foreground">Registered</p>
                  </div>
                  <div className="text-center">
                    <p className="text-2xl font-bold text-green-600">
                      {vehicle.isActive ? "Active" : "Inactive"}
                    </p>
                    <p className="text-sm text-muted-foreground">Status</p>
                  </div>
                </div>
              </CardContent>
            </Card>

            {/* Live Telemetry */}
            {vehicle.isActive && (
              <VehicleTelemetry vehicleId={vehicleId} />
            )}
          </div>

          {/* Purchase Panel */}
          <div className="space-y-6">
            <Card className="sticky top-4">
              <CardHeader>
                <CardTitle className="flex items-center gap-2">
                  <Shield className="h-5 w-5 text-green-500" />
                  Purchase Access
                </CardTitle>
                <CardDescription>
                  Get real-time access to this vehicle's data stream
                </CardDescription>
              </CardHeader>
              <CardContent className="space-y-4">
                {!isConnected ? (
                  <div className="text-center py-4">
                    <p className="mb-4 text-sm text-muted-foreground">
                      Connect your wallet to purchase data access
                    </p>
                    <ConnectWallet />
                  </div>
                ) : purchaseComplete ? (
                  <div className="text-center py-4">
                    <div className="mb-4 p-4 bg-green-50 rounded-lg">
                      <TrendingUp className="mx-auto h-8 w-8 text-green-500 mb-2" />
                      <p className="font-medium text-green-800">Access Granted!</p>
                      <p className="text-sm text-green-600">
                        You now have access to this vehicle's data stream
                      </p>
                    </div>
                    <Button variant="outline" onClick={() => setPurchaseComplete(false)}>
                      Purchase More Access
                    </Button>
                  </div>
                ) : (
                  <>
                    <div className="space-y-4">
                      <div className="text-center p-4 bg-blue-50 rounded-lg">
                        <p className="font-medium text-blue-800">Premium Vehicle Data</p>
                        <p className="text-sm text-blue-600">
                          Real-time {vehicle.streamType} data stream
                        </p>
                      </div>
                      
                      <div className="grid grid-cols-1 gap-3">
                        <div className="p-3 border rounded-lg">
                          <div className="flex justify-between items-center">
                            <span className="font-medium">1 Hour Access</span>
                            <span className="font-bold text-lg">{vehicle.price} ETH</span>
                          </div>
                          <p className="text-sm text-muted-foreground">Perfect for testing</p>
                        </div>
                        
                        <div className="p-3 border rounded-lg border-blue-200 bg-blue-50">
                          <div className="flex justify-between items-center">
                            <span className="font-medium">24 Hour Access</span>
                            <span className="font-bold text-lg">{(parseFloat(vehicle.price) * 20).toFixed(3)} ETH</span>
                          </div>
                          <p className="text-sm text-blue-600">Most Popular - 17% savings</p>
                        </div>
                        
                        <div className="p-3 border rounded-lg">
                          <div className="flex justify-between items-center">
                            <span className="font-medium">7 Day Access</span>
                            <span className="font-bold text-lg">{(parseFloat(vehicle.price) * 120).toFixed(3)} ETH</span>
                          </div>
                          <p className="text-sm text-muted-foreground">Best value - 29% savings</p>
                        </div>
                      </div>
                    </div>

                    <Separator />

                    <div className="space-y-3">
                      <Button
                        className="w-full"
                        onClick={() => handlePurchase(1)}
                        disabled={isPurchasing}
                      >
                        {isPurchasing ? "Processing..." : "Purchase 1 Hour Access"}
                      </Button>
                      
                      <Button
                        variant="outline"
                        className="w-full"
                        onClick={() => handlePurchase(24)}
                        disabled={isPurchasing}
                      >
                        {isPurchasing ? "Processing..." : "Purchase 24 Hour Access"}
                      </Button>
                    </div>

                    <Separator />

                    <div className="space-y-2">
                      <div className="flex items-center gap-2 text-sm text-muted-foreground">
                        <Shield className="h-4 w-4" />
                        Secure payment processing
                      </div>
                      <div className="flex items-center gap-2 text-sm text-muted-foreground">
                        <Clock className="h-4 w-4" />
                        Instant access after payment
                      </div>
                      <div className="flex items-center gap-2 text-sm text-muted-foreground">
                        <TrendingUp className="h-4 w-4" />
                        Real-time data streaming
                      </div>
                    </div>

                    {!vehicle.isActive && (
                      <div className="p-3 bg-yellow-50 rounded-lg">
                        <p className="text-sm text-yellow-800">
                          ⚠️ This vehicle is currently offline. You can still purchase access, 
                          but data streaming will begin when the vehicle comes back online.
                        </p>
                      </div>
                    )}
                  </>
                )}
              </CardContent>
            </Card>

            {/* Stats Card */}
            <Card>
              <CardHeader>
                <CardTitle className="text-lg">Stream Statistics</CardTitle>
              </CardHeader>
              <CardContent className="space-y-4">
                <div className="space-y-2">
                  <div className="flex justify-between text-sm">
                    <span>Data Quality</span>
                    <span>98%</span>
                  </div>
                  <Progress value={98} className="h-2" />
                </div>
                
                <div className="space-y-2">
                  <div className="flex justify-between text-sm">
                    <span>Reliability</span>
                    <span>{vehicle.isActive ? "99%" : "0%"}</span>
                  </div>
                  <Progress value={vehicle.isActive ? 99 : 0} className="h-2" />
                </div>
                
                <div className="space-y-2">
                  <div className="flex justify-between text-sm">
                    <span>Update Frequency</span>
                    <span>Excellent</span>
                  </div>
                  <Progress value={95} className="h-2" />
                </div>
              </CardContent>
            </Card>
          </div>
        </div>
      </div>
    </div>
  );
}