const { Keypair } = require('@solana/web3.js');
const bs58 = require('bs58');

// Replace this array with your JSON array from the .env file
const privateKeyArray = [74,137,198,153,11,122,22,247,127,138,30,15,74,141,247,92,117,164,159,19,227,217,21,43,244,68,221,16,106,145,37,205,186,10,146,92,197,49,156,159,93,189,209,227,255,59,246,91,96,45,170,180,251,116,132,240,196,209,243,26,212,179,25,12];

try {
    const keypair = Keypair.fromSecretKey(new Uint8Array(privateKeyArray));
    const base58String = bs58.encode(keypair.secretKey);
    
    console.log('Base58 private key:');
    console.log(base58String);
} catch (error) {
    console.error('Error converting keypair:', error);
}
