"use client";

import { useState } from "react";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { 
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Textarea } from "@/components/ui/textarea";
import { Checkbox } from "@/components/ui/checkbox";
import { Badge } from "@/components/ui/badge";
import { useAccount } from "wagmi";
import { parseEther } from "viem";
import { ConnectWallet } from "@/components/connect-wallet";
import { useVehicleContract } from "@/hooks/useVehicleContract";
import { Car, AlertCircle, CheckCircle } from "lucide-react";
import { toast } from "sonner";

export default function ListVehicle() {
  const { isConnected } = useAccount();
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [submissionSuccess, setSubmissionSuccess] = useState(false);
  
  const { listDataProduct, isWritePending, isConfirming, isConfirmed } = useVehicleContract();

  const availableFeatures = [
    "Real-time location",
    "Speed data", 
    "Direction heading",
    "Engine diagnostics",
    "Fuel consumption",
    "Battery health",
    "Tire pressure",
    "Temperature sensors",
    "Brake system data",
    "Transmission data"
  ];

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    
    if (!isConnected) {
      toast.error("Please connect your wallet first");
      return;
    }

    setIsSubmitting(true);
    
    try {
      // For now, we'll use a placeholder vehicle ID of 1
      const vehicleId = 1;
      
      await listDataProduct(
        vehicleId,
        "gps",
        1000000000000000, // 0.001 ETH in wei
        3600, // 1 hour minimum
        2592000, // 30 days maximum
        "Sample vehicle data stream",
        "https://api.example.com/vehicle/data"
      );
      
      toast.success("Transaction submitted! Your vehicle will be listed once confirmed.");
      
      // Wait for confirmation
      if (isConfirmed) {
        toast.success("Vehicle data listed successfully on the blockchain!");
        setSubmissionSuccess(true);
        
        // Reset after successful submission
        setTimeout(() => {
          setSubmissionSuccess(false);
        }, 3000);
      }
      
    } catch (error: any) {
      console.error("Listing failed:", error);
      toast.error(error?.message || "Failed to list vehicle data. Please try again.");
    } finally {
      setIsSubmitting(false);
    }
  };

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

  if (submissionSuccess) {
    return (
      <div className="container mx-auto px-4 py-16">
        <Card className="mx-auto max-w-md text-center">
          <CardHeader>
            <CheckCircle className="mx-auto h-12 w-12 text-green-500 mb-4" />
            <CardTitle>Successfully Listed!</CardTitle>
            <CardDescription>
              Your vehicle data has been added to the marketplace and is now available for purchase.
            </CardDescription>
          </CardHeader>
          <CardContent>
            <Button onClick={() => window.location.href = '/dashboard'}>
              View Marketplace
            </Button>
          </CardContent>
        </Card>
      </div>
    );
  }

  return (
    <div className="container mx-auto px-4 py-8">
      <div className="mx-auto max-w-2xl">
        <div className="mb-8">
          <h1 className="mb-2 text-3xl font-bold flex items-center gap-2">
            <Car className="h-8 w-8" />
            List Your Vehicle Data
          </h1>
          <p className="text-muted-foreground">
            Share your vehicle's data streams and earn crypto from interested buyers
          </p>
        </div>

        <form onSubmit={handleSubmit} className="space-y-6">
          <Card>
            <CardHeader>
              <CardTitle>Vehicle Information</CardTitle>
              <CardDescription>
                Basic information about your vehicle
              </CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="space-y-2">
                <Label htmlFor="title">Vehicle Title *</Label>
                <Input
                  id="title"
                  placeholder="e.g., 2023 Tesla Model S GPS Stream"
                  defaultValue=""
                />
              </div>

              <div className="space-y-2">
                <Label htmlFor="vin">VIN Number *</Label>
                <Input
                  id="vin"
                  placeholder="17-character VIN"
                  maxLength={17}
                  defaultValue=""
                />
              </div>

              <div className="space-y-2">
                <Label htmlFor="location">Location *</Label>
                <Input
                  id="location"
                  placeholder="e.g., San Francisco, CA"
                  defaultValue=""
                />
              </div>
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <CardTitle>Data Stream Configuration</CardTitle>
              <CardDescription>
                Configure what data you want to share and how often
              </CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="space-y-2">
                <Label htmlFor="streamType">Data Stream Type *</Label>
                <Select defaultValue="gps">
                  <SelectTrigger>
                    <SelectValue placeholder="Select stream type" />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="gps">GPS & Location</SelectItem>
                    <SelectItem value="diagnostics">Vehicle Diagnostics</SelectItem>
                    <SelectItem value="telemetry">Telemetry Data</SelectItem>
                    <SelectItem value="sensor">Sensor Array</SelectItem>
                  </SelectContent>
                </Select>
              </div>

              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div className="space-y-2">
                  <Label htmlFor="price">Price (ETH) *</Label>
                  <Input
                    id="price"
                    type="number"
                    step="0.0001"
                    placeholder="0.001"
                    defaultValue="0.001"
                  />
                </div>

                <div className="space-y-2">
                  <Label htmlFor="billingPeriod">Billing Period *</Label>
                  <Select defaultValue="hour">
                    <SelectTrigger>
                      <SelectValue />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value="hour">Per Hour</SelectItem>
                      <SelectItem value="day">Per Day</SelectItem>
                      <SelectItem value="week">Per Week</SelectItem>
                      <SelectItem value="month">Per Month</SelectItem>
                    </SelectContent>
                  </Select>
                </div>
              </div>

              <div className="space-y-2">
                <Label htmlFor="frequency">Update Frequency *</Label>
                <Input
                  id="frequency"
                  placeholder="e.g., Every 10 seconds, Every 2 minutes"
                  defaultValue="Every 30 seconds"
                />
              </div>
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <CardTitle>Features & Description</CardTitle>
              <CardDescription>
                Help buyers understand what data you're providing
              </CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="space-y-2">
                <Label>Available Features</Label>
                <div className="grid grid-cols-2 gap-2">
                  {availableFeatures.map((feature) => (
                    <div key={feature} className="flex items-center space-x-2">
                      <Checkbox id={feature} />
                      <Label htmlFor={feature} className="text-sm">
                        {feature}
                      </Label>
                    </div>
                  ))}
                </div>
                <div className="flex flex-wrap gap-2 mt-2">
                  <Badge variant="secondary">Real-time location</Badge>
                  <Badge variant="secondary">Speed data</Badge>
                </div>
              </div>

              <div className="space-y-2">
                <Label htmlFor="description">Description *</Label>
                <Textarea
                  id="description"
                  placeholder="Describe your vehicle data stream in detail. What makes it valuable? What can buyers expect?"
                  rows={4}
                  defaultValue="High-quality GPS and telemetry data from a modern electric vehicle. Real-time location tracking, speed monitoring, and battery status information. Perfect for fleet management, insurance analytics, or research purposes."
                />
                <p className="text-xs text-muted-foreground">
                  150/50 characters minimum
                </p>
              </div>
            </CardContent>
          </Card>

          <Card>
            <CardContent className="pt-6">
              <div className="flex items-center space-x-2 mb-6">
                <Checkbox id="terms" defaultChecked />
                <Label htmlFor="terms" className="text-sm">
                  I agree to the terms and conditions and confirm that I have the right to share this vehicle data *
                </Label>
              </div>

              <Button
                type="submit"
                className="w-full"
                disabled={isSubmitting || isWritePending || isConfirming}
                size="lg"
              >
                {isWritePending 
                  ? "Confirming Transaction..." 
                  : isConfirming 
                    ? "Waiting for Confirmation..." 
                    : isSubmitting 
                      ? "Listing Vehicle..." 
                      : "List Vehicle Data"}
              </Button>
            </CardContent>
          </Card>
        </form>
      </div>
    </div>
  );
}