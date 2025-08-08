import { useWriteContract, useReadContract, useWaitForTransactionReceipt } from 'wagmi';
import { parseEther } from 'viem';
import { VEHICLE_REGISTRY_ADDRESS, VEHICLE_REGISTRY_ABI, DATA_MARKETPLACE_ADDRESS, DATA_MARKETPLACE_ABI } from '@/lib/contracts';

export function useVehicleContract() {
  const { writeContract, data: hash, isPending: isWritePending } = useWriteContract();
  
  const { isLoading: isConfirming, isSuccess: isConfirmed } = 
    useWaitForTransactionReceipt({ 
      hash,
    });

  // Register a new vehicle
  const registerVehicle = async (
    vin: string,
    vehicleWallet: string,
    manufacturer: string,
    model: string,
    year: number,
    dataTypes: string[],
    ipfsHash: string,
    registrationFeeEth: number = 0.01
  ) => {
    try {
      await writeContract({
        address: VEHICLE_REGISTRY_ADDRESS,
        abi: VEHICLE_REGISTRY_ABI,
        functionName: 'registerVehicle',
        args: [vin, vehicleWallet as `0x${string}`, manufacturer, model, BigInt(year), dataTypes, ipfsHash],
        value: parseEther(registrationFeeEth.toString()),
      });
    } catch (error) {
      console.error('Failed to register vehicle:', error);
      throw error;
    }
  };

  return {
    registerVehicle,
    isWritePending,
    isConfirming,
    isConfirmed,
    hash,
  };
}

// Hook to get total number of vehicles
export function useTotalVehicles() {
  const { data, isLoading, error, refetch } = useReadContract({
    address: VEHICLE_REGISTRY_ADDRESS,
    abi: VEHICLE_REGISTRY_ABI,
    functionName: 'getTotalVehicles',
  });

  return {
    totalVehicles: data ? Number(data) : 0,
    isLoading,
    error,
    refetch,
  };
}

// Hook to read vehicle data by ID
export function useVehicleData(vehicleId: number) {
  const { data, isLoading, error, refetch } = useReadContract({
    address: VEHICLE_REGISTRY_ADDRESS,
    abi: VEHICLE_REGISTRY_ABI,
    functionName: 'getVehicle',
    args: [BigInt(vehicleId)],
    query: {
      enabled: vehicleId > 0,
    },
  });

  // Parse the returned vehicle data
  const vehicle = data ? {
    vin: data[0] as string,
    wallet: data[1] as string,
    manufacturer: data[2] as string,
    model: data[3] as string,
    year: Number(data[4]),
    isActive: data[5] as boolean,
    registrationTimestamp: Number(data[6]),
    owner: data[7] as string,
  } : null;

  return {
    vehicle,
    isLoading,
    error,
    refetch,
  };
}

// Hook to read vehicle metadata by ID
export function useVehicleMetadata(vehicleId: number) {
  const { data, isLoading, error, refetch } = useReadContract({
    address: VEHICLE_REGISTRY_ADDRESS,
    abi: VEHICLE_REGISTRY_ABI,
    functionName: 'getVehicleMetadata',
    args: [BigInt(vehicleId)],
    query: {
      enabled: vehicleId > 0,
    },
  });

  // Parse the returned metadata
  const metadata = data ? {
    dataTypes: data[0] as string[],
    ipfsHash: data[1] as string,
    lastUpdate: Number(data[2]),
  } : null;

  return {
    metadata,
    isLoading,
    error,
    refetch,
  };
}

// Hook to get vehicle ID by wallet address
export function useVehicleIdByWallet(walletAddress?: string) {
  const { data, isLoading, error, refetch } = useReadContract({
    address: VEHICLE_REGISTRY_ADDRESS,
    abi: VEHICLE_REGISTRY_ABI,
    functionName: 'getVehicleIdByWallet',
    args: walletAddress ? [walletAddress as `0x${string}`] : undefined,
    query: {
      enabled: !!walletAddress,
    },
  });

  return {
    vehicleId: data ? Number(data) : 0,
    isLoading,
    error,
    refetch,
  };
}

// Hook to get vehicle ID by VIN
export function useVehicleIdByVin(vin?: string) {
  const { data, isLoading, error, refetch } = useReadContract({
    address: VEHICLE_REGISTRY_ADDRESS,
    abi: VEHICLE_REGISTRY_ABI,
    functionName: 'getVehicleIdByVin',
    args: vin ? [vin] : undefined,
    query: {
      enabled: !!vin,
    },
  });

  return {
    vehicleId: data ? Number(data) : 0,
    isLoading,
    error,
    refetch,
  };
}

// Hook to check if vehicle is active
export function useIsVehicleActive(vehicleId: number) {
  const { data, isLoading, error, refetch } = useReadContract({
    address: VEHICLE_REGISTRY_ADDRESS,
    abi: VEHICLE_REGISTRY_ABI,
    functionName: 'isVehicleActive',
    args: [BigInt(vehicleId)],
    query: {
      enabled: vehicleId > 0,
    },
  });

  return {
    isActive: data as boolean,
    isLoading,
    error,
    refetch,
  };
}