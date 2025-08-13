const { createPublicClient, createWalletClient, http } = require('viem');
const { defineChain } = require('viem');
const { privateKeyToAccount } = require('viem/accounts');

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
    public: { http: ['http://localhost:8545'] },
    default: { http: ['http://localhost:8545'] },
  },
});

// Use Anvil's default first account private key
const account = privateKeyToAccount('0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80');

const VEHICLE_REGISTRY_ABI = [
  {
    inputs: [
      { name: 'vin', type: 'string' },
      { name: 'vehicleWallet', type: 'address' },
      { name: 'manufacturer', type: 'string' },
      { name: 'model', type: 'string' },
      { name: 'year', type: 'uint256' },
      { name: 'dataTypes', type: 'string[]' },
      { name: 'ipfsHash', type: 'string' }
    ],
    name: 'registerVehicle',
    outputs: [],
    stateMutability: 'payable',
    type: 'function'
  },
  {
    inputs: [],
    name: 'getTotalVehicles',
    outputs: [{ name: '', type: 'uint256' }],
    stateMutability: 'view',
    type: 'function'
  }
];

async function addTestVehicle() {
  try {
    const publicClient = createPublicClient({
      chain: anvil,
      transport: http('http://localhost:8545'),
    });

    const walletClient = createWalletClient({
      account,
      chain: anvil,
      transport: http('http://localhost:8545'),
    });

    console.log('Adding test vehicle...');
    
    // Register a test vehicle
    const tx = await walletClient.writeContract({
      address: '0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9',
      abi: VEHICLE_REGISTRY_ABI,
      functionName: 'registerVehicle',
      args: [
        'VIN123456789', // VIN
        account.address, // vehicleWallet
        'Tesla', // manufacturer
        'Model 3', // model
        2023n, // year (uint256)
        ['GPS', 'Diagnostics'], // dataTypes
        'QmTestHash123' // ipfsHash
      ],
      value: BigInt(0.01 * 10**18), // 0.01 ether registration fee
    });

    console.log('Transaction hash:', tx);

    // Wait for transaction to be mined
    const receipt = await publicClient.waitForTransactionReceipt({ hash: tx });
    console.log('Transaction confirmed in block:', receipt.blockNumber);

    // Check total vehicles now
    const totalVehicles = await publicClient.readContract({
      address: '0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9',
      abi: VEHICLE_REGISTRY_ABI,
      functionName: 'getTotalVehicles',
    });

    console.log('Total vehicles after registration:', Number(totalVehicles));

  } catch (error) {
    console.error('Error adding test vehicle:', error);
  }
}

addTestVehicle();