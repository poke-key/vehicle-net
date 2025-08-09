'use client';

import React from 'react';
import { Card, CardContent, CardHeader, CardTitle } from './ui/card';
import { Badge } from './ui/badge';
import { Progress } from './ui/progress';
import { useVehicleTelemetry } from '@/hooks/useMockVehicleData';
import { 
  Battery, 
  MapPin, 
  Thermometer, 
  Gauge,
  Zap,
  Clock,
  Activity
} from 'lucide-react';

interface VehicleTelemetryProps {
  vehicleId: string;
}

export const VehicleTelemetry: React.FC<VehicleTelemetryProps> = ({ vehicleId }) => {
  const telemetryData = useVehicleTelemetry(vehicleId);

  if (!telemetryData) {
    return (
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Activity className="h-5 w-5" />
            Live Telemetry
          </CardTitle>
        </CardHeader>
        <CardContent>
          <p className="text-muted-foreground">No telemetry data available for this vehicle.</p>
        </CardContent>
      </Card>
    );
  }

  const formatTimestamp = (timestamp: number) => {
    return new Date(timestamp).toLocaleTimeString();
  };

  const getChargingStatusColor = (status: string) => {
    switch (status) {
      case 'charging': return 'text-green-600';
      case 'not_charging': return 'text-gray-600';
      case 'error': return 'text-red-600';
      default: return 'text-gray-600';
    }
  };

  const getChargingStatusText = (status: string) => {
    switch (status) {
      case 'charging': return 'Charging';
      case 'not_charging': return 'Not Charging';
      case 'error': return 'Charging Error';
      default: return 'Unknown';
    }
  };

  return (
    <div className="space-y-6">
      {/* Real-time Status */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Activity className="h-5 w-5 text-green-500" />
            Live Telemetry Data
            <Badge variant="outline" className="ml-auto">
              <Clock className="h-3 w-3 mr-1" />
              {formatTimestamp(telemetryData.timestamp)}
            </Badge>
          </CardTitle>
        </CardHeader>
        <CardContent className="grid gap-6 md:grid-cols-2">
          {/* GPS Data */}
          <div className="space-y-4">
            <h4 className="flex items-center gap-2 font-medium">
              <MapPin className="h-4 w-4" />
              Location & Movement
            </h4>
            <div className="space-y-2">
              <div className="flex justify-between">
                <span className="text-sm text-muted-foreground">Latitude</span>
                <span className="font-mono text-sm">{telemetryData.gps.latitude.toFixed(6)}</span>
              </div>
              <div className="flex justify-between">
                <span className="text-sm text-muted-foreground">Longitude</span>
                <span className="font-mono text-sm">{telemetryData.gps.longitude.toFixed(6)}</span>
              </div>
              <div className="flex justify-between">
                <span className="text-sm text-muted-foreground">Speed</span>
                <span className="font-medium">{Math.round(telemetryData.gps.speed)} mph</span>
              </div>
            </div>
          </div>

          {/* Battery Data */}
          <div className="space-y-4">
            <h4 className="flex items-center gap-2 font-medium">
              <Battery className="h-4 w-4" />
              Battery & Power
            </h4>
            <div className="space-y-3">
              <div>
                <div className="flex justify-between text-sm mb-1">
                  <span className="text-muted-foreground">Battery Health</span>
                  <span className="font-medium">{telemetryData.batteryHealth}%</span>
                </div>
                <Progress value={telemetryData.batteryHealth} className="h-2" />
              </div>
              
              <div className="flex justify-between">
                <span className="text-sm text-muted-foreground">Range</span>
                <span className="font-medium">{telemetryData.range} mi</span>
              </div>
              
              <div className="flex justify-between">
                <span className="text-sm text-muted-foreground">Charging Status</span>
                <span className={`font-medium capitalize ${getChargingStatusColor(telemetryData.chargingStatus)}`}>
                  {getChargingStatusText(telemetryData.chargingStatus)}
                </span>
              </div>
              
              {telemetryData.chargingStatus === 'charging' && (
                <div className="flex items-center gap-2 text-sm text-green-600">
                  <Zap className="h-3 w-3" />
                  <span>Currently charging</span>
                </div>
              )}
            </div>
          </div>
        </CardContent>
      </Card>

      {/* Diagnostics */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Gauge className="h-5 w-5" />
            Vehicle Diagnostics
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div className="grid gap-4 md:grid-cols-3">
            <div className="space-y-2">
              <div className="flex items-center gap-2">
                <Thermometer className="h-4 w-4 text-orange-500" />
                <span className="text-sm font-medium">Engine Temperature</span>
              </div>
              <div className="flex justify-between">
                <span className="text-muted-foreground">Current</span>
                <span className="font-medium">{telemetryData.diagnostics.engineTemp}°F</span>
              </div>
              <Progress 
                value={Math.min(100, (telemetryData.diagnostics.engineTemp / 220) * 100)} 
                className="h-2" 
              />
            </div>

            <div className="space-y-2">
              <div className="flex items-center gap-2">
                <Gauge className="h-4 w-4 text-blue-500" />
                <span className="text-sm font-medium">Fuel Level</span>
              </div>
              <div className="flex justify-between">
                <span className="text-muted-foreground">Current</span>
                <span className="font-medium">{telemetryData.diagnostics.fuelLevel}%</span>
              </div>
              <Progress value={telemetryData.diagnostics.fuelLevel} className="h-2" />
            </div>

            <div className="space-y-2">
              <div className="flex items-center gap-2">
                <Battery className="h-4 w-4 text-yellow-500" />
                <span className="text-sm font-medium">Battery Voltage</span>
              </div>
              <div className="flex justify-between">
                <span className="text-muted-foreground">Current</span>
                <span className="font-medium">{telemetryData.diagnostics.batteryVoltage}V</span>
              </div>
              <Progress 
                value={Math.min(100, Math.max(0, ((telemetryData.diagnostics.batteryVoltage - 11) / 3) * 100))} 
                className="h-2" 
              />
            </div>
          </div>

          {telemetryData.diagnostics.errorCodes.length > 0 && (
            <div className="mt-6 p-4 bg-red-50 rounded-lg border border-red-200">
              <h5 className="font-medium text-red-800 mb-2">Active Error Codes</h5>
              <div className="space-y-1">
                {telemetryData.diagnostics.errorCodes.map((code, index) => (
                  <div key={index} className="flex items-center gap-2">
                    <Badge variant="destructive" className="font-mono text-xs">
                      {code}
                    </Badge>
                    <span className="text-sm text-red-700">Check engine system</span>
                  </div>
                ))}
              </div>
            </div>
          )}

          {telemetryData.diagnostics.errorCodes.length === 0 && (
            <div className="mt-6 p-4 bg-green-50 rounded-lg border border-green-200">
              <p className="text-sm text-green-800 font-medium">
                ✅ No diagnostic errors detected - All systems operating normally
              </p>
            </div>
          )}
        </CardContent>
      </Card>
    </div>
  );
};