#!/bin/bash

# Phase 2 Production Deployment Orchestration Script
set -e

echo "🚀 Phase 2 Production Deployment Starting..."
echo "================================================"

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

log_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

log_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

log_error() {
    echo -e "${RED}❌ $1${NC}"
}

# Check prerequisites
check_prerequisites() {
    log_info "Checking prerequisites..."
    
    # Check Docker
    if ! command -v docker &> /dev/null; then
        log_error "Docker is not installed"
        exit 1
    fi
    
    # Check Docker Compose
    if ! command -v docker-compose &> /dev/null; then
        log_error "Docker Compose is not installed"
        exit 1
    fi
    
    # Check environment file
    if [ ! -f .env ]; then
        log_error ".env file not found. Please create it with production settings."
        exit 1
    fi
    
    # Check Phase 2 date in env
    if ! grep -q "PHASE2_START_DATE=2025-01-25T00:00:00Z" .env; then
        log_warning "PHASE2_START_DATE not set to 2025-01-25T00:00:00Z in .env"
        log_info "Please ensure your .env contains: PHASE2_START_DATE=2025-01-25T00:00:00Z"
    fi
    
    log_success "Prerequisites check passed"
}

# Deploy smart contracts
deploy_contracts() {
    log_info "Deploying smart contracts to mainnet..."
    
    if [ -f "deploy/deploy-contracts.sh" ]; then
        cd deploy
        ./deploy-contracts.sh
        cd ..
        log_success "Smart contracts deployed successfully"
    else
        log_error "Contract deployment script not found"
        exit 1
    fi
}

# Migrate database
migrate_database() {
    log_info "Running database migrations..."
    
    if [ -f "deploy/migrate-database.sh" ]; then
        cd deploy
        ./migrate-database.sh
        cd ..
        log_success "Database migrations completed"
    else
        log_error "Database migration script not found"
        exit 1
    fi
}

# Deploy services
deploy_services() {
    log_info "Deploying backend and frontend services..."
    
    # Stop existing services
    docker-compose -f deploy/docker-compose.prod.yml down
    
    # Build and start services
    docker-compose -f deploy/docker-compose.prod.yml up --build -d
    
    # Wait for services to start
    log_info "Waiting for services to start..."
    sleep 30
    
    # Health check
    if curl -f http://localhost:8000/ > /dev/null 2>&1; then
        log_success "Backend service is running"
    else
        log_error "Backend service failed to start"
        docker-compose -f deploy/docker-compose.prod.yml logs backend
        exit 1
    fi
    
    if curl -f http://localhost:3000/ > /dev/null 2>&1; then
        log_success "Frontend service is running"
    else
        log_error "Frontend service failed to start"
        docker-compose -f deploy/docker-compose.prod.yml logs frontend
        exit 1
    fi
    
    log_success "All services deployed successfully"
}

# Main deployment process
main() {
    echo "Starting Phase 2 Production Deployment..."
    echo "Timestamp: $(date)"
    echo ""
    
    # Step 1: Prerequisites
    check_prerequisites
    echo ""
    
    # Step 2: Database Migration
    log_info "Step 1/3: Database Migration"
    migrate_database
    echo ""
    
    # Step 3: Smart Contract Deployment
    log_info "Step 2/3: Smart Contract Deployment"
    read -p "Deploy smart contracts to mainnet? This will cost SOL. (y/N): " -n 1 -r
    echo ""
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        deploy_contracts
    else
        log_warning "Skipping smart contract deployment"
    fi
    echo ""
    
    # Step 4: Service Deployment
    log_info "Step 3/3: Service Deployment"
    deploy_services
    echo ""
    
    # Success summary
    echo "================================================"
    log_success "🎉 Phase 2 Production Deployment Complete!"
    echo ""
    echo "📋 Deployment Summary:"
    echo "   🗄️  Database: Migrated with Phase 2 schema"
    echo "   🔗 Smart Contracts: $([ -f deployment-record.json ] && echo "Deployed to mainnet" || echo "Skipped")"
    echo "   🖥️  Backend: http://localhost:8000"
    echo "   🌐 Frontend: http://localhost:3000"
    echo "   📅 Phase 2 Start: 2025-01-25T00:00:00Z"
    echo ""
    echo "🔧 Next Steps:"
    echo "   1. Configure your domain and SSL certificates"
    echo "   2. Set up monitoring and logging"
    echo "   3. Configure backup schedules"
    echo "   4. Update DNS records to point to your server"
    echo ""
    echo "📊 To check service status:"
    echo "   docker-compose -f deploy/docker-compose.prod.yml ps"
    echo ""
    echo "📝 To view logs:"
    echo "   docker-compose -f deploy/docker-compose.prod.yml logs -f"
}

# Execute main function
main "$@"
