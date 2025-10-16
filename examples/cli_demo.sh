#!/bin/bash
# NeuroGraph OS - CLI Demo Script
# Demonstrates various CLI commands

set -e  # Exit on error

echo "╔══════════════════════════════════════════════╗"
echo "║     NeuroGraph OS - CLI Demo Script         ║"
echo "╚══════════════════════════════════════════════╝"
echo ""

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

step() {
    echo -e "${BLUE}▶ $1${NC}"
}

success() {
    echo -e "${GREEN}✓ $1${NC}"
}

wait_for_key() {
    echo -e "${YELLOW}Press Enter to continue...${NC}"
    read
}

# 1. System Info
step "Step 1: Show system information"
neurograph info
wait_for_key

# 2. Database Status
step "Step 2: Check database status"
neurograph db status
wait_for_key

# 3. System Health
step "Step 3: Check system health"
neurograph system health
wait_for_key

# 4. Create Tokens
step "Step 4: Creating test tokens..."

echo "Creating token 1..."
TOKEN1=$(neurograph token create --type demo --x 1.0 --y 0.0 --z 0.0 --weight 1.0 | grep "ID:" | awk '{print $2}')
success "Token 1 created: $TOKEN1"

echo "Creating token 2..."
TOKEN2=$(neurograph token create --type demo --x 2.0 --y 1.0 --z 0.0 --weight 1.5 | grep "ID:" | awk '{print $2}')
success "Token 2 created: $TOKEN2"

echo "Creating token 3..."
TOKEN3=$(neurograph token create --type demo --x 3.0 --y 2.0 --z 1.0 --weight 2.0 | grep "ID:" | awk '{print $2}')
success "Token 3 created: $TOKEN3"

wait_for_key

# 5. List Tokens
step "Step 5: List all tokens"
neurograph token list --limit 10
wait_for_key

# 6. Get Token Details
step "Step 6: Get token details"
neurograph token get $TOKEN1 --verbose
wait_for_key

# 7. Create Connections
step "Step 7: Creating graph connections..."

echo "Connecting Token 1 → Token 2"
neurograph graph connect $TOKEN1 $TOKEN2 --type spatial --weight 0.8
success "Connection created"

echo "Connecting Token 2 → Token 3"
neurograph graph connect $TOKEN2 $TOKEN3 --type temporal --weight 0.6
success "Connection created"

echo "Connecting Token 1 → Token 3"
neurograph graph connect $TOKEN1 $TOKEN3 --type semantic --weight 0.4 --bidirectional
success "Bidirectional connection created"

wait_for_key

# 8. Show Neighbors
step "Step 8: Show token neighbors"
echo "Neighbors of Token 1:"
neurograph graph neighbors $TOKEN1
wait_for_key

# 9. Find Path
step "Step 9: Find path between tokens"
echo "Finding path from Token 1 to Token 3:"
neurograph graph path $TOKEN1 $TOKEN3
wait_for_key

# 10. Graph Visualization
step "Step 10: Visualize graph"
echo "Visualizing Token 1 neighborhood:"
neurograph graph visualize $TOKEN1 --depth 2
wait_for_key

# 11. Graph Statistics
step "Step 11: Show graph statistics"
neurograph graph stats
wait_for_key

# 12. Token Count
step "Step 12: Count tokens"
neurograph token count
neurograph token count --type demo
wait_for_key

# 13. Spatial Search
step "Step 13: Spatial search"
echo "Searching tokens in region (0,0,0) to (5,5,5):"
neurograph token search --region 0 0 0 5 5 5
wait_for_key

# 14. System Metrics
step "Step 14: Show system metrics"
neurograph system metrics
wait_for_key

# 15. Configuration
step "Step 15: Show configuration"
echo "Available configurations:"
neurograph config list
wait_for_key

echo "Database configuration:"
neurograph config show infrastructure/database
wait_for_key

# 16. Cleanup (optional)
echo ""
echo -e "${YELLOW}Demo completed!${NC}"
echo ""
echo "Cleanup options:"
echo "  1. Keep demo data"
echo "  2. Delete demo tokens only"
echo "  3. Clean all data"
echo ""
read -p "Choose option (1-3): " cleanup_option

case $cleanup_option in
    2)
        step "Deleting demo tokens..."
        neurograph token delete $TOKEN1 --force
        neurograph token delete $TOKEN2 --force
        neurograph token delete $TOKEN3 --force
        success "Demo tokens deleted"
        ;;
    3)
        step "Cleaning all data..."
        neurograph db clean --force
        success "Database cleaned"
        ;;
    *)
        echo "Demo data kept"
        ;;
esac

echo ""
echo -e "${GREEN}╔══════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║     Demo completed successfully! ✓           ║${NC}"
echo -e "${GREEN}╚══════════════════════════════════════════════╝${NC}"
echo ""
echo "Learn more:"
echo "  - Documentation: docs/CLI.md"
echo "  - Help: neurograph --help"
echo "  - Quickstart: neurograph quickstart"
echo ""