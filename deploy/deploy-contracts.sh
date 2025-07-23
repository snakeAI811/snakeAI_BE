#!/bin/bash

# Phase 2 Smart Contract Deployment Script
# This script deploys the snake_contract to Solana mainnet

set -e

echo "🚀 Starting Phase 2 Smart Contract Deployment to Mainnet"

# Check if Anchor is installed
if ! command -v anchor &> /dev/null; then
    echo "❌ Anchor CLI not found. Please install Anchor first."
    exit 1
fi

# Check if solana CLI is installed
if ! command -v solana &> /dev/null; then
    echo "❌ Solana CLI not found. Please install Solana CLI first."
    exit 1
fi

# Set Solana network to mainnet
echo "🔧 Setting Solana network to mainnet-beta..."
solana config set --url https://api.mainnet-beta.solana.com

# Check wallet balance
echo "💰 Checking wallet balance..."
BALANCE=$(solana balance --lamports)
MIN_BALANCE=5000000000  # 5 SOL in lamports

if [ "$BALANCE" -lt "$MIN_BALANCE" ]; then
    echo "❌ Insufficient SOL balance. Need at least 5 SOL for deployment."
    echo "Current balance: $(solana balance)"
    exit 1
fi

echo "✅ Sufficient balance: $(solana balance)"

# Build the program
echo "🔨 Building smart contract..."
anchor build

# Generate a new program ID if needed
echo "🆔 Checking program ID..."
PROGRAM_ID=$(solana-keygen pubkey target/deploy/snake_contract-keypair.json 2>/dev/null || echo "")

if [ -z "$PROGRAM_ID" ]; then
    echo "🆔 Generating new program keypair..."
    solana-keygen new --outfile target/deploy/snake_contract-keypair.json --no-bip39-passphrase
    PROGRAM_ID=$(solana-keygen pubkey target/deploy/snake_contract-keypair.json)
fi

echo "Program ID: $PROGRAM_ID"

# Update Anchor.toml with the program ID
echo "📝 Updating Anchor.toml with program ID..."
sed -i.bak "s/snake_contract = \".*\"/snake_contract = \"$PROGRAM_ID\"/" Anchor.toml

# Rebuild with updated program ID
echo "🔨 Rebuilding with updated program ID..."
anchor build

# Deploy to mainnet
echo "🚀 Deploying to mainnet..."
anchor deploy --provider.cluster mainnet

# Verify deployment
echo "✅ Verifying deployment..."
solana program show "$PROGRAM_ID"

echo ""
echo "🎉 Smart Contract Deployment Complete!"
echo "📋 Deployment Summary:"
echo "   Program ID: $PROGRAM_ID"
echo "   Network: Mainnet Beta"
echo "   Deployer: $(solana address)"
echo ""
echo "⚠️  IMPORTANT: Save this Program ID for your backend configuration!"
echo "   Add this to your .env file:"
echo "   PROGRAM_ID=$PROGRAM_ID"
echo ""
echo "🔗 View on Solana Explorer:"
echo "   https://explorer.solana.com/address/$PROGRAM_ID"

# Create deployment record
cat > deployment-record.json << EOF
{
  "program_id": "$PROGRAM_ID",
  "network": "mainnet-beta",
  "deployed_at": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
  "deployer": "$(solana address)",
  "anchor_version": "$(anchor --version)"
}
EOF

echo "📁 Deployment record saved to deployment-record.json"
