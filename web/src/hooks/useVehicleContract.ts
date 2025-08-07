import { useWriteContract, useReadContract, useWaitForTransactionReceipt } from 'wagmi';
import { parseEther, formatEther } from 'viem';
import { mockContractABI, mockContractAddress } from '@/lib/mock-data';

export function useVehicleContract() {
  const { writeContract, data: hash, isPending: isWritePending } = useWriteContract();
  
  const { isLoading: isConfirming, isSuccess: isConfirmed } = 
    useWaitForTransactionReceipt({ 
      hash,
    });

  // Purchase data access
  const purchaseDataAccess = async (vehicleId: string, duration: number, priceInEth: number) => {
    try {
      await writeContract({
        address: mockContractAddress as `0x${string}`,
        abi: mockContractABI,
        functionName: 'purchaseDataAccess',
        args: [vehicleId, BigInt(duration)],
        value: parseEther(priceInEth.toString()),
      });
    } catch (error) {
      console.error('Failed to purchase data access:', error);
      throw error;
    }
  };

  // List vehicle data
  const listVehicleData = async (vehicleId: string, priceInEth: number, dataType: string) => {
    try {
      const priceInWei = parseEther(priceInEth.toString());
      
      await writeContract({
        address: mockContractAddress as `0x${string}`,
        abi: mockContractABI,
        functionName: 'listVehicleData',
        args: [vehicleId, priceInWei, dataType],
      });
    } catch (error) {
      console.error('Failed to list vehicle data:', error);
      throw error;
    }
  };

  return {
    purchaseDataAccess,
    listVehicleData,
    isWritePending,
    isConfirming,
    isConfirmed,
    hash,
  };
}

// Hook to read vehicle data from contract
export function useReadVehicleData(vehicleId: string) {
  const { data, isLoading, error } = useReadContract({
    address: mockContractAddress as `0x${string}`,
    abi: mockContractABI,
    functionName: 'getVehicleData',
    args: [vehicleId],
  });

  // Parse the returned data
  const vehicleData = data ? {
    owner: data[0],
    price: formatEther(data[1] as bigint),
    dataType: data[2],
    isActive: data[3],
  } : null;

  return {
    vehicleData,
    isLoading,
    error,
  };
}