'use client';

import React, { createContext, useContext, useEffect, useState, ReactNode } from 'react';
import { useAccount, useDisconnect } from 'wagmi';
import { connectPorto, disconnectPorto, getPortoAccount, isPortoConnected } from '@/lib/porto';

interface WalletState {
  address: string | null;
  isConnected: boolean;
  isPortoConnected: boolean;
  isWagmiConnected: boolean;
  isLoading: boolean;
  error: string | null;
}

interface WalletContextType extends WalletState {
  connectWithPorto: () => Promise<void>;
  connectWithWagmi: () => void;
  disconnect: () => Promise<void>;
  clearError: () => void;
}

const WalletContext = createContext<WalletContextType | null>(null);

export const useWallet = () => {
  const context = useContext(WalletContext);
  if (!context) {
    throw new Error('useWallet must be used within a WalletProvider');
  }
  return context;
};

interface WalletProviderProps {
  children: ReactNode;
}

export const WalletProvider: React.FC<WalletProviderProps> = ({ children }) => {
  const [state, setState] = useState<WalletState>({
    address: null,
    isConnected: false,
    isPortoConnected: false,
    isWagmiConnected: false,
    isLoading: true,
    error: null,
  });

  // Wagmi hooks
  const { address: wagmiAddress, isConnected: wagmiConnected } = useAccount();
  const { disconnect: wagmiDisconnect } = useDisconnect();

  // Update state when Wagmi connection changes
  useEffect(() => {
    setState(prev => ({
      ...prev,
      isWagmiConnected: wagmiConnected,
      address: wagmiConnected ? wagmiAddress || null : prev.address,
      isConnected: wagmiConnected || prev.isPortoConnected,
    }));
  }, [wagmiAddress, wagmiConnected]);

  // Check for existing Porto connection on mount
  useEffect(() => {
    const checkPortoConnection = async () => {
      try {
        const connected = await isPortoConnected();
        const portoAddress = connected ? await getPortoAccount() : null;
        
        setState(prev => ({
          ...prev,
          isPortoConnected: connected,
          address: connected ? portoAddress : prev.address,
          isConnected: connected || prev.isWagmiConnected,
          isLoading: false,
        }));
      } catch (error) {
        setState(prev => ({
          ...prev,
          error: 'Failed to check Porto connection',
          isLoading: false,
        }));
      }
    };

    checkPortoConnection();
  }, []);

  const connectWithPorto = async () => {
    setState(prev => ({ ...prev, isLoading: true, error: null }));
    
    try {
      const address = await connectPorto();
      setState(prev => ({
        ...prev,
        address,
        isPortoConnected: true,
        isConnected: true,
        isLoading: false,
        error: null,
      }));
    } catch (error) {
      setState(prev => ({
        ...prev,
        error: error instanceof Error ? error.message : 'Failed to connect with Porto',
        isLoading: false,
      }));
    }
  };

  const connectWithWagmi = () => {
    // Wagmi connection is handled by the connect-wallet component
    // This just clears any Porto connection if switching to Wagmi
    setState(prev => ({
      ...prev,
      isPortoConnected: false,
      error: null,
    }));
  };

  const disconnect = async () => {
    setState(prev => ({ ...prev, isLoading: true, error: null }));
    
    try {
      // Disconnect both Porto and Wagmi
      if (state.isPortoConnected) {
        await disconnectPorto();
      }
      
      if (state.isWagmiConnected) {
        wagmiDisconnect();
      }
      
      setState(prev => ({
        ...prev,
        address: null,
        isConnected: false,
        isPortoConnected: false,
        isWagmiConnected: false,
        isLoading: false,
        error: null,
      }));
    } catch (error) {
      setState(prev => ({
        ...prev,
        error: error instanceof Error ? error.message : 'Failed to disconnect',
        isLoading: false,
      }));
    }
  };

  const clearError = () => {
    setState(prev => ({ ...prev, error: null }));
  };

  const contextValue: WalletContextType = {
    ...state,
    connectWithPorto,
    connectWithWagmi,
    disconnect,
    clearError,
  };

  return (
    <WalletContext.Provider value={contextValue}>
      {children}
    </WalletContext.Provider>
  );
};