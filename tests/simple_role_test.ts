import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SnakeContract } from "../target/types/snake_contract";
import {
  createMint,
  getAssociatedTokenAddressSync,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { Keypair, PublicKey, SystemProgram, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { assert } from "chai";

const REWARD_POOL_SEED = Buffer.from("reward_pool");
const USER_CLAIM_SEED = Buffer.from("user_claim");

describe("Simple Role Test", () => {
  anchor.setProvider(anchor.AnchorProvider.local());
  const program = anchor.workspace.SnakeContract as Program<SnakeContract>;
  const provider = anchor.getProvider();
  const wallet = provider.wallet.payer;
  
  let tokenMint: Keypair;
  let ownerAta: PublicKey;
  let rewardPoolPda: PublicKey;
  let treasuryTokenAccount: PublicKey;

  before(async () => {
    tokenMint = Keypair.generate();
    
    await createMint(
      provider.connection,
      wallet,
      wallet.publicKey,
      null,
      9,
      tokenMint
    );
    
    [rewardPoolPda] = PublicKey.findProgramAddressSync(
      [REWARD_POOL_SEED],
      program.programId
    );
    
    treasuryTokenAccount = getAssociatedTokenAddressSync(
      tokenMint.publicKey,
      rewardPoolPda,
      true
    );
    
    ownerAta = getAssociatedTokenAddressSync(
      tokenMint.publicKey,
      wallet.publicKey,
      false
    );
    
    const ownerAtaAccount = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      wallet,
      tokenMint.publicKey,
      wallet.publicKey
    );
    
    const requiredTokens = 500_000_000 * 1_000_000_000;
    await mintTo(
      provider.connection,
      wallet,
      tokenMint.publicKey,
      ownerAtaAccount.address,
      wallet.publicKey,
      BigInt(requiredTokens)
    );
    
    await program.methods
      .initializeRewardPool({ admin: wallet.publicKey })
      .accounts({ 
        owner: wallet.publicKey,
        ownerAta: ownerAta,
        rewardPool: rewardPoolPda,
        treasury: treasuryTokenAccount,
        mint: tokenMint.publicKey,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .rpc();
  });

  async function setupUserWithMining(user: Keypair, miningAmount: number = 1000): Promise<PublicKey> {
    const airdropSig = await provider.connection.requestAirdrop(
      user.publicKey, 
      2 * LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(airdropSig);
    
    const [userClaimPda] = PublicKey.findProgramAddressSync(
      [USER_CLAIM_SEED, user.publicKey.toBuffer()],
      program.programId
    );
    
    await program.methods
      .initializeUserClaim()
      .accounts({ 
        user: user.publicKey, 
        userClaim: userClaimPda,
        systemProgram: SystemProgram.programId,
      })
      .signers([user])
      .rpc();
    
    const miningAmountTokens = new anchor.BN(miningAmount * 1_000_000_000);
    await program.methods
      .updateUserStats({
        phase1Mined: miningAmountTokens,
        walletAgeDays: 30,
        communityScore: 50,
        phase2MiningCompleted: false,
      })
      .accounts({
        admin: wallet.publicKey,
        rewardPool: rewardPoolPda,
        user: user.publicKey,
        userClaim: userClaimPda,
      })
      .rpc();
    
    return userClaimPda;
  }

  it("Should allow user to select Staker role with mining history", async () => {
    const user = Keypair.generate();
    const userClaimPda = await setupUserWithMining(user, 1000);
    
    await program.methods
      .selectRole({ staker: {} })
      .accounts({ 
        user: user.publicKey, 
        userClaim: userClaimPda,
      })
      .signers([user])
      .rpc();
    
    const userClaimAccount = await program.account.userClaim.fetch(userClaimPda);
    assert.deepEqual(userClaimAccount.role, { staker: {} });
    console.log("✅ Staker role selected successfully with mining history");
  });
});
