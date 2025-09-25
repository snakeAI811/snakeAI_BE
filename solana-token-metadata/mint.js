import { createUmi } from '@metaplex-foundation/umi-bundle-defaults';
import { mplTokenMetadata } from '@metaplex-foundation/mpl-token-metadata';
import { irysUploader } from '@metaplex-foundation/umi-uploader-irys';
import { generateSigner, keypairIdentity, percentAmount } from '@metaplex-foundation/umi';
import { createFungible } from '@metaplex-foundation/mpl-token-metadata';
import fs from 'fs';

const umi = createUmi('https://api.devnet.solana.com')
  .use(mplTokenMetadata())
  .use(irysUploader());

// Load or generate wallet
const walletFile = JSON.parse(fs.readFileSync('/home/super/.config/solana/devnet.json', 'utf-8'));
const keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(walletFile));
umi.use(keypairIdentity(keypair));

// Airdrop SOL if needed
// await umi.rpc.airdrop(umi.identity.publicKey, sol(1));

async function updateTokenMetadata() {
  // Use existing token mint address
  const mint = { publicKey: '76WnJcKdr9RvTADjgA9ZqaV6gsjCFy1oJT3VLv6AJpR' };

  // Upload icon to Arweave
  const iconFile = fs.readFileSync('./SNAKE.png');
  const iconUri = await umi.uploader.upload([iconFile]);

  // Create metadata
  await createFungible(umi, {
    mint,
    name: 'SNAKEAI',
    symbol: '$SSS',
    uri: iconUri[0], // Or metadata JSON URI
    sellerFeeBasisPoints: percentAmount(0),
  }).sendAndConfirm(umi);

  console.log(`Metadata updated for token: ${mint.publicKey}`);
}

updateTokenMetadata();