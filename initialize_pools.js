const anchor = require('@coral-xyz/anchor');
const { PublicKey, Keypair, SystemProgram, Connection, clusterApiUrl } = require('@solana/web3.js');
const { TOKEN_PROGRAM_ID, getAssociatedTokenAddress, createAssociatedTokenAccountInstruction } = require('@solana/spl-token');
const fs = require('fs');

async function initializePools() {
    // Load environment variables
    require('dotenv').config();
    
    // Setup connection and provider manually
    const connection = new Connection(process.env.ANCHOR_PROVIDER_URL || 'http://localhost:8899', 'confirmed');
    
    // Load wallet
    const walletKeypair = Keypair.fromSecretKey(
        new Uint8Array(JSON.parse(fs.readFileSync(process.env.ANCHOR_WALLET || '/home/super/.config/solana/devnet.json', 'utf8')))
    );
    
    const wallet = {
        publicKey: walletKeypair.publicKey,
        signTransaction: async (tx) => {
            tx.sign(walletKeypair);
            return tx;
        },
        signAllTransactions: async (txs) => {
            return txs.map(tx => {
                tx.sign(walletKeypair);
                return tx;
            });
        }
    };
    
    const provider = new anchor.AnchorProvider(connection, wallet, { commitment: 'confirmed' });
    anchor.setProvider(provider);
    
    // Load program
    const idl = JSON.parse(fs.readFileSync('./target/idl/snake_contract.json', 'utf8'));
    const programId = new PublicKey(process.env.PROGRAM_ID || idl.address || idl.metadata.address);
    const program = new anchor.Program(idl, programId, provider);
    
    // Validate token mint address
    let tokenMint;
    try {
        const tokenMintStr = process.env.TOKEN_MINT;
        console.log('🔍 Debug - TOKEN_MINT from env:', tokenMintStr);
        console.log('🔍 Debug - PROGRAM_ID from env:', process.env.PROGRAM_ID);
        
        if (!tokenMintStr) {
            throw new Error('TOKEN_MINT not set in environment');
        }
        
        console.log('🔍 Debug - Creating PublicKey for token mint...');
        tokenMint = new PublicKey(tokenMintStr);
        console.log('🔍 Debug - Token mint PublicKey created successfully');
        
        // Verify the token mint exists on localnet
        console.log('🔍 Debug - Checking token mint account on localnet...');
        const mintInfo = await connection.getAccountInfo(tokenMint);
        if (!mintInfo) {
            throw new Error('Token mint account not found on localnet');
        }
        console.log('🔍 Debug - Token mint account verified on localnet');
    } catch (error) {
        console.error('❌ Token mint validation failed:', error.message);
        console.error('❌ Stack trace:', error.stack);
        console.log('💡 Please run: ./setup_dev_env.sh token');
        process.exit(1);
    }
    
    console.log('🚀 Initializing pools...');
    console.log('Wallet:', wallet.publicKey.toString());
    console.log('Token Mint:', tokenMint.toString());
    console.log('Program ID:', programId.toString());
    
    try {
        // Initialize reward pool
        console.log('\n📦 Initializing reward pool...');
        
        const [rewardPoolPda] = PublicKey.findProgramAddressSync(
            [Buffer.from("reward_pool"), tokenMint.toBuffer()],
            programId
        );
        
        const rewardPoolTokenAccount = await getAssociatedTokenAddress(
            tokenMint,
            rewardPoolPda,
            true
        );
        
        const walletTokenAccount = await getAssociatedTokenAddress(
            tokenMint,
            wallet.publicKey
        );
        
        const rewardPoolAmount = process.env.REWARD_POOL_AMOUNT || '100000000';
        
        const tx = await program.methods
            .initializeRewardPool({
                totalRewards: new anchor.BN(rewardPoolAmount),
                rewardsPerSecond: new anchor.BN('1000'), // 1000 tokens per second
                startTime: new anchor.BN(Math.floor(Date.now() / 1000)),
                endTime: new anchor.BN(Math.floor(Date.now() / 1000) + 86400 * 30), // 30 days
            })
            .accounts({
                rewardPool: rewardPoolPda,
                mint: tokenMint,
                rewardPoolTokenAccount: rewardPoolTokenAccount,
                authority: wallet.publicKey,
                authorityTokenAccount: walletTokenAccount,
                tokenProgram: TOKEN_PROGRAM_ID,
                systemProgram: SystemProgram.programId,
            })
            .rpc();
        
        console.log('✅ Reward pool initialized:', tx);
        console.log('   Pool PDA:', rewardPoolPda.toString());
        console.log('   Amount:', rewardPoolAmount, 'tokens');
        
    } catch (error) {
        if (error.message.includes('already in use')) {
            console.log('⚠️  Reward pool already exists');
        } else {
            console.error('❌ Error initializing reward pool:', error.message);
        }
    }
    
    try {
        // Initialize DAO registry
        console.log('\n🏛️  Initializing DAO registry...');
        
        const [daoRegistryPda] = PublicKey.findProgramAddressSync(
            [Buffer.from("dao_registry")],
            programId
        );
        
        const maxSeats = process.env.MAX_DAO_SEATS || '21';
        const minDaoStake = process.env.MIN_DAO_STAKE || '1000000';
        const month6Timestamp = process.env.MONTH6_TIMESTAMP || Math.floor(Date.now() / 1000) + 15552000;
        
        const tx = await program.methods
            .initializeDaoRegistry(
                parseInt(maxSeats),
                new anchor.BN(minDaoStake),
                new anchor.BN(month6Timestamp)
            )
            .accounts({
                daoRegistry: daoRegistryPda,
                authority: wallet.publicKey,
                systemProgram: SystemProgram.programId,
            })
            .rpc();
        
        console.log('✅ DAO registry initialized:', tx);
        console.log('   Registry PDA:', daoRegistryPda.toString());
        console.log('   Max seats:', maxSeats);
        console.log('   Min stake:', minDaoStake, 'tokens');
        
    } catch (error) {
        if (error.message.includes('already in use')) {
            console.log('⚠️  DAO registry already exists');
        } else {
            console.error('❌ Error initializing DAO registry:', error.message);
        }
    }
    
    console.log('\n🎉 Pool initialization completed!');
}

// Load environment variables
require('dotenv').config();

initializePools().catch(console.error);
