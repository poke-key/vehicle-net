'use client';

import { useMemo, useState, useEffect } from 'react'
import { mockVehicleListings, mockTelemetryData, type MockVehicle } from '@/lib/mock-data'

export interface VehicleWithMetadata {
  id: string
  vin: string
  wallet: string
  manufacturer: string
  model: string
  year: number
  isActive: boolean
  registrationTimestamp: number
  owner: string
  dataTypes: string[]
  ipfsHash: string
  lastUpdate: number
  mileage: number
  batteryHealth?: number
  condition: string
  price: string
  verified: boolean
  title: string
  streamType: string
  location: string
  frequency: string
  billingPeriod: string
  description?: string
  features?: string[]
  totalSales?: number
  rating?: number
}

// Hook to fetch a single vehicle with all its data
export function useVehicleWithMetadata(vehicleId: string) {
  const [isLoading, setIsLoading] = useState(true)
  const [error, setError] = useState<Error | null>(null)
  
  const vehicle = useMemo(() => {
    const mockVehicle = mockVehicleListings.find(v => v.id === vehicleId)
    if (!mockVehicle) return null
    
    return {
      id: mockVehicle.id,
      vin: mockVehicle.vin,
      wallet: `0x${Math.random().toString(16).substring(2, 42)}`, // Mock wallet
      manufacturer: mockVehicle.make,
      model: mockVehicle.model,
      year: mockVehicle.year,
      isActive: mockVehicle.isActive,
      registrationTimestamp: Math.floor(Date.now() / 1000) - 86400, // 1 day ago
      owner: mockVehicle.owner,
      dataTypes: [mockVehicle.streamType],
      ipfsHash: `Qm${Math.random().toString(36).substring(2, 46)}`, // Mock IPFS hash
      lastUpdate: Math.floor(Date.now() / 1000) - 3600, // 1 hour ago
      mileage: mockVehicle.mileage,
      batteryHealth: mockVehicle.batteryHealth,
      condition: mockVehicle.condition,
      price: mockVehicle.price,
      verified: mockVehicle.verified,
      title: mockVehicle.title,
      streamType: mockVehicle.streamType,
      location: mockVehicle.location,
      frequency: mockVehicle.frequency,
      billingPeriod: mockVehicle.billingPeriod,
      description: mockVehicle.description,
      features: mockVehicle.features,
      totalSales: mockVehicle.totalSales,
      rating: mockVehicle.rating,
    } as VehicleWithMetadata
  }, [vehicleId])

  useEffect(() => {
    // Simulate loading delay
    const timer = setTimeout(() => {
      setIsLoading(false)
      if (!vehicle) {
        setError(new Error(`Vehicle with ID ${vehicleId} not found`))
      }
    }, 500)
    
    return () => clearTimeout(timer)
  }, [vehicle, vehicleId])

  const refetch = () => {
    setIsLoading(true)
    setError(null)
    setTimeout(() => setIsLoading(false), 300)
  }

  return {
    vehicle,
    isLoading,
    error,
    refetch,
  }
}

// Hook to fetch all vehicles with their metadata
export function useAllVehicles() {
  const [isLoading, setIsLoading] = useState(true)
  const [error, setError] = useState<Error | null>(null)
  
  const vehicles = useMemo(() => {
    return mockVehicleListings.map(mockVehicle => ({
      id: mockVehicle.id,
      vin: mockVehicle.vin,
      wallet: `0x${Math.random().toString(16).substring(2, 42)}`, // Mock wallet
      manufacturer: mockVehicle.make,
      model: mockVehicle.model,
      year: mockVehicle.year,
      isActive: mockVehicle.isActive,
      registrationTimestamp: Math.floor(Date.now() / 1000) - 86400, // 1 day ago
      owner: mockVehicle.owner,
      dataTypes: [mockVehicle.streamType],
      ipfsHash: `Qm${Math.random().toString(36).substring(2, 46)}`, // Mock IPFS hash
      lastUpdate: Math.floor(Date.now() / 1000) - 3600, // 1 hour ago
      mileage: mockVehicle.mileage,
      batteryHealth: mockVehicle.batteryHealth,
      condition: mockVehicle.condition,
      price: mockVehicle.price,
      verified: mockVehicle.verified,
      title: mockVehicle.title,
      streamType: mockVehicle.streamType,
      location: mockVehicle.location,
      frequency: mockVehicle.frequency,
      billingPeriod: mockVehicle.billingPeriod,
      description: mockVehicle.description,
      features: mockVehicle.features,
      totalSales: mockVehicle.totalSales,
      rating: mockVehicle.rating,
    })) as VehicleWithMetadata[]
  }, [])

  const totalVehicles = vehicles.length

  useEffect(() => {
    // Simulate loading delay
    const timer = setTimeout(() => {
      setIsLoading(false)
    }, 800)
    
    return () => clearTimeout(timer)
  }, [])

  const refetch = () => {
    setIsLoading(true)
    setError(null)
    setTimeout(() => setIsLoading(false), 500)
  }

  // Helper functions
  const getVehicleById = (id: string) => {
    return vehicles.find(v => v.id === id)
  }

  const getVehicleByVin = (vin: string) => {
    return vehicles.find(v => v.vin === vin)
  }

  const getActiveVehicles = () => {
    return vehicles.filter(v => v.isActive)
  }

  const getVehiclesByOwner = (owner: string) => {
    return vehicles.filter(v => v.owner.toLowerCase() === owner.toLowerCase())
  }

  return {
    vehicles,
    totalVehicles,
    isLoading,
    error,
    refetch,
    getVehicleById,
    getVehicleByVin,
    getActiveVehicles,
    getVehiclesByOwner,
  }
}

// Hook to get real-time telemetry data for a vehicle
export function useVehicleTelemetry(vehicleId: string) {
  const [telemetryData, setTelemetryData] = useState(mockTelemetryData[vehicleId as keyof typeof mockTelemetryData])
  
  useEffect(() => {
    // Simulate real-time data updates
    const interval = setInterval(() => {
      const baseTelemetry = mockTelemetryData[vehicleId as keyof typeof mockTelemetryData]
      if (baseTelemetry) {
        setTelemetryData({
          ...baseTelemetry,
          timestamp: Date.now(),
          gps: {
            ...baseTelemetry.gps,
            speed: baseTelemetry.gps.speed + (Math.random() - 0.5) * 10, // Simulate speed changes
          },
          batteryHealth: Math.max(90, baseTelemetry.batteryHealth + (Math.random() - 0.5) * 2),
        })
      }
    }, 2000) // Update every 2 seconds
    
    return () => clearInterval(interval)
  }, [vehicleId])
  
  return telemetryData
}

// Legacy export for compatibility
export const useVehicleData = useAllVehicles;