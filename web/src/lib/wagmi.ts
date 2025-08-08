import { http, createConfig, createStorage } from 'wagmi'
import { baseSepolia, mainnet, localhost } from 'wagmi/chains'
import { Porto } from 'porto'

// Initialize Porto for auth
Porto.create()

export const wagmiConfig = createConfig({
  chains: [localhost, baseSepolia, mainnet],
  connectors: [],
  storage: createStorage({ storage: localStorage }),
  transports: {
    [localhost.id]: http('http://localhost:8545'),
    [baseSepolia.id]: http(),
    [mainnet.id]: http(),
  },
})

declare module 'wagmi' {
  interface Register {
    config: typeof wagmiConfig
  }
}