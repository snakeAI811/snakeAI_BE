#!/bin/bash

# Setup script for Snake AI Smart Contract Integration
echo "🐍 Setting up Snake AI Smart Contract..."

# Contract details
PROGRAM_ID="7atsSKAcDXELnLGjvNX27ke8wdC8XUpsG1bJciCj1pQZ"
NETWORK="devnet"

echo "📋 Contract Information:"
echo "  Program ID: $PROGRAM_ID"
echo "  Network: $NETWORK"
echo ""

# Check Solana CLI
# if ! command -v solana &> /dev/null; then
#     echo "❌ Solana CLI not found. Please install it first."
#     exit 1
# fi

# Check current network
CURRENT_NETWORK=$(solana config get | grep "RPC URL" | awk '{print $3}')
echo "🌐 Current RPC: $CURRENT_NETWORK"

# Create token mint
echo "🪙 Creating token mint..."
TOKEN_MINT=$(spl-token create-token --decimals 9 | grep "Creating token" | awk '{print $3}')

if [ -z "$TOKEN_MINT" ]; then
    echo "❌ Failed to create token mint"
    exit 1
fi

echo "✅ Token mint created: $TOKEN_MINT"

# Create token account for your wallet
echo "💰 Creating token account..."
TOKEN_ACCOUNT=$(spl-token create-account $TOKEN_MINT | grep "Creating account" | awk '{print $3}')
echo "✅ Token account created: $TOKEN_ACCOUNT"

# Mint initial supply (100M tokens)
echo "🏭 Minting initial supply..."
spl-token mint $TOKEN_MINT 100000000 $TOKEN_ACCOUNT
echo "✅ Minted 100M tokens to $TOKEN_ACCOUNT"

# Update environment files
echo "📝 Updating configuration files..."

# Update backend .env
sed -i "s/TOKEN_MINT=.*/TOKEN_MINT=$TOKEN_MINT/" backend/.env
echo "✅ Updated backend/.env with token mint"

# Update frontend .env  
sed -i "s/REACT_APP_TOKEN_MINT=.*/REACT_APP_TOKEN_MINT=$TOKEN_MINT/" frontend/.env
echo "✅ Updated frontend/.env with token mint"

echo ""
echo "🎉 Setup Complete!"
echo ""
echo "📋 Configuration Summary:"
echo "  Program ID: $PROGRAM_ID"
echo "  Token Mint: $TOKEN_MINT"
echo "  Token Account: $TOKEN_ACCOUNT"
echo "  Network: $NETWORK"
echo ""
echo "📝 Next Steps:"
echo "  1. Update your backend/.env with Twitter API keys"
echo "  2. Setup PostgreSQL database"
echo "  3. Run: cd backend && cargo run --bin server"
echo "  4. Run: cd frontend && npm start"
echo ""
echo "🐦 Twitter Mining Ready!"
