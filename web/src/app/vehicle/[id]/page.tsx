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
import { mockVehicleListings } from "@/lib/mock-data";
import { useVehicleContract, useReadVehicleData } from "@/hooks/useVehicleContract";
import { toast } from "sonner";
import Link from "next/link";

export default function VehicleProfile() {
  const params = useParams();
  const { isConnected } = useAccount();
  const [isPurchasing, setIsPurchasing] = useState(false);
  const [purchaseComplete, setPurchaseComplete] = useState(false);
  
  const { purchaseDataAccess, isWritePending, isConfirming, isConfirmed } = useVehicleContract();

  const vehicleId = params?.id as string;
  const vehicle = mockVehicleListings.find(v => v.id === vehicleId);

  if (!vehicle) {
    return (
      <div className="container mx-auto px-4 py-16">
        <Card className="mx-auto max-w-md text-center">
          <CardHeader>
            <CardTitle>Vehicle Not Found</CardTitle>
            <CardDescription>
              The vehicle you're looking for doesn't exist or has been removed.
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
      const totalPrice = parseFloat(vehicle.price) * duration;
      
      await purchaseDataAccess(vehicleId, duration, totalPrice);
      
      toast.success(`Transaction submitted! ${duration} ${vehicle.billingPeriod}(s) of access will be granted once confirmed.`);
      
      // Wait for confirmation
      if (isConfirmed) {
        setPurchaseComplete(true);
        toast.success("Purchase confirmed! You now have access to the vehicle data.");
      }
      
    } catch (error: any) {
      console.error("Purchase failed:", error);
      toast.error(error?.message || "Failed to purchase data access. Please try again.");
    } finally {
      setIsPurchasing(false);
    }
  };

  const calculatePrice = (duration: number) => {
    return (parseFloat(vehicle.price) * duration).toFixed(4);
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
            {vehicle.title}
          </h1>
          <div className="mt-2 flex items-center gap-4 text-muted-foreground">
            <span className="flex items-center gap-1">
              <MapPin className="h-4 w-4" />
              {vehicle.location}
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
                    <label className="text-sm font-medium text-muted-foreground">Stream Type</label>
                    <p className="capitalize">{vehicle.streamType}</p>
                  </div>
                  <div>
                    <label className="text-sm font-medium text-muted-foreground">Update Frequency</label>
                    <p>{vehicle.frequency}</p>
                  </div>
                  <div>
                    <label className="text-sm font-medium text-muted-foreground">Last Updated</label>
                    <p>{vehicle.lastUpdated ? new Date(vehicle.lastUpdated).toLocaleDateString() : 'N/A'}</p>
                  </div>
                </div>
              </CardContent>
            </Card>

            {/* Description */}
            <Card>
              <CardHeader>
                <CardTitle>Description</CardTitle>
              </CardHeader>
              <CardContent>
                <p className="text-muted-foreground leading-relaxed">
                  {vehicle.description}
                </p>
              </CardContent>
            </Card>

            {/* Available Features */}
            <Card>
              <CardHeader>
                <CardTitle>Available Features</CardTitle>
                <CardDescription>
                  Data points included in this stream
                </CardDescription>
              </CardHeader>
              <CardContent>
                <div className="grid grid-cols-1 md:grid-cols-2 gap-2">
                  {vehicle.features?.map((feature) => (
                    <div key={feature} className="flex items-center gap-2">
                      <Activity className="h-4 w-4 text-green-500" />
                      <span className="text-sm">{feature}</span>
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
                
                <div className="grid grid-cols-1 md:grid-cols-3 gap-4 pt-4">
                  <div className="text-center">
                    <p className="text-2xl font-bold">{vehicle.totalSales}</p>
                    <p className="text-sm text-muted-foreground">Total Sales</p>
                  </div>
                  <div className="text-center">
                    <div className="flex items-center justify-center gap-1">
                      <Star className="h-4 w-4 fill-yellow-400 text-yellow-400" />
                      <span className="text-2xl font-bold">{vehicle.rating}</span>
                    </div>
                    <p className="text-sm text-muted-foreground">Rating</p>
                  </div>
                  <div className="text-center">
                    <p className="text-2xl font-bold text-green-600">
                      {vehicle.isActive ? "99%" : "0%"}
                    </p>
                    <p className="text-sm text-muted-foreground">Uptime</p>
                  </div>
                </div>
              </CardContent>
            </Card>
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
                    <div className="space-y-2">
                      <div className="flex items-center justify-between">
                        <span className="text-sm">Base Price</span>
                        <span className="font-semibold">
                          {vehicle.price} ETH / {vehicle.billingPeriod}
                        </span>
                      </div>
                    </div>

                    <Separator />

                    <div className="space-y-3">
                      <h4 className="font-medium">Quick Purchase Options</h4>
                      
                      {[1, 24, 168].map((duration) => {
                        const label = duration === 1 
                          ? `1 ${vehicle.billingPeriod}` 
                          : duration === 24 
                            ? vehicle.billingPeriod === 'hour' ? '1 Day' : `24 ${vehicle.billingPeriod}s`
                            : vehicle.billingPeriod === 'hour' ? '1 Week' : `168 ${vehicle.billingPeriod}s`;
                            
                        return (
                          <Button
                            key={duration}
                            variant="outline"
                            className="w-full justify-between"
                            onClick={() => handlePurchase(duration)}
                            disabled={isPurchasing || isWritePending || isConfirming || !vehicle.isActive}
                          >
                            <span>{label}</span>
                            <span className="font-semibold">
                              {calculatePrice(duration)} ETH
                            </span>
                          </Button>
                        );
                      })}
                    </div>

                    <Separator />

                    <div className="space-y-2">
                      <div className="flex items-center gap-2 text-sm text-muted-foreground">
                        <Shield className="h-4 w-4" />
                        Secure blockchain payment
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