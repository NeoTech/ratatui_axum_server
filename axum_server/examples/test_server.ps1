# Test script for the Axum server with YAML configuration

# Default server address
$Host = "localhost"
$Port = "3000"
$BaseUrl = "http://$Host`:$Port"

# Function to make a request and display the result
function Make-Request {
    param (
        [string]$Method,
        [string]$Endpoint,
        [string]$Description
    )
    
    Write-Host "Testing: $Description" -ForegroundColor Blue
    Write-Host "$Method $BaseUrl$Endpoint" -ForegroundColor Blue
    
    # Make the request
    try {
        $response = Invoke-WebRequest -Uri "$BaseUrl$Endpoint" -Method $Method -UseBasicParsing
        Write-Host "Response: $($response.Content)" -ForegroundColor Green
    }
    catch {
        Write-Host "Error: $_" -ForegroundColor Red
    }
    
    Write-Host ""
}

# Check if the server is running
Write-Host "Checking if server is running..." -ForegroundColor Blue
try {
    $null = Invoke-WebRequest -Uri "$BaseUrl/health" -UseBasicParsing -ErrorAction Stop
    Write-Host "Server is running at $BaseUrl" -ForegroundColor Green
}
catch {
    Write-Host "Server is not running at $BaseUrl" -ForegroundColor Red
    Write-Host "Please start the server before running this script" -ForegroundColor Red
    exit 1
}

Write-Host ""

# Test basic endpoints
Make-Request -Method "GET" -Endpoint "/" -Description "Root endpoint"
Make-Request -Method "GET" -Endpoint "/health" -Description "Health check endpoint"
Make-Request -Method "GET" -Endpoint "/api/status" -Description "API status endpoint"

# Test API endpoints if using the advanced configuration
Write-Host "Testing advanced endpoints (these may not exist with the default config)" -ForegroundColor Blue
Make-Request -Method "GET" -Endpoint "/api/users" -Description "Get users endpoint"
Make-Request -Method "GET" -Endpoint "/api/resource" -Description "Get resource endpoint"
Make-Request -Method "POST" -Endpoint "/api/resource" -Description "Create resource endpoint"
Make-Request -Method "PUT" -Endpoint "/api/resource" -Description "Update resource endpoint"
Make-Request -Method "DELETE" -Endpoint "/api/resource" -Description "Delete resource endpoint"

Write-Host "Testing complete!" -ForegroundColor Green 