"use client";

import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { ConnectWallet } from "@/components/connect-wallet";
import { useAccount } from "wagmi";
import { Car, AlertCircle } from "lucide-react";

export default function ListVehiclePage() {
  const { isConnected } = useAccount();

  if (!isConnected) {
    return (
      <div className="container mx-auto px-4 py-16">
        <Card className="mx-auto max-w-md text-center">
          <CardHeader>
            <AlertCircle className="mx-auto h-12 w-12 text-yellow-500 mb-4" />
            <CardTitle>Wallet Connection Required</CardTitle>
            <CardDescription>
              You need to connect your wallet to list vehicle data on the marketplace.
            </CardDescription>
          </CardHeader>
          <CardContent>
            <ConnectWallet />
          </CardContent>
        </Card>
      </div>
    );
  }

  return (
    <div className="container mx-auto px-4 py-8">
      <div className="mb-8">
        <h1 className="mb-2 text-3xl font-bold">List Your Vehicle Data</h1>
        <p className="text-muted-foreground">
          Create a data stream listing for your vehicle on the marketplace.
        </p>
      </div>

      <Card className="mx-auto max-w-2xl">
        <CardHeader>
          <div className="flex items-center gap-2">
            <Car className="h-6 w-6" />
            <CardTitle>Vehicle Data Listing</CardTitle>
          </div>
          <CardDescription>
            This feature is coming soon. You'll be able to list your vehicle data streams for sale.
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div className="text-center py-8">
            <Car className="mx-auto h-16 w-16 text-muted-foreground mb-4" />
            <h3 className="text-lg font-semibold mb-2">Feature Coming Soon</h3>
            <p className="text-muted-foreground mb-6">
              We're working on the vehicle listing functionality. Check back soon!
            </p>
            <Button variant="outline" disabled>
              List Vehicle Data
            </Button>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}