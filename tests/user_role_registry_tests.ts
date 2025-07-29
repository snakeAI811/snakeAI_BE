import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SnakeContract } from "../target/types/snake_contract";
import {
  createMint,
  getAccount,
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
const TREASURY_SEED = Buffer.from("treasury");

describe("User Role Registry & Claim Features", () => {
  anchor.setProvider(anchor.AnchorProvider.local());
  const program = anchor.workspace.SnakeContract as Program<SnakeContract>;
  const provider = anchor.getProvider();
  const wallet = provider.wallet.payer;
  
  let tokenMint: Keypair;
  let ownerAta: PublicKey;
  let rewardPoolPda: PublicKey;
  let treasuryPda: PublicKey;
  let treasuryTokenAccount: PublicKey;

  before(async () => {
    console.log("=== SETUP: Creating test environment ===");
    
    tokenMint = Keypair.generate();
    
    // Create mint with proper authority for minting yield
    await createMint(
      provider.connection,
      wallet,
      wallet.publicKey, // mint authority
      null,
      9,
      tokenMint
    );
    
    // Derive reward pool PDA
    [rewardPoolPda] = PublicKey.findProgramAddressSync(
      [REWARD_POOL_SEED],
      program.programId
    );
    
    // Derive treasury PDA
    [treasuryPda] = PublicKey.findProgramAddressSync(
      [TREASURY_SEED],
      program.programId
    );
    
    // Treasury token account (ATA for reward pool)
    treasuryTokenAccount = getAssociatedTokenAddressSync(
      tokenMint.publicKey,
      rewardPoolPda,
      true // allowOwnerOffCurve for PDA
    );
    
    // Owner ATA
    ownerAta = getAssociatedTokenAddressSync(
      tokenMint.publicKey,
      wallet.publicKey,
      false
    );
    
    // Create owner's associated token account
    const ownerAtaAccount = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      wallet,
      tokenMint.publicKey,
      wallet.publicKey
    );
    
    // Mint tokens to owner's ATA (500M tokens for testing)
    const requiredTokens = 500_000_000n * 1_000_000_000n;
    await mintTo(
      provider.connection,
      wallet,
      tokenMint.publicKey,
      ownerAtaAccount.address,
      wallet.publicKey,
      requiredTokens
    );
    
    // Initialize reward pool
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
    
    console.log("Setup completed:");
    console.log("- Token mint:", tokenMint.publicKey.toString());
    console.log("- Owner ATA:", ownerAta.toString());
    console.log("- Reward pool PDA:", rewardPoolPda.toString());
    console.log("- Treasury PDA:", treasuryPda.toString());
    console.log("- Treasury token account:", treasuryTokenAccount.toString());
  });

  // Helper function to set up user with mining history
  async function setupUserWithMining(user: Keypair, miningAmount: number = 1000): Promise<PublicKey> {
    // Request airdrop if needed
    const balance = await provider.connection.getBalance(user.publicKey);
    if (balance < LAMPORTS_PER_SOL) {
      const airdropSig = await provider.connection.requestAirdrop(
        user.publicKey, 
        2 * LAMPORTS_PER_SOL
      );
      await provider.connection.confirmTransaction(airdropSig);
    }
    
    // Derive userClaim PDA
    const [userClaimPda] = PublicKey.findProgramAddressSync(
      [USER_CLAIM_SEED, user.publicKey.toBuffer()],
      program.programId
    );
    
    // Check if user claim already exists
    try {
      const existingClaim = await program.account.userClaim.fetch(userClaimPda);
      console.log("User claim already exists, skipping initialization");
    } catch (error) {
      // Account doesn't exist, initialize it
      await program.methods
        .initializeUserClaim()
        .accounts({ 
          user: user.publicKey, 
          userClaim: userClaimPda,
          systemProgram: SystemProgram.programId,
        })
        .signers([user])
        .rpc();
    }
    
    // Always update mining stats (this can be called multiple times)
    const miningAmountTokens = new anchor.BN(miningAmount * 1_000_000_000); // Convert to token decimals
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

  describe("User Role Registry Tests", () => {
    
    it("Should initialize user claim successfully", async () => {
      console.log("\n=== TEST: Initialize User Claim ===");
      
      const user = Keypair.generate();
      
      // Request airdrop
      const airdropSig = await provider.connection.requestAirdrop(
        user.publicKey, 
        2 * LAMPORTS_PER_SOL
      );
      await provider.connection.confirmTransaction(airdropSig);
      
      // Derive userClaim PDA
      const [userClaimPda] = PublicKey.findProgramAddressSync(
        [USER_CLAIM_SEED, user.publicKey.toBuffer()],
        program.programId
      );
      
      console.log("User:", user.publicKey.toString());
      console.log("User claim PDA:", userClaimPda.toString());
      
      // Initialize user claim
      const initTx = await program.methods
        .initializeUserClaim()
        .accounts({ 
          user: user.publicKey, 
          userClaim: userClaimPda,
          systemProgram: SystemProgram.programId,
        })
        .signers([user])
        .rpc();
        
      console.log("Initialize user claim tx:", initTx);
      
      // Verify user claim was initialized
      const userClaimAccount = await program.account.userClaim.fetch(userClaimPda);
      assert.equal(userClaimAccount.initialized, true);
      assert.equal(userClaimAccount.user.toString(), user.publicKey.toString());
      assert.deepEqual(userClaimAccount.role, { none: {} });
      
      console.log("✅ User claim initialized successfully");
    });

    it("Should allow user to select Staker role", async () => {
      console.log("\n=== TEST: Select Staker Role ===");
      
      const user = Keypair.generate();
      
      // Set up user with mining history
      const userClaimPda = await setupUserWithMining(user, 1000);
      
      // Select Staker role
      const selectTx = await program.methods
        .selectRole({ staker: {} })
        .accounts({ 
          user: user.publicKey, 
          userClaim: userClaimPda,
        })
        .signers([user])
        .rpc();
        
      console.log("Select Staker role tx:", selectTx);
      
      // Verify role was set
      const userClaimAccount = await program.account.userClaim.fetch(userClaimPda);
      assert.deepEqual(userClaimAccount.role, { staker: {} });
      
      console.log("✅ Staker role selected successfully");
    });

    it("Should allow user to select Patron role", async () => {
      console.log("\n=== TEST: Select Patron Role ===");
      
      const user = Keypair.generate();
      
      // Set up user with mining history
      const userClaimPda = await setupUserWithMining(user, 1000);
      
      // First need to apply and get approved as patron
      await program.methods
        .applyForPatron(30, 50)  // wallet age 30 days, community score 50
        .accounts({
          user: user.publicKey,
          userClaim: userClaimPda,
        })
        .signers([user])
        .rpc();
      
      // Admin approves patron application
      await program.methods
        .approvePatronApplication(70) // min qualification score
        .accounts({
          admin: wallet.publicKey,
          userClaim: userClaimPda,
          applicant: user.publicKey,
        })
        .rpc();
      
      // Select Patron role
      const selectTx = await program.methods
        .selectRole({ patron: {} })
        .accounts({ 
          user: user.publicKey, 
          userClaim: userClaimPda,
        })
        .signers([user])
        .rpc();
        
      console.log("Select Patron role tx:", selectTx);
      
      // Verify role was set
      const userClaimAccount = await program.account.userClaim.fetch(userClaimPda);
      assert.deepEqual(userClaimAccount.role, { patron: {} });
      
      console.log("✅ Patron role selected successfully");
    });

    it("Should allow Staker to upgrade to Patron role", async () => {
      console.log("\n=== TEST: Staker to Patron Role Upgrade ===");
      
      const user = Keypair.generate();
      
      // Set up user with mining history and select Staker role
      const userClaimPda = await setupUserWithMining(user, 1000);
      
      await program.methods
        .selectRole({ staker: {} })
        .accounts({ 
          user: user.publicKey, 
          userClaim: userClaimPda,
        })
        .signers([user])
        .rpc();
      
      // Verify Staker role
      let userClaimAccount = await program.account.userClaim.fetch(userClaimPda);
      assert.deepEqual(userClaimAccount.role, { staker: {} });
      
      // Apply for patron status
      await program.methods
        .applyForPatron(30, 50)
        .accounts({
          user: user.publicKey,
          userClaim: userClaimPda,
        })
        .signers([user])
        .rpc();
      
      // Admin approves patron application
      await program.methods
        .approvePatronApplication(70)
        .accounts({
          admin: wallet.publicKey,
          userClaim: userClaimPda,
          applicant: user.publicKey,
        })
        .rpc();
      
      // Upgrade to Patron role
      const upgradeTx = await program.methods
        .selectRole({ patron: {} })
        .accounts({ 
          user: user.publicKey, 
          userClaim: userClaimPda,
        })
        .signers([user])
        .rpc();
        
      console.log("Upgrade to Patron role tx:", upgradeTx);
      
      // Verify role was upgraded
      userClaimAccount = await program.account.userClaim.fetch(userClaimPda);
      assert.deepEqual(userClaimAccount.role, { patron: {} });
      
      console.log("✅ Successfully upgraded from Staker to Patron");
    });

    it("Should prevent invalid role transitions", async () => {
      console.log("\n=== TEST: Invalid Role Transitions ===");
      
      const user = Keypair.generate();
      
      // Set up user with mining history
      const userClaimPda = await setupUserWithMining(user, 1000);
      
      // Apply for patron status and get approved
      await program.methods
        .applyForPatron(30, 50)
        .accounts({
          user: user.publicKey,
          userClaim: userClaimPda,
        })
        .signers([user])
        .rpc();
      
      await program.methods
        .approvePatronApplication(70)
        .accounts({
          admin: wallet.publicKey,
          userClaim: userClaimPda,
          applicant: user.publicKey,
        })
        .rpc();
      
      // Select Patron role
      await program.methods
        .selectRole({ patron: {} })
        .accounts({ 
          user: user.publicKey, 
          userClaim: userClaimPda,
        })
        .signers([user])
        .rpc();
      
      // Try to downgrade from Patron to Staker (should fail)
      try {
        await program.methods
          .selectRole({ staker: {} })
          .accounts({ 
            user: user.publicKey, 
            userClaim: userClaimPda,
          })
          .signers([user])
          .rpc();
        
        assert.fail("Should have failed to downgrade from Patron to Staker");
      } catch (error: any) {
        console.log("✅ Correctly prevented invalid role transition:", error.message);
        assert.include(error.message, "InvalidRoleTransition");
      }
    });
  });

  describe("Claim Tokens with Role Tests", () => {
    
    it("Should allow user to claim tokens with Staker role", async () => {
      console.log("\n=== TEST: Claim Tokens with Staker Role ===");
      
      const user = Keypair.generate();
      const claimAmount = new anchor.BN(1000 * 1_000_000_000); // 1000 tokens
      
      // Set up user with mining history
      const userClaimPda = await setupUserWithMining(user, 1000);
      
      // Create user's token account
      const userTokenAta = await getOrCreateAssociatedTokenAccount(
        provider.connection,
        wallet, // payer
        tokenMint.publicKey,
        user.publicKey
      );
      
      // Check initial balance
      const initialBalance = await getAccount(provider.connection, userTokenAta.address);
      console.log("Initial user token balance:", initialBalance.amount.toString());
      
      // Claim tokens with Staker role
      const claimTx = await program.methods
        .claimTokensWithRole(claimAmount, { staker: {} })
        .accounts({
          user: user.publicKey,
          userClaim: userClaimPda,
          userTokenAta: userTokenAta.address,
          rewardPoolPda: rewardPoolPda,
          treasuryTokenAccount: treasuryTokenAccount,
          mint: tokenMint.publicKey,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .signers([user])
        .rpc();
        
      console.log("Claim tokens with Staker role tx:", claimTx);
      
      // Verify tokens were transferred and role was set
      const finalBalance = await getAccount(provider.connection, userTokenAta.address);
      const userClaimAccount = await program.account.userClaim.fetch(userClaimPda);
      
      console.log("Final user token balance:", finalBalance.amount.toString());
      console.log("User role:", userClaimAccount.role);
      
      assert.equal(finalBalance.amount.toString(), claimAmount.toString());
      assert.deepEqual(userClaimAccount.role, { staker: {} });
      
      console.log("✅ Successfully claimed tokens with Staker role");
    });

    it("Should allow user to claim tokens with Patron role", async () => {
      console.log("\n=== TEST: Claim Tokens with Patron Role ===");
      
      const user = Keypair.generate();
      const claimAmount = new anchor.BN(2000 * 1_000_000_000); // 2000 tokens
      
      // Set up user with mining history
      const userClaimPda = await setupUserWithMining(user, 1000);
      
      // Create user's token account
      const userTokenAta = await getOrCreateAssociatedTokenAccount(
        provider.connection,
        wallet, // payer
        tokenMint.publicKey,
        user.publicKey
      );
      
      // Claim tokens with Patron role
      const claimTx = await program.methods
        .claimTokensWithRole(claimAmount, { patron: {} })
        .accounts({
          user: user.publicKey,
          userClaim: userClaimPda,
          userTokenAta: userTokenAta.address,
          rewardPoolPda: rewardPoolPda,
          treasuryTokenAccount: treasuryTokenAccount,
          mint: tokenMint.publicKey,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .signers([user])
        .rpc();
        
      console.log("Claim tokens with Patron role tx:", claimTx);
      
      // Verify tokens were transferred and role was set
      const finalBalance = await getAccount(provider.connection, userTokenAta.address);
      const userClaimAccount = await program.account.userClaim.fetch(userClaimPda);
      
      console.log("Final user token balance:", finalBalance.amount.toString());
      console.log("User role:", userClaimAccount.role);
      
      assert.equal(finalBalance.amount.toString(), claimAmount.toString());
      assert.deepEqual(userClaimAccount.role, { patron: {} });
      
      console.log("✅ Successfully claimed tokens with Patron role");
    });

    it("Should prevent claiming tokens if role already selected", async () => {
      console.log("\n=== TEST: Prevent Double Role Selection ===");
      
      const user = Keypair.generate();
      const claimAmount = new anchor.BN(1000 * 1_000_000_000);
      
      // Set up user with mining history
      const userClaimPda = await setupUserWithMining(user, 1000);
      
      // Create user's token account
      const userTokenAta = await getOrCreateAssociatedTokenAccount(
        provider.connection,
        wallet,
        tokenMint.publicKey,
        user.publicKey
      );
      
      // Select role first
      await program.methods
        .selectRole({ staker: {} })
        .accounts({ 
          user: user.publicKey, 
          userClaim: userClaimPda,
        })
        .signers([user])
        .rpc();
      
      // Try to claim tokens (should fail because role is already selected)
      try {
        await program.methods
          .claimTokensWithRole(claimAmount, { patron: {} })
          .accounts({
            user: user.publicKey,
            userClaim: userClaimPda,
            userTokenAta: userTokenAta.address,
            rewardPoolPda: rewardPoolPda,
            treasuryTokenAccount: treasuryTokenAccount,
            mint: tokenMint.publicKey,
            tokenProgram: TOKEN_PROGRAM_ID,
          })
          .signers([user])
          .rpc();
        
        assert.fail("Should have failed to claim tokens when role already selected");
      } catch (error: any) {
        console.log("✅ Correctly prevented claiming tokens with role already selected:", error.message);
        assert.include(error.message, "InvalidRoleTransition");
      }
    });

    it("Should prevent claiming tokens if user not initialized", async () => {
      console.log("\n=== TEST: Prevent Claiming Without Initialization ===");
      
      const user = Keypair.generate();
      const claimAmount = new anchor.BN(1000 * 1_000_000_000);
      
      // Request airdrop
      const airdropSig = await provider.connection.requestAirdrop(
        user.publicKey, 
        2 * LAMPORTS_PER_SOL
      );
      await provider.connection.confirmTransaction(airdropSig);
      
      // Derive userClaim PDA (but don't initialize)
      const [userClaimPda] = PublicKey.findProgramAddressSync(
        [USER_CLAIM_SEED, user.publicKey.toBuffer()],
        program.programId
      );
      
      // Create user's token account
      const userTokenAta = await getOrCreateAssociatedTokenAccount(
        provider.connection,
        wallet,
        tokenMint.publicKey,
        user.publicKey
      );
      
      // Try to claim tokens without initializing (should fail)
      try {
        await program.methods
          .claimTokensWithRole(claimAmount, { staker: {} })
          .accounts({
            user: user.publicKey,
            userClaim: userClaimPda,
            userTokenAta: userTokenAta.address,
            rewardPoolPda: rewardPoolPda,
            treasuryTokenAccount: treasuryTokenAccount,
            mint: tokenMint.publicKey,
            tokenProgram: TOKEN_PROGRAM_ID,
          })
          .signers([user])
          .rpc();
        
        assert.fail("Should have failed to claim tokens without initialization");
      } catch (error: any) {
        console.log("✅ Correctly prevented claiming tokens without initialization");
        // This will fail at the account level since the account doesn't exist
        assert.include(error.message.toLowerCase(), "account");
      }
    });
  });

  describe("Vesting/Locking Tests", () => {
    
    it("Should allow Staker to lock tokens for 3 months", async () => {
      console.log("\n=== TEST: Staker Lock Tokens ===");
      
      const user = Keypair.generate();
      const lockAmount = new anchor.BN(5000 * 1_000_000_000); // 5000 tokens
      
      // Set up user with mining history
      const userClaimPda = await setupUserWithMining(user, 1000);
      
      // Create user's token account and give them tokens
      const userTokenAta = await getOrCreateAssociatedTokenAccount(
        provider.connection,
        wallet,
        tokenMint.publicKey,
        user.publicKey
      );
      
      // Mint tokens to user
      await mintTo(
        provider.connection,
        wallet,
        tokenMint.publicKey,
        userTokenAta.address,
        wallet.publicKey,
        lockAmount.toNumber()
      );
      
      // Select Staker role
      await program.methods
        .selectRole({ staker: {} })
        .accounts({ 
          user: user.publicKey, 
          userClaim: userClaimPda,
        })
        .signers([user])
        .rpc();
      
      // Lock tokens for 3 months
      const lockTx = await program.methods
        .lockTokens(lockAmount, 3) // 3 months
        .accounts({
          user: user.publicKey,
          userClaim: userClaimPda,
          userTokenAccount: userTokenAta.address,
          rewardPoolPda: rewardPoolPda,
          treasuryTokenAccount: treasuryTokenAccount,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .signers([user])
        .rpc();
        
      console.log("Lock tokens tx:", lockTx);
      
      // Verify tokens were locked
      const userClaimAccount = await program.account.userClaim.fetch(userClaimPda);
      assert.equal(userClaimAccount.lockedAmount.toString(), lockAmount.toString());
      assert.equal(userClaimAccount.lockDurationMonths, 3);
      assert.isTrue(userClaimAccount.lockEndTimestamp > userClaimAccount.lockStartTimestamp);
      
      console.log("✅ Successfully locked tokens for 3 months");
    });

    it("Should allow Patron to lock tokens for 6 months", async () => {
      console.log("\n=== TEST: Patron Lock Tokens ===");
      
      const user = Keypair.generate();
      const lockAmount = new anchor.BN(10000 * 1_000_000_000); // 10000 tokens
      
      // Set up user with mining history
      const userClaimPda = await setupUserWithMining(user, 1000);
      
      // Create user's token account and give them tokens
      const userTokenAta = await getOrCreateAssociatedTokenAccount(
        provider.connection,
        wallet,
        tokenMint.publicKey,
        user.publicKey
      );
      
      // Mint tokens to user
      await mintTo(
        provider.connection,
        wallet,
        tokenMint.publicKey,
        userTokenAta.address,
        wallet.publicKey,
        lockAmount.toNumber()
      );
      
      // Apply for patron status and get approved
      await program.methods
        .applyForPatron(30, 50)
        .accounts({
          user: user.publicKey,
          userClaim: userClaimPda,
        })
        .signers([user])
        .rpc();
      
      await program.methods
        .approvePatronApplication(70)
        .accounts({
          admin: wallet.publicKey,
          userClaim: userClaimPda,
          applicant: user.publicKey,
        })
        .rpc();
      
      // Select Patron role
      await program.methods
        .selectRole({ patron: {} })
        .accounts({ 
          user: user.publicKey, 
          userClaim: userClaimPda,
        })
        .signers([user])
        .rpc();
      
      // Lock tokens for 6 months (now that patron is approved)
      const lockTx = await program.methods
        .lockTokens(lockAmount, 6) // 6 months
        .accounts({
          user: user.publicKey,
          userClaim: userClaimPda,
          userTokenAccount: userTokenAta.address,
          rewardPoolPda: rewardPoolPda,
          treasuryTokenAccount: treasuryTokenAccount,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .signers([user])
        .rpc();
        
      console.log("Lock tokens tx:", lockTx);
      
      // Verify tokens were locked
      const userClaimAccount = await program.account.userClaim.fetch(userClaimPda);
      assert.equal(userClaimAccount.lockedAmount.toString(), lockAmount.toString());
      assert.equal(userClaimAccount.lockDurationMonths, 6);
      assert.isTrue(userClaimAccount.lockEndTimestamp > userClaimAccount.lockStartTimestamp);
      
      console.log("✅ Successfully locked tokens for 6 months");
    });
  });

  describe("Role Registry Integration Tests", () => {
    
    it("Should handle complete user journey: Initialize → Select Role → Lock Tokens", async () => {
      console.log("\n=== TEST: Complete User Journey ===");
      
      const user = Keypair.generate();
      const lockAmount = new anchor.BN(3000 * 1_000_000_000); // 3000 tokens
      
      // Set up user with mining history
      console.log("Step 1: Set up user with mining history");
      const userClaimPda = await setupUserWithMining(user, 1000);
      
      // Create user's token account and give them tokens
      const userTokenAta = await getOrCreateAssociatedTokenAccount(
        provider.connection,
        wallet,
        tokenMint.publicKey,
        user.publicKey
      );
      
      await mintTo(
        provider.connection,
        wallet,
        tokenMint.publicKey,
        userTokenAta.address,
        wallet.publicKey,
        lockAmount.toNumber()
      );
      
      let userClaimAccount = await program.account.userClaim.fetch(userClaimPda);
      assert.equal(userClaimAccount.initialized, true);
      assert.deepEqual(userClaimAccount.role, { none: {} });
      console.log("✅ User initialized with None role");
      
      console.log("Step 2: Select Staker role");
      await program.methods
        .selectRole({ staker: {} })
        .accounts({ 
          user: user.publicKey, 
          userClaim: userClaimPda,
        })
        .signers([user])
        .rpc();
      
      userClaimAccount = await program.account.userClaim.fetch(userClaimPda);
      assert.deepEqual(userClaimAccount.role, { staker: {} });
      console.log("✅ User selected Staker role");
      
      console.log("Step 3: Lock tokens for 3 months");
      await program.methods
        .lockTokens(lockAmount, 3)
        .accounts({
          user: user.publicKey,
          userClaim: userClaimPda,
          userTokenAccount: userTokenAta.address,
          rewardPoolPda: rewardPoolPda,
          treasuryTokenAccount: treasuryTokenAccount,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .signers([user])
        .rpc();
      
      userClaimAccount = await program.account.userClaim.fetch(userClaimPda);
      assert.equal(userClaimAccount.lockedAmount.toString(), lockAmount.toString());
      assert.equal(userClaimAccount.lockDurationMonths, 3);
      console.log("✅ User locked tokens for 3 months");
      
      console.log("✅ Complete user journey successful");
    });

    it("Should handle multiple users with different roles", async () => {
      console.log("\n=== TEST: Multiple Users with Different Roles ===");
      
      const users = [
        { keypair: Keypair.generate(), role: { none: {} }, name: "Normal User" },
        { keypair: Keypair.generate(), role: { staker: {} }, name: "Staker" },
        { keypair: Keypair.generate(), role: { patron: {} }, name: "Patron" },
      ];
      
      for (const user of users) {
        // Set up user with mining history
        const userClaimPda = await setupUserWithMining(user.keypair, 1000);
        
        // Select role (skip for None role)
        if (!user.role.none) {
          // Special handling for patron role - need to apply and get approved
          if (user.role.patron) {
            await program.methods
              .applyForPatron(30, 50)
              .accounts({
                user: user.keypair.publicKey,
                userClaim: userClaimPda,
              })
              .signers([user.keypair])
              .rpc();
            
            await program.methods
              .approvePatronApplication(70)
              .accounts({
                admin: wallet.publicKey,
                userClaim: userClaimPda,
                applicant: user.keypair.publicKey,
              })
              .rpc();
          }
          
          await program.methods
            .selectRole(user.role)
            .accounts({ 
              user: user.keypair.publicKey, 
              userClaim: userClaimPda,
            })
            .signers([user.keypair])
            .rpc();
        }
        
        // Verify role
        const userClaimAccount = await program.account.userClaim.fetch(userClaimPda);
        assert.deepEqual(userClaimAccount.role, user.role);
        
        console.log(`✅ ${user.name} successfully set up with role:`, user.role);
      }
      
      console.log("✅ Multiple users with different roles set up successfully");
    });
  });

  after(async () => {
    console.log("\n=== CLEANUP: Test environment ===");
    console.log("All tests completed successfully!");
  });
});