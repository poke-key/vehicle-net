import { http, createConfig, createStorage } from 'wagmi'
import { baseSepolia, mainnet } from 'wagmi/chains'
import { defineChain } from 'viem'
import { Porto } from 'porto'
import { injected, metaMask, walletConnect, coinbaseWallet } from 'wagmi/connectors'

// Initialize Porto
Porto.create()

// Define local Anvil chain
const anvil = defineChain({
  id: 31337,
  name: 'Anvil',
  network: 'anvil',
  nativeCurrency: {
    decimals: 18,
    name: 'Ether',
    symbol: 'ETH',
  },
  rpcUrls: {
    public: { http: [process.env.NEXT_PUBLIC_RPC_URL || 'http://localhost:8545'] },
    default: { http: [process.env.NEXT_PUBLIC_RPC_URL || 'http://localhost:8545'] },
  },
})

export const wagmiConfig = createConfig({
  chains: [anvil, baseSepolia, mainnet],
  connectors: [
    injected(),
    metaMask(),
    walletConnect({ projectId: 'demo' }),
    coinbaseWallet({ appName: 'VehicleNet' }),
  ],
  storage: createStorage({ 
    storage: typeof window !== 'undefined' ? localStorage : undefined 
  }),
  transports: {
    [anvil.id]: http(process.env.NEXT_PUBLIC_RPC_URL || 'http://localhost:8545'),
    [baseSepolia.id]: http(),
    [mainnet.id]: http(),
  },
})

declare module 'wagmi' {
  interface Register {
    config: typeof wagmiConfig
  }
}