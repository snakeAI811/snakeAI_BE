import { createUmi } from '@metaplex-foundation/umi-bundle-defaults';
import { mplTokenMetadata, updateV1 } from '@metaplex-foundation/mpl-token-metadata';
import { irysUploader } from '@metaplex-foundation/umi-uploader-irys';
import { keypairIdentity, percentAmount, publicKey } from '@metaplex-foundation/umi';
import { readFileSync } from 'fs';
import {
  fetchMetadataFromSeeds,
} from '@metaplex-foundation/mpl-token-metadata'; 

(async () => {
  try {
    // Initialize Umi for Devnet
    const umi = createUmi('https://api.devnet.solana.com')
      .use(mplTokenMetadata())
      .use(irysUploader());

    // Load wallet
    const walletFile = JSON.parse(readFileSync('/home/super/.config/solana/devnet.json', 'utf-8'));
    const keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(walletFile));
    umi.use(keypairIdentity(keypair));

    // Token mint address
    const mint = publicKey('76WnJcKdr9RvTADjgA9ZqaV6gsjCFy1oJT3VLv6AJpR');

    // Fetch existing metadata
    const metadata = await fetchMetadataFromSeeds(umi, { mint });

    // Upload new icon to Arweave
    const iconFile = readFileSync('./SNAKE.png');
    const [iconUri] = await umi.uploader.upload([iconFile]);

    // Update metadata with preserved creators
    await updateV1(umi, {
      mint,
      authority: umi.identity,
      data: {
        name: 'SNAKEAI',
        symbol: '$SSS',
        uri: iconUri,
        sellerFeeBasisPoints: percentAmount(0),
        creators: metadata.creators, // <- Reuse existing verified creators
      },
    }).sendAndConfirm(umi);

    console.log(`Metadata updated for token: ${mint}`);
  } catch (error) {
    console.error('Error:', error);
    if (error.logs) console.error('Transaction Logs:', error.logs);
  }
})();