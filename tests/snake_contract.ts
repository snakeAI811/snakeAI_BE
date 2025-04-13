import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SnakeContract } from "../target/types/snake_contract";
import {
  createMint,
  getAccount,
  getAssociatedTokenAddressSync,
  getOrCreateAssociatedTokenAccount,
  mintTo,
} from "@solana/spl-token";
import { Keypair } from "@solana/web3.js";
import { assert, use } from "chai";

const REWARD_POOL_SEED = Buffer.from("reward_pool");
const USER_CLAIM_SEED = Buffer.from("user_claim");

describe("snake_contract", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.snakeContract as Program<SnakeContract>;

  const provider = anchor.getProvider();
  const wallet = provider.wallet.payer;
  const tokenMint = Keypair.generate();

  it("create token", async () => {
    await createMint(
      provider.connection,
      wallet,
      wallet.publicKey,
      null,
      9,
      tokenMint
    );
  });

  it("mint token to wallet", async () => {
    const recipientAssociatedTokenAccount =
      await getOrCreateAssociatedTokenAccount(
        provider.connection,
        wallet,
        tokenMint.publicKey,
        wallet.publicKey
      );

    await mintTo(
      provider.connection,
      wallet,
      tokenMint.publicKey,
      recipientAssociatedTokenAccount.address,
      wallet,
      BigInt(1000000000) * BigInt(1000000000)
    );
  });

  it("Initialize reward pool", async () => {
    const tx = await program.methods
      .initializeRewardPool({
        admin: wallet.publicKey
      })
      .accounts({
        owner: wallet.publicKey,
        mint: tokenMint.publicKey,
      })
      .rpc();

    const rewardPool = anchor.web3.PublicKey.findProgramAddressSync(
      [REWARD_POOL_SEED],
      program.programId
    )[0];

    const treasuryAta = getAssociatedTokenAddressSync(
      tokenMint.publicKey,
      rewardPool,
      true
    );

    const treasury = await getAccount(provider.connection, treasuryAta);
    assert(treasury.amount === BigInt(500000000) * BigInt(1000000000));

    console.log("initializeRewardPool transaction: ", tx);
  });

  it("Claim reward", async () => {
    // Add your test here.
    const user = Keypair.generate();
    let signature = await provider.connection.requestAirdrop(
      user.publicKey,
      1_000_000_000
    );
    await provider.connection.confirmTransaction(signature);

    const tx = await program.methods
      .claimReward()
      .accounts({
        user: user.publicKey,
      })
      .transaction();

    // tx.recentBlockhash = (await provider.connection.getLatestBlockhash()).blockhash;
    // tx.feePayer = user.publicKey;

    console.log(user.publicKey);
    console.log(wallet.publicKey);

    tx.partialSign(wallet);

    // await new Promise(resolve => setTimeout(resolve, 1000));

    tx.partialSign(user);

    console.log(user.publicKey);
    console.log(wallet.publicKey);

    signature = await provider.connection.sendTransaction(tx, [user, wallet]);
    await provider.connection.confirmTransaction(signature);

    const rewardPool = anchor.web3.PublicKey.findProgramAddressSync(
      [REWARD_POOL_SEED],
      program.programId
    )[0];

    const treasuryAta = getAssociatedTokenAddressSync(
      tokenMint.publicKey,
      rewardPool,
      true
    );

    const estimateRewardAmount = 625;
    const treasury = await getAccount(provider.connection, treasuryAta);
    assert(
      treasury.amount ===
        BigInt(500000000 - estimateRewardAmount * 2) * BigInt(1000000000)
    );

    const userAta = getAssociatedTokenAddressSync(
      tokenMint.publicKey,
      user.publicKey
    );

    const userToken = await getAccount(provider.connection, userAta);
    assert(
      userToken.amount === BigInt(estimateRewardAmount) * BigInt(1000000000)
    );
  });
});
