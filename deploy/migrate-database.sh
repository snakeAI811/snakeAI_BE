#!/bin/bash

# Database Migration Script for Phase 2 Production
set -e

echo "🗄️  Starting Database Migration for Phase 2 Production"

# Check if environment variables are set
if [ -z "$DATABASE_URL" ]; then
    echo "❌ DATABASE_URL environment variable is not set"
    exit 1
fi

# Check if sqlx CLI is installed
if ! command -v sqlx &> /dev/null; then
    echo "🔧 Installing sqlx CLI..."
    cargo install sqlx-cli --no-default-features --features postgres
fi

echo "✅ sqlx CLI ready"

# Create database if it doesn't exist
echo "🏗️  Creating database if needed..."
sqlx database create

# Run migrations
echo "📋 Running database migrations..."
sqlx migrate run --source migrations

echo "✅ Database migrations completed successfully!"

# Verify migration status
echo "📊 Migration status:"
sqlx migrate info --source migrations

# Create initial data if needed (Phase 2 configuration)
echo "🔧 Setting up Phase 2 initial data..."

# Create SQL script for Phase 2 initial data
cat > /tmp/phase2_init.sql << 'EOF'
-- Phase 2 Initial Data Setup

-- Insert Phase 2 configuration
INSERT INTO system_config (key, value, description) 
VALUES 
  ('phase2_start_date', '2025-01-25T00:00:00Z', 'Phase 2 launch date'),
  ('phase2_enabled', 'true', 'Phase 2 features enabled'),
  ('patron_mining_multiplier', '2.0', 'Patron mining reward multiplier'),
  ('staker_mining_multiplier', '1.5', 'Staker mining reward multiplier')
ON CONFLICT (key) DO UPDATE SET 
  value = EXCLUDED.value,
  updated_at = NOW();

-- Create admin user for Phase 2 management
INSERT INTO users (id, twitter_id, username, email, role, created_at, updated_at)
VALUES 
  ('admin-phase2', 'admin_twitter_id', 'phase2_admin', 'admin@snakeai.com', 'admin', NOW(), NOW())
ON CONFLICT (twitter_id) DO NOTHING;

-- Set up reward pools for Phase 2
INSERT INTO reward_pools (name, total_tokens, distributed_tokens, pool_type, phase, created_at)
VALUES 
  ('Phase 2 Patron Pool', 10000000000000, 0, 'patron', 2, NOW()),
  ('Phase 2 Staker Pool', 5000000000000, 0, 'staker', 2, NOW())
ON CONFLICT (name) DO NOTHING;

-- Log Phase 2 initialization
INSERT INTO system_logs (level, message, context, created_at)
VALUES 
  ('INFO', 'Phase 2 database initialization completed', '{"migration": "phase2_init", "version": "2.0"}', NOW());

EOF

# Execute the initial data script
echo "💾 Inserting Phase 2 initial data..."
psql "$DATABASE_URL" -f /tmp/phase2_init.sql

# Clean up temp file
rm /tmp/phase2_init.sql

echo ""
echo "🎉 Database Migration Complete!"
echo "📋 Summary:"
echo "   ✅ Database created and migrated"
echo "   ✅ Phase 2 configuration inserted"
echo "   ✅ Initial reward pools created"
echo "   ✅ Admin user configured"
echo ""
echo "🔗 Database URL: $DATABASE_URL"
echo "📊 To verify, run: sqlx migrate info --source migrations"
