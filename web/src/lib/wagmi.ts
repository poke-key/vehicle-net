import { http, createConfig, createStorage } from 'wagmi'
import { baseSepolia, mainnet } from 'wagmi/chains'
import { Porto } from 'porto'

// Initialize Porto
Porto.create()

export const wagmiConfig = createConfig({
  chains: [baseSepolia, mainnet],
  connectors: [],
  storage: createStorage({ storage: localStorage }),
  transports: {
    [baseSepolia.id]: http(),
    [mainnet.id]: http(),
  },
})

declare module 'wagmi' {
  interface Register {
    config: typeof wagmiConfig
  }
}