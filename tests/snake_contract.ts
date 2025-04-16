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
import { Keypair, PublicKey, Transaction, sendAndConfirmRawTransaction, sendAndConfirmTransaction } from "@solana/web3.js";
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
    return;
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
    return;
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
    return;
    const tokenMint = new anchor.web3.PublicKey("E1BHSRCrWvBe1hVBKjHvUbaA8H2QGWttQva14xr2DEJJ");
    const tx = await program.methods
      .initializeRewardPool({
        admin: wallet.publicKey
      })
      .accounts({
        owner: wallet.publicKey,
        mint: tokenMint
      })
      .rpc();

    const rewardPool = anchor.web3.PublicKey.findProgramAddressSync(
      [REWARD_POOL_SEED],
      program.programId
    )[0];

    const treasuryAta = getAssociatedTokenAddressSync(
      tokenMint,
      rewardPool,
      true
    );

    const treasury = await getAccount(provider.connection, treasuryAta);
    assert(treasury.amount === BigInt(500000000) * BigInt(1000000000));

    console.log("initializeRewardPool transaction: ", tx);
  });

  it("Claim reward", async () => {
    return;
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

  // it("Claim reward with signed transaction", async () => {
  //   // Add your test here.
  //   const tx = await program.methods
  //     .updateRewardPool({
  //       admin: new anchor.web3.PublicKey("CPtWsrTiHV8sLHd94JmTUo86znBbruV1EHbo6VdMnPPR")
  //     })
  //     .rpc();
  //   console.log(tx);
  // });

  it("Claim reward with signed transaction", async () => {
    // Add your test here.
    const serializedTransaction = "AgAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAADDfrhi6j1P7R6iexIb/IhkD0eU+IEopJBvUPe8DPojVOZj7SoJnolml52IjZ49tvlZmoE+VUcfwoVSN3W5Hn4NAgAEC0HI9aDPJ0FsCI389jkfUTRGSXzJSW1w0JYKqcTh4/k/qU2VEsaKAIKfX6NUGyg2d+rtMBUDs3+xx5zMoVw/HHQkpYslhDlqEQ0o+g2vtJmz6Fn3eCccG6KXwH2En01ChVtA+uoETCtZzdMSLp5jueGd0DlCIVk2yMu9SlKh6jFOwTOOuvK3kpH1nQdLsT3qcH5X412PoguSbEnu4pN+po/bq+4JytIEz64NDti95cFxoVtVF0+J+JAyYkHTj8ENj/8AIjptDl4193u8NDa+Y95Fws0faRFAE1jYBRgnf8SWAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAG3fbh12Whk9nL4UbO63msHLSF7V9bN5E6jPWFfv8AqYyXJY9OJInxuz0QKRSODYMLWhOZ2v8QhASOe9jb6fhZk5MOsDlx+UZTeCPxiV0AZieao9r6G5WGMp+TRZBctVrNKyKMz4j4GYVUBU927c5Pjr0ZZcpSmcWpQEo2mhUPAgEKCgABAwYCBQQJCAcIlV+18l5anqI=";
    const transaction = Transaction.from(Buffer.from(serializedTransaction, 'base64'));
    console.log(transaction.signatures)
    transaction.partialSign(wallet);
    console.log(transaction.signatures)
    const signature = await sendAndConfirmRawTransaction(provider.connection, transaction.serialize());
    console.log(signature);
  });
});
