#!/bin/bash

# Decentralized Autonomous University Deployment Script
# This script deploys all canisters to the Internet Computer

set -e  # Exit on error

echo "🎓 Deploying Decentralized Autonomous University Platform"
echo "================================================"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check prerequisites
print_status "Checking prerequisites..."

if ! command -v dfx &> /dev/null; then
    print_error "DFX is not installed. Please install DFX first."
    exit 1
fi

if ! command -v cargo &> /dev/null; then
    print_error "Rust/Cargo is not installed. Please install Rust first."
    exit 1
fi

# Check if WASM target is installed
if ! rustup target list | grep -q "wasm32-unknown-unknown (installed)"; then
    print_warning "WASM target not installed. Installing..."
    rustup target add wasm32-unknown-unknown
fi

print_success "Prerequisites check completed"

# Get network parameter
NETWORK=${1:-local}
if [ "$NETWORK" != "local" ] && [ "$NETWORK" != "ic" ]; then
    print_error "Invalid network specified. Use 'local' or 'ic'"
    exit 1
fi

print_status "Deploying to network: $NETWORK"

# Start local replica if deploying locally
if [ "$NETWORK" = "local" ]; then
    print_status "Starting local IC replica..."
    dfx start --background --clean
    
    # Wait for replica to be ready
    print_status "Waiting for replica to be ready..."
    sleep 5
fi

# Build all canisters
print_status "Building all canisters..."
if ! dfx build --network $NETWORK; then
    print_error "Build failed!"
    exit 1
fi
print_success "All canisters built successfully"

# Deploy canisters in order (dependencies first)
print_status "Deploying canisters..."

# Deploy User Management first (no dependencies)
print_status "Deploying User Management canister..."
if ! dfx deploy user_management --network $NETWORK; then
    print_error "Failed to deploy User Management canister"
    exit 1
fi
print_success "User Management canister deployed"

# Deploy Course Management
print_status "Deploying Course Management canister..."
if ! dfx deploy course_management --network $NETWORK; then
    print_error "Failed to deploy Course Management canister"
    exit 1
fi
print_success "Course Management canister deployed"

# Deploy Certification System
print_status "Deploying Certification System canister..."
if ! dfx deploy certification_system --network $NETWORK; then
    print_error "Failed to deploy Certification System canister"
    exit 1
fi
print_success "Certification System canister deployed"

# Deploy Governance
print_status "Deploying Governance canister..."
if ! dfx deploy governance --network $NETWORK; then
    print_error "Failed to deploy Governance canister"
    exit 1
fi
print_success "Governance canister deployed"

# Deploy frontend assets if available
if [ -d "src/decentralized_university_frontend" ]; then
    print_status "Deploying frontend assets..."
    if ! dfx deploy decentralized_university_frontend --network $NETWORK; then
        print_error "Failed to deploy frontend assets"
        exit 1
    fi
    print_success "Frontend assets deployed"
fi

# Get canister IDs and display them
print_status "Retrieving canister information..."

USER_MANAGEMENT_ID=$(dfx canister id user_management --network $NETWORK)
COURSE_MANAGEMENT_ID=$(dfx canister id course_management --network $NETWORK)
CERTIFICATION_SYSTEM_ID=$(dfx canister id certification_system --network $NETWORK)
GOVERNANCE_ID=$(dfx canister id governance --network $NETWORK)

echo ""
echo "🎉 Deployment completed successfully!"
echo "====================================="
echo ""
echo "📋 Canister Information:"
echo "------------------------"
echo "User Management:      $USER_MANAGEMENT_ID"
echo "Course Management:    $COURSE_MANAGEMENT_ID"
echo "Certification System: $CERTIFICATION_SYSTEM_ID"
echo "Governance:           $GOVERNANCE_ID"
echo ""

if [ "$NETWORK" = "local" ]; then
    echo "🌐 Local URLs:"
    echo "-------------"
    echo "Candid UI: http://localhost:4943/?canisterId=$(dfx canister id __Candid_UI --network local)"
    echo "User Management: http://localhost:4943/?canisterId=$USER_MANAGEMENT_ID"
    echo "Course Management: http://localhost:4943/?canisterId=$COURSE_MANAGEMENT_ID"
    echo "Certification System: http://localhost:4943/?canisterId=$CERTIFICATION_SYSTEM_ID"
    echo "Governance: http://localhost:4943/?canisterId=$GOVERNANCE_ID"
    echo ""
fi

if [ "$NETWORK" = "ic" ]; then
    echo "🌍 Mainnet URLs:"
    echo "---------------"
    echo "User Management: https://$USER_MANAGEMENT_ID.ic0.app"
    echo "Course Management: https://$COURSE_MANAGEMENT_ID.ic0.app"
    echo "Certification System: https://$CERTIFICATION_SYSTEM_ID.ic0.app"
    echo "Governance: https://$GOVERNANCE_ID.ic0.app"
    echo ""
fi

# Save deployment info to file
cat > deployment-info.txt << EOF
Decentralized Autonomous University Deployment Information
=========================================================

Network: $NETWORK
Deployment Date: $(date)

Canister IDs:
- User Management: $USER_MANAGEMENT_ID
- Course Management: $COURSE_MANAGEMENT_ID
- Certification System: $CERTIFICATION_SYSTEM_ID
- Governance: $GOVERNANCE_ID

Next Steps:
1. Test the canisters using the Candid UI or dfx commands
2. Create an admin user for initial setup
3. Configure governance parameters
4. Set up the first instructors and courses
5. Deploy and configure the frontend application

Example Commands:
- Create user: dfx canister call user_management create_user --network $NETWORK
- Get governance stats: dfx canister call governance get_governance_stats --network $NETWORK
- View all proposals: dfx canister call governance get_active_proposals --network $NETWORK
EOF

print_success "Deployment information saved to deployment-info.txt"

echo ""
echo "🚀 Ready to go! Your Decentralized Autonomous University is now live!"
echo ""
echo "📚 Next steps:"
echo "1. Create your first admin user"
echo "2. Set up instructor accounts through governance proposals"
echo "3. Create and publish your first courses"
echo "4. Start building your learning community!"
echo ""
echo "💡 Pro tip: Use the Candid UI to explore and test the canister APIs"
echo "   Visit: http://localhost:4943/?canisterId=\$(dfx canister id __Candid_UI --network local)"
