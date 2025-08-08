"use client";

import { useConnect } from 'wagmi';
import { Button } from './ui/button';
import { 
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
  DropdownMenuSeparator,
} from './ui/dropdown-menu';
import { Wallet, LogOut, User, Copy } from 'lucide-react';
import { useWallet } from './WalletProvider';
import { toast } from 'sonner';

export function ConnectWallet() {
  const { 
    address, 
    isConnected, 
    isPortoConnected, 
    isWagmiConnected, 
    isLoading,
    connectWithPorto, 
    disconnect,
    clearError 
  } = useWallet();
  
  const { connectors, connect } = useConnect();

  const handleCopyAddress = async () => {
    if (address) {
      await navigator.clipboard.writeText(address);
      toast.success('Address copied to clipboard');
    }
  };

  const handleWagmiConnect = (connector: any) => {
    connect({ connector });
  };

  if (isConnected) {
    return (
      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <Button variant="default" size="lg" className="gap-2" disabled={isLoading}>
            <User className="h-4 w-4" />
            {address ? `${address.slice(0, 6)}...${address.slice(-4)}` : 'Connected'}
            {isPortoConnected && <span className="text-xs text-blue-400">Porto</span>}
            {isWagmiConnected && <span className="text-xs text-green-400">Wagmi</span>}
          </Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent>
          <DropdownMenuItem 
            onClick={handleCopyAddress}
            className="cursor-pointer"
          >
            <Copy className="mr-2 h-4 w-4" />
            Copy Address
          </DropdownMenuItem>
          <DropdownMenuSeparator />
          <DropdownMenuItem 
            onClick={() => disconnect()}
            className="cursor-pointer text-red-600"
          >
            <LogOut className="mr-2 h-4 w-4" />
            Disconnect
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
    );
  }

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <Button size="lg" className="gap-2" disabled={isLoading}>
          <Wallet className="h-4 w-4" />
          {isLoading ? 'Connecting...' : 'Connect Wallet'}
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent>
        <DropdownMenuItem 
          onClick={connectWithPorto}
          className="cursor-pointer"
          disabled={isLoading}
        >
          <span className="mr-2">🚢</span>
          Porto (Recommended)
        </DropdownMenuItem>
        <DropdownMenuSeparator />
        {connectors.map((connector) => (
          <DropdownMenuItem 
            key={connector.uid}
            onClick={() => handleWagmiConnect(connector)}
            className="cursor-pointer"
            disabled={isLoading}
          >
            <Wallet className="mr-2 h-4 w-4" />
            {connector.name}
          </DropdownMenuItem>
        ))}
      </DropdownMenuContent>
    </DropdownMenu>
  );
}