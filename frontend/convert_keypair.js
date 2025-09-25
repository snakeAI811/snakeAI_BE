const bs58 = require('bs58');
const fs = require('fs');

// Load the keypair from file
const keypair = JSON.parse(fs.readFileSync('/home/super/.config/solana/devnet.json'));
console.log(keypair)
// Convert to Uint8Array
const secretKey = Uint8Array.from(keypair);

// Convert to base58 string
const base58SecretKey = bs58.encode(secretKey);

// Output
console.log("Base58 Private Key for Phantom Import:");
console.log(base58SecretKey);
