#!/bin/bash

# Database setup script for Snake AI
echo "🗄️ Setting up Snake AI Database..."

# Check if PostgreSQL is installed
if ! command -v psql &> /dev/null; then
    echo "❌ PostgreSQL not found. Installing..."
    
    # Install PostgreSQL based on OS
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        sudo apt update
        sudo apt install -y postgresql postgresql-contrib
        sudo systemctl start postgresql
        sudo systemctl enable postgresql
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        brew install postgresql
        brew services start postgresql
    else
        echo "Please install PostgreSQL manually"
        exit 1
    fi
fi

# Create database and user
echo "📝 Creating database..."
sudo -u postgres psql << EOF
CREATE DATABASE snakeai_db;
CREATE USER snakeai_user WITH ENCRYPTED PASSWORD 'snakeai_pass';
GRANT ALL PRIVILEGES ON DATABASE snakeai_db TO snakeai_user;
\q
EOF

# Update backend .env with database URL
sed -i 's|DATABASE_URL=.*|DATABASE_URL=postgresql://snakeai_user:snakeai_pass@localhost:5432/snakeai_db|' backend/.env

echo "✅ Database setup complete!"
echo "📋 Database Details:"
echo "  Database: snakeai_db"
echo "  User: snakeai_user"
echo "  Password: snakeai_pass"
echo ""
echo "🔄 Next: Run migrations with 'cd backend && sqlx migrate run'"
