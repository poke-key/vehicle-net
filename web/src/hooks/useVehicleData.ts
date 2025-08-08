'use client';

import { useMemo } from 'react'
import { useTotalVehicles, useVehicleData as useVehicleDataContract, useVehicleMetadata, useIsVehicleActive } from './useVehicleContract'

export interface VehicleWithMetadata {
  id: number
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
}

// Hook to fetch a single vehicle with all its data
export function useVehicleWithMetadata(vehicleId: number) {
  const { vehicle, isLoading: vehicleLoading, error: vehicleError, refetch: refetchVehicle } = useVehicleDataContract(vehicleId)
  const { metadata, isLoading: metadataLoading, error: metadataError, refetch: refetchMetadata } = useVehicleMetadata(vehicleId)

  const combinedData = useMemo(() => {
    if (!vehicle || !metadata) return null
    
    return {
      id: vehicleId,
      vin: vehicle.vin,
      wallet: vehicle.wallet,
      manufacturer: vehicle.manufacturer,
      model: vehicle.model,
      year: vehicle.year,
      isActive: vehicle.isActive,
      registrationTimestamp: vehicle.registrationTimestamp,
      owner: vehicle.owner,
      dataTypes: metadata.dataTypes,
      ipfsHash: metadata.ipfsHash,
      lastUpdate: metadata.lastUpdate,
    } as VehicleWithMetadata
  }, [vehicle, metadata, vehicleId])

  const isLoading = vehicleLoading || metadataLoading
  const error = vehicleError || metadataError

  const refetch = () => {
    refetchVehicle()
    refetchMetadata()
  }

  return {
    vehicle: combinedData,
    isLoading,
    error,
    refetch,
  }
}

// Hook to fetch all vehicles with their metadata
export function useAllVehicles() {
  const { totalVehicles, isLoading: totalLoading, error: totalError, refetch: refetchTotal } = useTotalVehicles()

  // Generate array of vehicle IDs (1 to totalVehicles)
  const vehicleIds = useMemo(() => {
    if (!totalVehicles || totalVehicles === 0) return []
    return Array.from({ length: totalVehicles }, (_, i) => i + 1)
  }, [totalVehicles])

  // Fetch data for each vehicle
  const vehicleQueries = vehicleIds.map(id => useVehicleWithMetadata(id))

  // Combine all results
  const vehicles = useMemo(() => {
    return vehicleQueries
      .map(query => query.vehicle)
      .filter((vehicle): vehicle is VehicleWithMetadata => vehicle !== null)
  }, [vehicleQueries])

  const isLoading = totalLoading || vehicleQueries.some(query => query.isLoading)
  const error = totalError || vehicleQueries.find(query => query.error)?.error

  const refetch = () => {
    refetchTotal()
    vehicleQueries.forEach(query => query.refetch())
  }

  // Helper functions
  const getVehicleById = (id: number) => {
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

// Legacy export for compatibility
export const useVehicleData = useAllVehicles;