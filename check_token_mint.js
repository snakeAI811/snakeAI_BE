const { Connection, PublicKey } = require('@solana/web3.js');
require('dotenv').config();

async function checkTokenMint() {
    const connection = new Connection(process.env.ANCHOR_PROVIDER_URL || 'http://localhost:8899', 'confirmed');
    const tokenMintStr = process.env.TOKEN_MINT;
    
    if (!tokenMintStr) {
        console.log('❌ TOKEN_MINT not set in .env file');
        return false;
    }
    
    try {
        const tokenMint = new PublicKey(tokenMintStr);
        const mintInfo = await connection.getAccountInfo(tokenMint);
        
        if (mintInfo) {
            console.log('✅ Token mint exists on localnet:', tokenMintStr);
            return true;
        } else {
            console.log('❌ Token mint not found on localnet:', tokenMintStr);
            return false;
        }
    } catch (error) {
        console.log('❌ Invalid token mint address:', tokenMintStr);
        console.log('Error:', error.message);
        return false;
    }
}

checkTokenMint().then(exists => {
    process.exit(exists ? 0 : 1);
});