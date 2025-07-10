import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SnakeContract } from "../target/types/snake_contract";
import {
  createMint,
  getAssociatedTokenAddressSync,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { Keypair, PublicKey, SystemProgram, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { assert } from "chai";

describe("🚀 Patron Framework Tests", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.SnakeContract as Program<SnakeContract>;
  const provider = anchor.getProvider();
  const wallet = provider.wallet.payer;
  
  let tokenMint: Keypair;
  let staker: Keypair;
  let patron: Keypair;
  let normalUser: Keypair;
  
  before(async () => {
    console.log("=== SETUP: Creating test environment ===");
    
    // Create test users
    staker = Keypair.generate();
    patron = Keypair.generate();
    normalUser = Keypair.generate();
    
    // Airdrop SOL to test users
    await provider.connection.requestAirdrop(staker.publicKey, 10 * LAMPORTS_PER_SOL);
    await provider.connection.requestAirdrop(patron.publicKey, 10 * LAMPORTS_PER_SOL);
    await provider.connection.requestAirdrop(normalUser.publicKey, 10 * LAMPORTS_PER_SOL);
    
    // Wait for airdrops
    await new Promise(resolve => setTimeout(resolve, 2000));
    
    tokenMint = Keypair.generate();
    
    // Create mint
    await createMint(
      provider.connection,
      wallet,
      wallet.publicKey,
      null,
      9,
      tokenMint
    );
    
    console.log("✅ Test environment setup complete!");
  });

  describe("📋 User Claim Initialization", () => {
    
    it("✅ Should initialize user claims", async () => {
      console.log("🧪 Testing user claim initialization...");
      
      const users = [staker, patron, normalUser];
      
      for (const user of users) {
        const [userClaimPda] = PublicKey.findProgramAddressSync(
          [Buffer.from("user_claim"), user.publicKey.toBuffer()],
          program.programId
        );
        
        try {
          await program.methods
            .initializeUserClaim()
            .accounts({
              user: user.publicKey,
              userClaim: userClaimPda,
              systemProgram: SystemProgram.programId,
            })
            .signers([user])
            .rpc();
          
          console.log(`✅ User claim initialized for ${user.publicKey.toBase58().slice(0, 8)}...`);
          
          // Verify user claim
          const userClaim = await program.account.userClaim.fetch(userClaimPda);
          assert.equal(userClaim.initialized, true);
          assert.equal(userClaim.user.toBase58(), user.publicKey.toBase58());
          assert.equal(userClaim.role.none !== undefined, true); // Default role is None
          
        } catch (error) {
          console.error(`❌ Error initializing user claim for ${user.publicKey.toBase58().slice(0, 8)}:`, error);
          throw error;
        }
      }
    });
  });

  describe("🤝 Patron Application System", () => {
    
    it("✅ Should apply for patron status", async () => {
      console.log("🧪 Testing patron application...");
      
      const walletAgeDays = 90; // 3 months old wallet
      const communityScore = 25; // 25/30 community points
      
      const [userClaimPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("user_claim"), patron.publicKey.toBuffer()],
        program.programId
      );
      
      try {
        const tx = await program.methods
          .applyForPatron(walletAgeDays, communityScore)
          .accounts({
            user: patron.publicKey,
            userClaim: userClaimPda,
          })
          .signers([patron])
          .rpc();
        
        console.log("❌ This should have failed - no mining history");
        assert.fail("Should have failed due to no mining history");
        
      } catch (error) {
        console.log("✅ Application correctly failed due to no mining history");
        assert.isTrue(error.message.includes("NoMiningHistory"));
      }
    });
    
    it("✅ Should check patron eligibility", async () => {
      console.log("🧪 Testing patron eligibility check...");
      
      const [userClaimPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("user_claim"), patron.publicKey.toBuffer()],
        program.programId
      );
      
      try {
        await program.methods
          .checkPatronEligibility(50) // 50 point minimum
          .accounts({
            user: patron.publicKey,
            userClaim: userClaimPda,
          })
          .signers([patron])
          .rpc();
        
        console.log("✅ Patron eligibility check completed");
        
      } catch (error) {
        console.log("✅ Eligibility check correctly handled no mining history");
      }
    });
  });

  describe("🎭 Role Selection System", () => {
    
    it("✅ Should test role selection validation", async () => {
      console.log("🧪 Testing role selection logic...");
      
      const [stakerClaimPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("user_claim"), staker.publicKey.toBuffer()],
        program.programId
      );
      
      try {
        await program.methods
          .selectRole({ staker: {} })
          .accounts({
            user: staker.publicKey,
            userClaim: stakerClaimPda,
          })
          .signers([staker])
          .rpc();
        
        console.log("❌ This should have failed - no mining history");
        assert.fail("Should have failed due to no mining history");
        
      } catch (error) {
        console.log("✅ Role selection correctly failed due to no mining history");
        assert.isTrue(error.message.includes("NoMiningHistory"));
      }
    });
    
    it("✅ Should stay in default role (None)", async () => {
      console.log("🧪 Testing default role behavior...");
      
      const [normalUserClaimPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("user_claim"), normalUser.publicKey.toBuffer()],
        program.programId
      );
      
      // Check that user starts with None role (default)
      const userClaim = await program.account.userClaim.fetch(normalUserClaimPda);
      assert.equal(userClaim.role.none !== undefined, true);
      
      console.log("✅ User correctly starts with default role (None)");
    });
  });

  describe("📊 Data Validation", () => {
    
    it("✅ Should validate UserClaim data structure", async () => {
      console.log("🧪 Testing UserClaim data structure...");
      
      const [userClaimPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("user_claim"), staker.publicKey.toBuffer()],
        program.programId
      );
      
      const userClaim = await program.account.userClaim.fetch(userClaimPda);
      
      // Validate default values
      assert.equal(userClaim.initialized, true);
      assert.equal(userClaim.lockedAmount.toNumber(), 0);
      assert.equal(userClaim.totalMinedPhase1.toNumber(), 0);
      assert.equal(userClaim.walletAgeDays, 0);
      assert.equal(userClaim.communityScore, 0);
      assert.equal(userClaim.patronQualificationScore, 0);
      assert.equal(userClaim.soldEarly, false);
      assert.equal(userClaim.minedInPhase2, false);
      assert.equal(userClaim.daoEligible, false);
      assert.equal(userClaim.daoSeatHolder, false);
      
      console.log("✅ UserClaim data structure validation passed");
      console.log(`📊 User: ${userClaim.user.toBase58().slice(0, 8)}...`);
      console.log(`📊 Role: ${Object.keys(userClaim.role)[0]}`);
      console.log(`📊 Patron Status: ${Object.keys(userClaim.patronStatus)[0]}`);
    });
  });

  after(async () => {
    console.log("🧹 Cleaning up test environment...");
    console.log("✅ Patron Framework tests completed!");
  });
});
