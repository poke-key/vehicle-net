"use client";

import { useAccount, useConnect, useDisconnect } from 'wagmi';
import { Button } from './ui/button';
import { 
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from './ui/dropdown-menu';
import { Wallet, LogOut, User } from 'lucide-react';

export function ConnectWallet() {
  const { address, isConnected } = useAccount();
  const { connectors, connect } = useConnect();
  const { disconnect } = useDisconnect();

  if (isConnected) {
    return (
      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <Button variant="default" size="lg" className="gap-2">
            <User className="h-4 w-4" />
            {address ? `${address.slice(0, 6)}...${address.slice(-4)}` : 'Connected'}
          </Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent>
          <DropdownMenuItem 
            onClick={() => navigator.clipboard.writeText(address || '')}
            className="cursor-pointer"
          >
            Copy Address
          </DropdownMenuItem>
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
        <Button size="lg" className="gap-2">
          <Wallet className="h-4 w-4" />
          Connect Wallet
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent>
        {connectors.map((connector) => (
          <DropdownMenuItem 
            key={connector.uid}
            onClick={() => connect({ connector })}
            className="cursor-pointer"
          >
            {connector.name}
          </DropdownMenuItem>
        ))}
      </DropdownMenuContent>
    </DropdownMenu>
  );
}