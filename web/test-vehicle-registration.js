const { ethers } = require('ethers');

async function registerVehicle() {
  // Connect to local Anvil node
  const provider = new ethers.JsonRpcProvider('http://localhost:8545');
  
  // Use Anvil's default account
  const privateKey = '0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80';
  const wallet = new ethers.Wallet(privateKey, provider);
  
  // Vehicle Registry contract
  const contractAddress = '0x5fbdb2315678afecb367f032d93f642f64180aa3';
  const contractABI = [
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
    },
    {
      inputs: [],
      name: 'vehicleCounter',
      outputs: [{ name: '', type: 'uint256' }],
      stateMutability: 'view',
      type: 'function'
    }
  ];
  
  const contract = new ethers.Contract(contractAddress, contractABI, wallet);
  
  try {
    console.log('Checking initial vehicle count via vehicleCounter...');
    const initialCounter = await contract.vehicleCounter();
    console.log('Initial vehicle counter:', initialCounter.toString());
    
    console.log('Checking initial vehicle count via getTotalVehicles...');
    const initialCount = await contract.getTotalVehicles();
    console.log('Initial vehicle count:', initialCount.toString());
    
    console.log('Registering vehicle...');
    const tx = await contract.registerVehicle(
      'TESTVIN123456789',
      '0x70997970C51812dc3A010C7d01b50e0d17dc79C8', // Anvil's second address
      'Toyota',
      'Camry',
      2023,
      ['GPS', 'Diagnostics'],
      'QmTestHash123',
      { value: ethers.parseEther('0.01') }
    );
    
    console.log('Transaction hash:', tx.hash);
    console.log('Waiting for confirmation...');
    
    const receipt = await tx.wait();
    console.log('Transaction confirmed in block:', receipt.blockNumber);
    console.log('Gas used:', receipt.gasUsed.toString());
    console.log('Logs:', receipt.logs);
    
    console.log('Checking final vehicle count...');
    const finalCount = await contract.getTotalVehicles();
    console.log('Final vehicle count:', finalCount.toString());
    
  } catch (error) {
    console.error('Error:', error);
  }
}

registerVehicle();