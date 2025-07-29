// Script to extract wallet information
const fs = require('fs');

// Read the keypair from Solana CLI
const keypairPath = '/home/super/.config/solana/devnet.json';
const keypairData = JSON.parse(fs.readFileSync(keypairPath, 'utf8'));

// Convert array to comma-separated string (Rust format)
const privateKeyArray = '[' + keypairData.join(',') + ']';

// Get public key (last 32 bytes are public key)
const publicKeyBytes = keypairData.slice(32);

console.log('🔑 Wallet Information:');
console.log('');
console.log('Public Key (from solana address):', 'A3LWTunmiXDkexjL5MVTkLzgcKWKitHFQLEjfswZBaYV');
console.log('Private Key (Array format):', privateKeyArray);
console.log('');
console.log('✅ Use the Private Key array in your backend/.env file');
console.log('✅ Or you can use the JSON file path directly');
