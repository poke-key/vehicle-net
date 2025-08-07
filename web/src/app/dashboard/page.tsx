"use client";

import { useState } from "react";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Badge } from "@/components/ui/badge";
import { Search, Filter, Car, MapPin, Clock, DollarSign } from "lucide-react";
import Link from "next/link";
import { mockVehicleListings } from "@/lib/mock-data";

export default function Dashboard() {
  const [searchTerm, setSearchTerm] = useState("");
  const [selectedType, setSelectedType] = useState<string>("all");

  const filteredListings = mockVehicleListings.filter(vehicle => {
    const matchesSearch = vehicle.title.toLowerCase().includes(searchTerm.toLowerCase()) ||
                         vehicle.vin.toLowerCase().includes(searchTerm.toLowerCase());
    const matchesType = selectedType === "all" || vehicle.streamType === selectedType;
    return matchesSearch && matchesType;
  });

  const streamTypes = ["all", "gps", "diagnostics", "telemetry", "sensor"];

  return (
    <div className="container mx-auto px-4 py-8">
      <div className="mb-8">
        <h1 className="mb-2 text-3xl font-bold">Vehicle Data Marketplace</h1>
        <p className="text-muted-foreground">
          Discover and purchase access to real-time vehicle data streams
        </p>
      </div>

      {/* Search and Filter */}
      <div className="mb-6 flex flex-col gap-4 md:flex-row">
        <div className="relative flex-1">
          <Search className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
          <Input
            placeholder="Search by vehicle title or VIN..."
            value={searchTerm}
            onChange={(e) => setSearchTerm(e.target.value)}
            className="pl-10"
          />
        </div>
        
        <div className="flex gap-2">
          {streamTypes.map((type) => (
            <Button
              key={type}
              variant={selectedType === type ? "default" : "outline"}
              size="sm"
              onClick={() => setSelectedType(type)}
            >
              {type.charAt(0).toUpperCase() + type.slice(1)}
            </Button>
          ))}
        </div>
      </div>

      {/* Vehicle Listings Grid */}
      <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
        {filteredListings.map((vehicle) => (
          <Card key={vehicle.id} className="hover:shadow-md transition-shadow">
            <CardHeader>
              <div className="flex items-start justify-between">
                <div>
                  <CardTitle className="flex items-center gap-2">
                    <Car className="h-5 w-5" />
                    {vehicle.title}
                  </CardTitle>
                  <p className="text-sm text-muted-foreground mt-1">
                    VIN: {vehicle.vin}
                  </p>
                </div>
                <Badge variant={vehicle.isActive ? "default" : "secondary"}>
                  {vehicle.isActive ? "Live" : "Offline"}
                </Badge>
              </div>
            </CardHeader>
            
            <CardContent className="space-y-4">
              <div className="flex items-center gap-2 text-sm text-muted-foreground">
                <MapPin className="h-4 w-4" />
                {vehicle.location}
              </div>
              
              <div className="flex items-center gap-2 text-sm">
                <Clock className="h-4 w-4 text-muted-foreground" />
                <span className="capitalize">{vehicle.streamType} Data</span>
                <Badge variant="outline" className="ml-auto">
                  {vehicle.frequency}
                </Badge>
              </div>
              
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-1">
                  <DollarSign className="h-4 w-4 text-green-600" />
                  <span className="font-semibold text-lg">
                    {vehicle.price} ETH
                  </span>
                  <span className="text-sm text-muted-foreground">
                    /{vehicle.billingPeriod}
                  </span>
                </div>
                
                <Link href={`/vehicle/${vehicle.id}`}>
                  <Button size="sm">
                    View Details
                  </Button>
                </Link>
              </div>
              
              <div className="text-xs text-muted-foreground">
                Owner: {vehicle.owner.slice(0, 6)}...{vehicle.owner.slice(-4)}
              </div>
            </CardContent>
          </Card>
        ))}
      </div>

      {filteredListings.length === 0 && (
        <div className="text-center py-12">
          <Car className="mx-auto h-12 w-12 text-muted-foreground mb-4" />
          <h3 className="text-lg font-semibold mb-2">No vehicles found</h3>
          <p className="text-muted-foreground">
            Try adjusting your search criteria or browse all available listings.
          </p>
        </div>
      )}

      <div className="mt-8 text-center">
        <Link href="/list-vehicle">
          <Button size="lg">
            List Your Vehicle Data
          </Button>
        </Link>
      </div>
    </div>
  );
}