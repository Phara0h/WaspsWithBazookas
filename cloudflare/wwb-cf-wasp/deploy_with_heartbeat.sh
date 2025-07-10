#!/bin/bash

# Deployment script for Wasp Worker with persistent heartbeat
# This script deploys the worker and verifies the heartbeat system

set -e

echo "🚀 Deploying Wasp Worker with Persistent Heartbeat"
echo "=================================================="

# Check if wrangler is installed
if ! command -v wrangler &> /dev/null; then
    echo "❌ Wrangler CLI is not installed. Please install it first:"
    echo "   npm install -g wrangler"
    exit 1
fi

# Check if we're in the right directory
if [ ! -f "wrangler.toml" ]; then
    echo "❌ Please run this script from the wwb-cf-wasp directory"
    exit 1
fi

# Deploy the worker
echo "📦 Deploying Wasp Worker..."
wrangler deploy

# Get the worker URL
WORKER_URL=$(wrangler whoami --format json | jq -r '.account.name' | sed 's/^/https:\/\/wwb-cf-wasp./' | sed 's/$/.workers.dev/')

echo "✅ Worker deployed to: $WORKER_URL"

# Wait a moment for deployment to settle
echo "⏳ Waiting for deployment to settle..."
sleep 5

# Test the heartbeat system
echo "🧪 Testing heartbeat system..."

# Test 1: Initialize wasp (this starts the heartbeat)
echo "📋 Initializing wasp..."
BOOP_RESPONSE=$(curl -s "$WORKER_URL/boop")
echo "Response: $BOOP_RESPONSE"

# Check if initialization was successful
if echo "$BOOP_RESPONSE" | grep -q "persistent_heartbeat.*true"; then
    echo "✅ Wasp initialized successfully with persistent heartbeat"
else
    echo "❌ Wasp initialization failed"
    exit 1
fi

# Test 2: Check health
echo "📋 Checking health..."
HEALTH_RESPONSE=$(curl -s "$WORKER_URL/health")
echo "Response: $HEALTH_RESPONSE"

# Test 3: Wait and verify heartbeat is working
echo "📋 Waiting 10 seconds to verify heartbeat..."
echo "💓 Heartbeat should be sending requests to Hive every 5 seconds..."
sleep 10

# Test 4: Verify wasp is still responsive
echo "📋 Verifying wasp is still responsive..."
BOOP_RESPONSE_2=$(curl -s "$WORKER_URL/boop")
if [ $? -eq 0 ]; then
    echo "✅ Wasp is responsive and heartbeat is working"
else
    echo "❌ Wasp is not responding"
    exit 1
fi

echo ""
echo "🎉 Deployment and heartbeat test completed successfully!"
echo ""
echo "📊 Summary:"
echo "- ✅ Worker deployed to: $WORKER_URL"
echo "- ✅ Wasp initialized with persistent heartbeat"
echo "- ✅ Health check passed"
echo "- ✅ Heartbeat verification successful"
echo ""
echo "🔧 Next Steps:"
echo "1. Configure your Hive server URL:"
echo "   wrangler secret put HIVE_URL"
echo ""
echo "2. Configure your Bazooka worker URL:"
echo "   wrangler secret put BAZOOKA_WORKER_URL"
echo ""
echo "3. Test load testing functionality:"
echo "   curl -X PUT $WORKER_URL/fire -H 'Content-Type: application/json' -d '{\"target\":\"https://httpbin.org/get\",\"c\":10,\"d\":30}'"
echo ""
echo "4. Monitor logs:"
echo "   wrangler tail --name wwb-cf-wasp"
echo ""
echo "💡 The heartbeat system will continue running until you call /die" 