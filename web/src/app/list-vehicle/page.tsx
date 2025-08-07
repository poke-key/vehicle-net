"use client";

import { useState } from "react";
import { useForm } from "@tanstack/react-form";
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
import { ConnectWallet } from "@/components/connect-wallet";
import { useVehicleContract } from "@/hooks/useVehicleContract";
import { Car, AlertCircle, CheckCircle } from "lucide-react";
import { toast } from "sonner";

interface ListVehicleForm {
  title: string;
  vin: string;
  location: string;
  streamType: string;
  price: string;
  billingPeriod: string;
  frequency: string;
  description: string;
  features: string[];
  terms: boolean;
}

export default function ListVehicle() {
  const { isConnected } = useAccount();
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [submissionSuccess, setSubmissionSuccess] = useState(false);
  
  const { listVehicleData, isWritePending, isConfirming, isConfirmed } = useVehicleContract();

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

  const form = useForm<ListVehicleForm>({
    defaultValues: {
      title: '',
      vin: '',
      location: '',
      streamType: 'gps',
      price: '',
      billingPeriod: 'hour',
      frequency: '',
      description: '',
      features: [],
      terms: false,
    },
    onSubmit: async ({ value }) => {
      if (!isConnected) {
        toast.error("Please connect your wallet first");
        return;
      }

      setIsSubmitting(true);
      
      try {
        // Use VIN as unique vehicle ID
        const vehicleId = value.vin;
        const priceInEth = parseFloat(value.price);
        
        await listVehicleData(vehicleId, priceInEth, value.streamType);
        
        toast.success("Transaction submitted! Your vehicle will be listed once confirmed.");
        
        // Wait for confirmation
        if (isConfirmed) {
          console.log("Listing vehicle:", value);
          toast.success("Vehicle data listed successfully on the blockchain!");
          setSubmissionSuccess(true);
          
          // Reset form after successful submission
          setTimeout(() => {
            setSubmissionSuccess(false);
            form.reset();
          }, 3000);
        }
        
      } catch (error: any) {
        console.error("Listing failed:", error);
        toast.error(error?.message || "Failed to list vehicle data. Please try again.");
      } finally {
        setIsSubmitting(false);
      }
    },
  });

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

        <form
          onSubmit={(e) => {
            e.preventDefault();
            e.stopPropagation();
            form.handleSubmit();
          }}
          className="space-y-6"
        >
          <Card>
            <CardHeader>
              <CardTitle>Vehicle Information</CardTitle>
              <CardDescription>
                Basic information about your vehicle
              </CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              <form.Field
                name="title"
                validators={{
                  onChange: ({ value }) =>
                    !value ? 'Title is required' : undefined,
                }}
              >
                {(field) => (
                  <div className="space-y-2">
                    <Label htmlFor="title">Vehicle Title *</Label>
                    <Input
                      id="title"
                      placeholder="e.g., 2023 Tesla Model S GPS Stream"
                      value={field.state.value}
                      onChange={(e) => field.handleChange(e.target.value)}
                    />
                    {field.state.meta.errors.length > 0 && (
                      <p className="text-sm text-red-600">{field.state.meta.errors[0]}</p>
                    )}
                  </div>
                )}
              </form.Field>

              <form.Field
                name="vin"
                validators={{
                  onChange: ({ value }) =>
                    !value ? 'VIN is required' : value.length !== 17 ? 'VIN must be 17 characters' : undefined,
                }}
              >
                {(field) => (
                  <div className="space-y-2">
                    <Label htmlFor="vin">VIN Number *</Label>
                    <Input
                      id="vin"
                      placeholder="17-character VIN"
                      maxLength={17}
                      value={field.state.value}
                      onChange={(e) => field.handleChange(e.target.value.toUpperCase())}
                    />
                    {field.state.meta.errors.length > 0 && (
                      <p className="text-sm text-red-600">{field.state.meta.errors[0]}</p>
                    )}
                  </div>
                )}
              </form.Field>

              <form.Field
                name="location"
                validators={{
                  onChange: ({ value }) =>
                    !value ? 'Location is required' : undefined,
                }}
              >
                {(field) => (
                  <div className="space-y-2">
                    <Label htmlFor="location">Location *</Label>
                    <Input
                      id="location"
                      placeholder="e.g., San Francisco, CA"
                      value={field.state.value}
                      onChange={(e) => field.handleChange(e.target.value)}
                    />
                    {field.state.meta.errors.length > 0 && (
                      <p className="text-sm text-red-600">{field.state.meta.errors[0]}</p>
                    )}
                  </div>
                )}
              </form.Field>
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
              <form.Field name="streamType">
                {(field) => (
                  <div className="space-y-2">
                    <Label htmlFor="streamType">Data Stream Type *</Label>
                    <Select value={field.state.value} onValueChange={field.handleChange}>
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
                )}
              </form.Field>

              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                <form.Field
                  name="price"
                  validators={{
                    onChange: ({ value }) => {
                      if (!value) return 'Price is required';
                      if (isNaN(Number(value)) || Number(value) <= 0) return 'Price must be a positive number';
                      return undefined;
                    },
                  }}
                >
                  {(field) => (
                    <div className="space-y-2">
                      <Label htmlFor="price">Price (ETH) *</Label>
                      <Input
                        id="price"
                        type="number"
                        step="0.0001"
                        placeholder="0.001"
                        value={field.state.value}
                        onChange={(e) => field.handleChange(e.target.value)}
                      />
                      {field.state.meta.errors.length > 0 && (
                        <p className="text-sm text-red-600">{field.state.meta.errors[0]}</p>
                      )}
                    </div>
                  )}
                </form.Field>

                <form.Field name="billingPeriod">
                  {(field) => (
                    <div className="space-y-2">
                      <Label htmlFor="billingPeriod">Billing Period *</Label>
                      <Select value={field.state.value} onValueChange={field.handleChange}>
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
                  )}
                </form.Field>
              </div>

              <form.Field
                name="frequency"
                validators={{
                  onChange: ({ value }) =>
                    !value ? 'Update frequency is required' : undefined,
                }}
              >
                {(field) => (
                  <div className="space-y-2">
                    <Label htmlFor="frequency">Update Frequency *</Label>
                    <Input
                      id="frequency"
                      placeholder="e.g., Every 10 seconds, Every 2 minutes"
                      value={field.state.value}
                      onChange={(e) => field.handleChange(e.target.value)}
                    />
                    {field.state.meta.errors.length > 0 && (
                      <p className="text-sm text-red-600">{field.state.meta.errors[0]}</p>
                    )}
                  </div>
                )}
              </form.Field>
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
              <form.Field name="features">
                {(field) => (
                  <div className="space-y-2">
                    <Label>Available Features</Label>
                    <div className="grid grid-cols-2 gap-2">
                      {availableFeatures.map((feature) => (
                        <div key={feature} className="flex items-center space-x-2">
                          <Checkbox
                            id={feature}
                            checked={field.state.value.includes(feature)}
                            onCheckedChange={(checked) => {
                              if (checked) {
                                field.handleChange([...field.state.value, feature]);
                              } else {
                                field.handleChange(field.state.value.filter((f: string) => f !== feature));
                              }
                            }}
                          />
                          <Label htmlFor={feature} className="text-sm">
                            {feature}
                          </Label>
                        </div>
                      ))}
                    </div>
                    <div className="flex flex-wrap gap-2 mt-2">
                      {field.state.value.map((feature: string) => (
                        <Badge key={feature} variant="secondary">
                          {feature}
                        </Badge>
                      ))}
                    </div>
                  </div>
                )}
              </form.Field>

              <form.Field
                name="description"
                validators={{
                  onChange: ({ value }) =>
                    !value ? 'Description is required' : value.length < 50 ? 'Description must be at least 50 characters' : undefined,
                }}
              >
                {(field) => (
                  <div className="space-y-2">
                    <Label htmlFor="description">Description *</Label>
                    <Textarea
                      id="description"
                      placeholder="Describe your vehicle data stream in detail. What makes it valuable? What can buyers expect?"
                      rows={4}
                      value={field.state.value}
                      onChange={(e) => field.handleChange(e.target.value)}
                    />
                    <p className="text-xs text-muted-foreground">
                      {field.state.value.length}/50 characters minimum
                    </p>
                    {field.state.meta.errors.length > 0 && (
                      <p className="text-sm text-red-600">{field.state.meta.errors[0]}</p>
                    )}
                  </div>
                )}
              </form.Field>
            </CardContent>
          </Card>

          <Card>
            <CardContent className="pt-6">
              <form.Field
                name="terms"
                validators={{
                  onChange: ({ value }) =>
                    !value ? 'You must accept the terms and conditions' : undefined,
                }}
              >
                {(field) => (
                  <div className="flex items-center space-x-2">
                    <Checkbox
                      id="terms"
                      checked={field.state.value}
                      onCheckedChange={field.handleChange}
                    />
                    <Label htmlFor="terms" className="text-sm">
                      I agree to the terms and conditions and confirm that I have the right to share this vehicle data *
                    </Label>
                    {field.state.meta.errors.length > 0 && (
                      <p className="text-sm text-red-600 ml-6">{field.state.meta.errors[0]}</p>
                    )}
                  </div>
                )}
              </form.Field>

              <Button
                type="submit"
                className="w-full mt-6"
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