'use client'

import { useEffect, useState } from 'react'
import { useAccount, useReadContracts, useChainId } from 'wagmi'
import { Porto } from 'porto'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { SIMPLE_VEHICLE_NODES_ABI, getContractAddress, type VehicleNode } from '@/lib/contracts'

export default function VehiclesPage() {
  const [isAuthenticated, setIsAuthenticated] = useState(false)
  const [vehicles, setVehicles] = useState<VehicleNode[]>([])
  const [totalVehicles, setTotalVehicles] = useState(0)
  const { isConnected } = useAccount()
  const chainId = useChainId()
  const contractAddress = getContractAddress(chainId)

  // Check Porto authentication
  useEffect(() => {
    const checkAuth = async () => {
      try {
        // For demo purposes, assume authenticated after timeout
        setTimeout(() => setIsAuthenticated(true), 1000)
      } catch (error) {
        console.error('Porto auth check failed:', error)
        setIsAuthenticated(false)
      }
    }
    checkAuth()
  }, [])

  // Get total vehicles
  const { data: totalVehiclesData } = useReadContracts({
    contracts: [{
      address: contractAddress,
      abi: SIMPLE_VEHICLE_NODES_ABI,
      functionName: 'getTotalVehicles',
    }],
  })

  // Create contracts for each vehicle
  useEffect(() => {
    if (totalVehiclesData?.[0]?.result) {
      const total = Number(totalVehiclesData[0].result)
      setTotalVehicles(total)
    }
  }, [totalVehiclesData])

  // Get all vehicles
  const vehicleContracts = Array.from({ length: totalVehicles }, (_, i) => ({
    address: contractAddress,
    abi: SIMPLE_VEHICLE_NODES_ABI,
    functionName: 'getVehicle' as const,
    args: [BigInt(i + 1)],
  }))

  const { data: vehiclesData } = useReadContracts({
    contracts: vehicleContracts,
  })

  useEffect(() => {
    if (vehiclesData) {
      const vehiclesList: VehicleNode[] = vehiclesData
        .filter((result) => result.status === 'success' && result.result)
        .map((result) => {
          const data = result.result as any
          return {
            vin: data[0] as string,
            nodeAddress: data[1] as string,
            mileage: data[2] as bigint,
            batteryHealth: data[3] as bigint,
            lastReportTimestamp: data[4] as bigint,
            isActive: data[5] as boolean,
          }
        })
        .filter((vehicle) => vehicle.isActive)
      
      setVehicles(vehiclesList)
    }
  }, [vehiclesData])

  const handlePortoLogin = async () => {
    try {
      // For demo purposes, just set authenticated to true
      setIsAuthenticated(true)
    } catch (error) {
      console.error('Porto login failed:', error)
    }
  }

  const formatTimestamp = (timestamp: bigint) => {
    return new Date(Number(timestamp) * 1000).toLocaleString()
  }

  const formatAddress = (address: string) => {
    return `${address.slice(0, 6)}...${address.slice(-4)}`
  }

  if (!isAuthenticated) {
    return (
      <div className="container mx-auto px-4 py-8">
        <div className="max-w-md mx-auto text-center">
          <Card>
            <CardHeader>
              <CardTitle>Access Required</CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              <p className="text-muted-foreground">
                Please authenticate with Porto to view vehicle data.
              </p>
              <Button onClick={handlePortoLogin} className="w-full">
                Login with Porto
              </Button>
            </CardContent>
          </Card>
        </div>
      </div>
    )
  }

  return (
    <div className="container mx-auto px-4 py-8">
      <div className="flex justify-between items-center mb-8">
        <div>
          <h1 className="text-3xl font-bold">Vehicle Network</h1>
          <p className="text-muted-foreground">
            Real-time vehicle data from the blockchain
          </p>
        </div>
        <div className="flex items-center gap-4">
          <Badge variant="secondary">
            {totalVehicles} Total Vehicles
          </Badge>
          <Badge variant={isConnected ? "default" : "destructive"}>
            {isConnected ? "Connected" : "Disconnected"}
          </Badge>
        </div>
      </div>

      {vehicles.length === 0 ? (
        <Card>
          <CardContent className="py-12 text-center">
            <p className="text-muted-foreground">
              No active vehicles found. Add some vehicle nodes using the Rust application.
            </p>
          </CardContent>
        </Card>
      ) : (
        <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
          {vehicles.map((vehicle, index) => (
            <Card key={index} className="hover:shadow-lg transition-shadow">
              <CardHeader>
                <div className="flex justify-between items-start">
                  <CardTitle className="text-lg">{vehicle.vin}</CardTitle>
                  <Badge variant={vehicle.isActive ? "default" : "secondary"}>
                    {vehicle.isActive ? "Active" : "Inactive"}
                  </Badge>
                </div>
              </CardHeader>
              <CardContent className="space-y-4">
                <div className="grid grid-cols-2 gap-4 text-sm">
                  <div>
                    <p className="text-muted-foreground">Node Address</p>
                    <p className="font-mono">{formatAddress(vehicle.nodeAddress)}</p>
                  </div>
                  <div>
                    <p className="text-muted-foreground">Mileage</p>
                    <p className="font-semibold">{vehicle.mileage.toLocaleString()} miles</p>
                  </div>
                  <div>
                    <p className="text-muted-foreground">Battery Health</p>
                    <div className="flex items-center gap-2">
                      <div className="w-12 h-2 bg-muted rounded-full overflow-hidden">
                        <div 
                          className="h-full bg-green-500 transition-all"
                          style={{ width: `${Number(vehicle.batteryHealth)}%` }}
                        />
                      </div>
                      <span className="font-semibold">{Number(vehicle.batteryHealth)}%</span>
                    </div>
                  </div>
                  <div>
                    <p className="text-muted-foreground">Last Report</p>
                    <p className="text-xs">{formatTimestamp(vehicle.lastReportTimestamp)}</p>
                  </div>
                </div>
              </CardContent>
            </Card>
          ))}
        </div>
      )}
    </div>
  )
}