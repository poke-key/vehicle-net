'use client';

import { useWallet } from '@/components/WalletProvider';

// Legacy useAuth hook - now uses unified WalletProvider
export const useAuth = () => {
  const wallet = useWallet();
  
  return {
    user: wallet.address ? {
      address: wallet.address,
      isAuthenticated: wallet.isConnected,
    } : null,
    isLoading: wallet.isLoading,
    error: wallet.error,
    login: wallet.connectWithPorto, // Default to Porto for legacy compatibility
    logout: wallet.disconnect,
  };
};