# Vehicle Marketplace - Web Frontend

A React-based web application for browsing, verifying, and purchasing vehicles on the blockchain.

## Features

- 🚗 Browse vehicle listings from the blockchain
- 🔍 View detailed vehicle condition reports
- ⚡ Connect wallet via WalletConnect/MetaMask
- 💰 Purchase vehicles using ETH (Porto integration)
- 📊 Vehicle condition history and verification
- 📱 Responsive design with Tailwind CSS

## Tech Stack

- **Framework**: Next.js 13+ with App Router
- **Styling**: Tailwind CSS
- **Blockchain**: Wagmi + Viem for Ethereum interactions
- **State Management**: React Context + useReducer
- **Type Safety**: TypeScript
- **Wallet Connection**: RainbowKit or ConnectKit
- **Data Fetching**: SWR or TanStack Query

## Prerequisites

- Node.js 18+ and npm/yarn
- A running Ethereum node (Anvil for local development)
- Deployed smart contracts (VehicleConditionRegistry, ListingMarketplace, PortoReceiver)

## Quick Start

```bash
cd web-marketplace
npm install
cp .env.example .env.local
# Edit .env.local with your contract addresses
npm run dev
```

## Environment Variables

Create `.env.local`:

```env
# Blockchain Configuration
NEXT_PUBLIC_CHAIN_ID=31337
NEXT_PUBLIC_RPC_URL=http://localhost:8545
NEXT_PUBLIC_CONDITION_REGISTRY_ADDRESS=0x...
NEXT_PUBLIC_MARKETPLACE_ADDRESS=0x...
NEXT_PUBLIC_PORTO_RECEIVER_ADDRESS=0x...

# Optional: IPFS Gateway
NEXT_PUBLIC_IPFS_GATEWAY=https://gateway.pinata.cloud/ipfs/

# Optional: Analytics
NEXT_PUBLIC_ANALYTICS_ID=
```

## Project Structure

```
web-marketplace/
├── src/
│   ├── app/                    # Next.js App Router pages
│   │   ├── page.tsx           # Home page - listing browser
│   │   ├── listing/           
│   │   │   └── [id]/          
│   │   │       └── page.tsx   # Individual listing details
│   │   └── layout.tsx         # Root layout with providers
│   ├── components/            # Reusable UI components
│   │   ├── ui/                # Basic UI components (Button, Card, etc)
│   │   ├── ListingCard.tsx    # Vehicle listing preview
│   │   ├── ListingDetails.tsx # Detailed listing view
│   │   ├── ConditionChart.tsx # Vehicle condition visualization
│   │   ├── PaymentWidget.tsx  # Purchase interface
│   │   └── WalletConnect.tsx  # Wallet connection component
│   ├── hooks/                 # Custom React hooks
│   │   ├── useContract.ts     # Contract interaction hooks
│   │   ├── useListings.ts     # Marketplace data hooks
│   │   └── useConditionReports.ts # Vehicle condition hooks
│   ├── lib/                   # Utility functions
│   │   ├── contracts.ts       # Contract ABIs and addresses
│   │   ├── utils.ts           # General utilities
│   │   ├── ipfs.ts            # IPFS data fetching
│   │   └── signatures.ts      # Signature verification
│   ├── types/                 # TypeScript type definitions
│   │   ├── contracts.ts       # Contract types
│   │   └── vehicle.ts         # Vehicle/listing types
│   └── styles/
│       └── globals.css        # Global styles and Tailwind imports
├── public/
│   └── images/                # Static images
├── package.json
├── tsconfig.json
├── tailwind.config.js
├── next.config.js
└── README.md
```

## Key Features Implementation

### 1. Listing Browser
- Fetch active listings from `ListingMarketplace` contract
- Display cards with basic info (make, model, year, price)
- Filter and search functionality
- Pagination for large datasets

### 2. Listing Details
- Detailed vehicle information from IPFS metadata
- Condition report history from `VehicleConditionRegistry`
- Interactive charts showing condition trends over time
- Signature verification for authenticity

### 3. Purchase Flow
- Connect wallet integration
- Price display and confirmation
- Transaction signing through wallet
- Support for Porto payment integration
- Success/failure handling with clear feedback

### 4. Condition Verification
- Fetch all condition reports for a vehicle
- Verify signatures against vehicle addresses
- Display condition trends (battery, engine, brakes, tires)
- Flag any suspicious or missing reports

## Smart Contract Integration

The app integrates with three main contracts:

1. **VehicleConditionRegistry**: Read condition reports and vehicle history
2. **ListingMarketplace**: Browse listings, purchase vehicles
3. **PortoReceiver**: Handle alternative payment methods (optional)

## Development Commands

```bash
npm run dev          # Start development server
npm run build        # Build for production
npm run start        # Start production server
npm run lint         # Run ESLint
npm run type-check   # Run TypeScript compiler
npm run test         # Run tests (if implemented)
```

## Deployment

### Vercel (Recommended)
1. Connect your GitHub repository to Vercel
2. Set environment variables in Vercel dashboard
3. Deploy automatically on push to main

### Alternative Platforms
- Netlify: Configure build command and environment variables
- Railway: Use `railway up` with proper env vars
- Self-hosted: Build with `npm run build` and serve `out/` directory

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## Testing Strategy

- **Unit Tests**: Individual component testing
- **Integration Tests**: Contract interaction testing
- **E2E Tests**: Full user flow testing with Playwright
- **Manual Testing**: Test with actual deployed contracts

## Security Considerations

- All contract addresses are verified before interactions
- Signature verification for condition reports
- Input validation on all user inputs
- Secure environment variable handling
- No private keys stored in frontend code

## Browser Support

- Chrome 90+
- Firefox 88+
- Safari 14+
- Edge 90+

## Performance Optimizations

- Next.js automatic code splitting
- Image optimization with next/image
- Lazy loading for heavy components
- SWR/React Query for efficient data fetching
- Responsive images and lazy loading

## Future Enhancements

- [ ] Advanced filtering (price range, condition scores)
- [ ] Favorite listings with local storage
- [ ] Email notifications for listing updates
- [ ] Mobile app with React Native
- [ ] Integration with vehicle APIs for real-time data
- [ ] Multi-language support
- [ ] Dark mode support

## Support

For questions and support:
- Check existing GitHub issues
- Create new issue with detailed description
- Include browser, OS, and reproduction steps