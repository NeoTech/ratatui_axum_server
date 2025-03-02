#!/bin/bash
# Test script for the Axum server with YAML configuration

# Default server address
HOST="localhost"
PORT="3000"
BASE_URL="http://$HOST:$PORT"

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to make a request and display the result
make_request() {
    local method=$1
    local endpoint=$2
    local description=$3
    
    echo -e "${BLUE}Testing: $description${NC}"
    echo -e "${BLUE}$method $BASE_URL$endpoint${NC}"
    
    # Make the request
    if [ "$method" = "GET" ]; then
        response=$(curl -s -X GET "$BASE_URL$endpoint")
    elif [ "$method" = "POST" ]; then
        response=$(curl -s -X POST "$BASE_URL$endpoint")
    elif [ "$method" = "PUT" ]; then
        response=$(curl -s -X PUT "$BASE_URL$endpoint")
    elif [ "$method" = "DELETE" ]; then
        response=$(curl -s -X DELETE "$BASE_URL$endpoint")
    fi
    
    # Display the response
    if [ -n "$response" ]; then
        echo -e "${GREEN}Response: $response${NC}"
    else
        echo -e "${RED}No response or error${NC}"
    fi
    
    echo ""
}

# Check if the server is running
echo -e "${BLUE}Checking if server is running...${NC}"
if ! curl -s "$BASE_URL/health" > /dev/null; then
    echo -e "${RED}Server is not running at $BASE_URL${NC}"
    echo -e "${RED}Please start the server before running this script${NC}"
    exit 1
fi

echo -e "${GREEN}Server is running at $BASE_URL${NC}"
echo ""

# Test basic endpoints
make_request "GET" "/" "Root endpoint"
make_request "GET" "/health" "Health check endpoint"
make_request "GET" "/api/status" "API status endpoint"

# Test API endpoints if using the advanced configuration
echo -e "${BLUE}Testing advanced endpoints (these may not exist with the default config)${NC}"
make_request "GET" "/api/users" "Get users endpoint"
make_request "GET" "/api/resource" "Get resource endpoint"
make_request "POST" "/api/resource" "Create resource endpoint"
make_request "PUT" "/api/resource" "Update resource endpoint"
make_request "DELETE" "/api/resource" "Delete resource endpoint"

echo -e "${GREEN}Testing complete!${NC}" 