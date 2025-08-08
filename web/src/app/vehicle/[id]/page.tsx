"use client";

import { useParams } from "next/navigation";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { Car, MapPin, DollarSign, Calendar } from "lucide-react";
import { useAccount } from "wagmi";
import { ConnectWallet } from "@/components/connect-wallet";
import { mockVehicleListings } from "@/lib/mock-data";
import Link from "next/link";

export default function VehicleProfile() {
  const params = useParams();
  const { isConnected } = useAccount();
  const vehicleId = params.id as string;
  
  const vehicle = mockVehicleListings.find(v => v.id === vehicleId);
  
  if (!vehicle) {
    return (
      <div className="container mx-auto px-4 py-8">
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

  return (
    <div className="container mx-auto px-4 py-8">
      <div className="grid gap-8 lg:grid-cols-3">
        <div className="lg:col-span-2 space-y-6">
          <Card>
            <CardHeader>
              <div className="flex items-start justify-between">
                <div>
                  <CardTitle className="flex items-center gap-2">
                    <Car className="h-6 w-6" />
                    {vehicle.make} {vehicle.model} {vehicle.year}
                  </CardTitle>
                  <CardDescription className="mt-2">
                    Vehicle ID: {vehicle.vehicleId}
                  </CardDescription>
                </div>
                <Badge variant={vehicle.isVerified ? "default" : "secondary"}>
                  {vehicle.isVerified ? "Verified" : "Unverified"}
                </Badge>
              </div>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="grid gap-4 md:grid-cols-2">
                <div className="space-y-2">
                  <div className="flex items-center gap-2 text-sm">
                    <MapPin className="h-4 w-4 text-muted-foreground" />
                    <span>Mileage: {vehicle.mileage.toLocaleString()} miles</span>
                  </div>
                  <div className="flex items-center gap-2 text-sm">
                    <Calendar className="h-4 w-4 text-muted-foreground" />
                    <span>Last Updated: {new Date(vehicle.lastUpdate).toLocaleDateString()}</span>
                  </div>
                </div>
                <div className="space-y-2">
                  <div className="text-sm">
                    <span className="text-muted-foreground">Data Type: </span>
                    <span className="font-medium">{vehicle.dataType}</span>
                  </div>
                  <div className="text-sm">
                    <span className="text-muted-foreground">Battery Health: </span>
                    <span className="font-medium">{vehicle.batteryHealth}%</span>
                  </div>
                </div>
              </div>
              
              <div className="pt-4 border-t">
                <p className="text-sm text-muted-foreground">
                  {vehicle.description}
                </p>
              </div>
            </CardContent>
          </Card>
        </div>

        <div className="space-y-6">
          <Card>
            <CardHeader>
              <CardTitle>Purchase Data Access</CardTitle>
              <CardDescription>
                Get access to this vehicle's data stream
              </CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="flex items-center justify-between">
                <span className="text-sm text-muted-foreground">Price per hour:</span>
                <div className="flex items-center gap-1">
                  <DollarSign className="h-4 w-4 text-green-600" />
                  <span className="font-bold">{vehicle.price} ETH</span>
                </div>
              </div>
              
              {!isConnected ? (
                <ConnectWallet />
              ) : (
                <div className="space-y-2">
                  <Button className="w-full" disabled>
                    Purchase Access (Coming Soon)
                  </Button>
                  <p className="text-xs text-muted-foreground text-center">
                    Purchase functionality will be available soon
                  </p>
                </div>
              )}
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <CardTitle>Vehicle Stats</CardTitle>
            </CardHeader>
            <CardContent className="space-y-3">
              <div className="flex justify-between text-sm">
                <span className="text-muted-foreground">Status:</span>
                <Badge variant={vehicle.isVerified ? "default" : "secondary"} className="text-xs">
                  {vehicle.isVerified ? "Active" : "Inactive"}
                </Badge>
              </div>
              <div className="flex justify-between text-sm">
                <span className="text-muted-foreground">Battery Health:</span>
                <span className="font-medium">{vehicle.batteryHealth}%</span>
              </div>
            </CardContent>
          </Card>
        </div>
      </div>
    </div>
  );
}