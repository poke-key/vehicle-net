'use client';

import React, { useState } from 'react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from './ui/card';
import { Button } from './ui/button';
import { Badge } from './ui/badge';
import { Input } from './ui/input';
import { Progress } from './ui/progress';
import { useAllVehicles, type VehicleWithMetadata } from '@/hooks/useMockVehicleData';
import { 
  Car, 
  MapPin, 
  Clock, 
  DollarSign, 
  Star, 
  Search,
  Filter,
  Zap,
  Shield,
  TrendingUp
} from 'lucide-react';
import Link from 'next/link';

export const MockMarketplace: React.FC = () => {
  const { vehicles, isLoading, error, refetch } = useAllVehicles();
  const [searchTerm, setSearchTerm] = useState('');
  const [selectedType, setSelectedType] = useState<string>('all');

  const filteredVehicles = vehicles.filter(vehicle => {
    const matchesSearch = 
      vehicle.title.toLowerCase().includes(searchTerm.toLowerCase()) ||
      vehicle.manufacturer.toLowerCase().includes(searchTerm.toLowerCase()) ||
      vehicle.model.toLowerCase().includes(searchTerm.toLowerCase()) ||
      vehicle.location.toLowerCase().includes(searchTerm.toLowerCase());
    
    const matchesType = selectedType === 'all' || vehicle.streamType === selectedType;
    
    return matchesSearch && matchesType;
  });

  const streamTypes = ['all', ...Array.from(new Set(vehicles.map(v => v.streamType)))];

  if (isLoading) {
    return (
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        <div className="flex items-center justify-center min-h-[400px]">
          <div className="w-8 h-8 border-2 border-blue-600 border-t-transparent rounded-full animate-spin" />
          <span className="ml-2 text-gray-600">Loading marketplace...</span>
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        <div className="bg-red-50 border border-red-200 rounded-md p-4">
          <p className="text-red-600">Error loading marketplace: {error.message}</p>
          <Button onClick={refetch} className="mt-2" variant="outline" size="sm">
            Retry
          </Button>
        </div>
      </div>
    );
  }

  return (
    <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
      {/* Header */}
      <div className="mb-8">
        <div className="flex items-center justify-between">
          <div>
            <h2 className="text-3xl font-bold text-gray-900 mb-2">
              Vehicle Data Marketplace
            </h2>
            <p className="text-gray-600">
              Discover and purchase real-time vehicle data streams ({vehicles.length} vehicles available)
            </p>
          </div>
          <Button onClick={refetch} variant="outline" size="sm">
            Refresh
          </Button>
        </div>
      </div>

      {/* Search and Filters */}
      <div className="mb-6 flex flex-col gap-4 md:flex-row">
        <div className="relative flex-1">
          <Search className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
          <Input
            placeholder="Search vehicles by make, model, or location..."
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
              className="capitalize"
            >
              {type}
            </Button>
          ))}
        </div>
      </div>

      {/* Marketplace Statistics */}
      <div className="mb-8 grid gap-4 md:grid-cols-4">
        <Card>
          <CardContent className="p-4">
            <div className="flex items-center space-x-2">
              <Car className="h-8 w-8 text-blue-600" />
              <div>
                <p className="text-sm font-medium text-muted-foreground">Total Vehicles</p>
                <p className="text-2xl font-bold">{vehicles.length}</p>
              </div>
            </div>
          </CardContent>
        </Card>
        
        <Card>
          <CardContent className="p-4">
            <div className="flex items-center space-x-2">
              <Zap className="h-8 w-8 text-green-600" />
              <div>
                <p className="text-sm font-medium text-muted-foreground">Active Streams</p>
                <p className="text-2xl font-bold">{vehicles.filter(v => v.isActive).length}</p>
              </div>
            </div>
          </CardContent>
        </Card>
        
        <Card>
          <CardContent className="p-4">
            <div className="flex items-center space-x-2">
              <TrendingUp className="h-8 w-8 text-purple-600" />
              <div>
                <p className="text-sm font-medium text-muted-foreground">Total Sales</p>
                <p className="text-2xl font-bold">{vehicles.reduce((acc, v) => acc + (v.totalSales || 0), 0)}</p>
              </div>
            </div>
          </CardContent>
        </Card>
        
        <Card>
          <CardContent className="p-4">
            <div className="flex items-center space-x-2">
              <Star className="h-8 w-8 text-yellow-600" />
              <div>
                <p className="text-sm font-medium text-muted-foreground">Avg Rating</p>
                <p className="text-2xl font-bold">
                  {(vehicles.reduce((acc, v) => acc + (v.rating || 0), 0) / vehicles.length).toFixed(1)}
                </p>
              </div>
            </div>
          </CardContent>
        </Card>
      </div>

      {/* Vehicle Listings */}
      {filteredVehicles.length === 0 ? (
        <div className="text-center py-12">
          <Car className="mx-auto h-12 w-12 text-muted-foreground mb-4" />
          <h3 className="text-lg font-medium text-gray-900 mb-2">No vehicles found</h3>
          <p className="text-gray-500 mb-4">
            Try adjusting your search criteria or browse all available listings.
          </p>
        </div>
      ) : (
        <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
          {filteredVehicles.map((vehicle) => (
            <Card key={vehicle.id} className="hover:shadow-lg transition-all duration-200">
              <CardHeader>
                <div className="flex justify-between items-start mb-2">
                  <div>
                    <CardTitle className="flex items-center gap-2">
                      <Car className="h-5 w-5" />
                      {vehicle.title}
                    </CardTitle>
                    <CardDescription className="flex items-center gap-1 mt-1">
                      <MapPin className="h-3 w-3" />
                      {vehicle.location}
                    </CardDescription>
                  </div>
                  <Badge variant={vehicle.isActive ? "default" : "secondary"}>
                    {vehicle.isActive ? "Live" : "Offline"}
                  </Badge>
                </div>
                {vehicle.verified && (
                  <Badge variant="outline" className="w-fit">
                    <Shield className="h-3 w-3 mr-1" />
                    Verified
                  </Badge>
                )}
              </CardHeader>
              
              <CardContent className="space-y-4">
                <div className="space-y-2">
                  <div className="flex justify-between text-sm">
                    <span className="text-muted-foreground">Price per hour</span>
                    <span className="font-medium">{vehicle.price} ETH</span>
                  </div>
                  
                  <div className="flex justify-between text-sm">
                    <span className="text-muted-foreground">Data Type</span>
                    <Badge variant="outline" className="text-xs capitalize">
                      {vehicle.streamType}
                    </Badge>
                  </div>
                  
                  <div className="flex justify-between text-sm">
                    <span className="text-muted-foreground">Update Frequency</span>
                    <span className="font-medium">{vehicle.frequency}</span>
                  </div>
                  
                  {vehicle.rating && (
                    <div className="flex justify-between text-sm">
                      <span className="text-muted-foreground">Rating</span>
                      <div className="flex items-center gap-1">
                        <Star className="h-3 w-3 fill-yellow-400 text-yellow-400" />
                        <span className="font-medium">{vehicle.rating}</span>
                      </div>
                    </div>
                  )}
                  
                  {vehicle.batteryHealth && (
                    <div className="space-y-1">
                      <div className="flex justify-between text-sm">
                        <span className="text-muted-foreground">Battery Health</span>
                        <span className="font-medium">{vehicle.batteryHealth}%</span>
                      </div>
                      <Progress value={vehicle.batteryHealth} className="h-2" />
                    </div>
                  )}
                </div>

                {vehicle.features && vehicle.features.length > 0 && (
                  <div>
                    <p className="text-sm font-medium text-muted-foreground mb-2">Features:</p>
                    <div className="flex flex-wrap gap-1">
                      {vehicle.features.slice(0, 2).map((feature, index) => (
                        <Badge key={index} variant="outline" className="text-xs">
                          {feature}
                        </Badge>
                      ))}
                      {vehicle.features.length > 2 && (
                        <Badge variant="outline" className="text-xs">
                          +{vehicle.features.length - 2} more
                        </Badge>
                      )}
                    </div>
                  </div>
                )}

                <div className="pt-4 border-t">
                  <div className="flex items-center justify-between">
                    <div className="text-xs text-muted-foreground">
                      {vehicle.totalSales ? `${vehicle.totalSales} sales` : 'New listing'}
                    </div>
                    <Link href={`/vehicle/${vehicle.id}`}>
                      <Button size="sm">
                        View Details
                      </Button>
                    </Link>
                  </div>
                </div>
              </CardContent>
            </Card>
          ))}
        </div>
      )}

      {/* Call to Action */}
      <div className="mt-12 text-center">
        <Card className="mx-auto max-w-2xl">
          <CardHeader>
            <CardTitle>Ready to List Your Vehicle?</CardTitle>
            <CardDescription>
              Start earning from your vehicle's data today
            </CardDescription>
          </CardHeader>
          <CardContent>
            <Link href="/list-vehicle">
              <Button size="lg">
                List Your Vehicle Data
              </Button>
            </Link>
          </CardContent>
        </Card>
      </div>
    </div>
  );
};