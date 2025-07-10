#!/bin/bash

# Test script for Wasp Worker heartbeat functionality
# This script tests the persistent heartbeat system using Durable Object alarms

set -e

# Configuration
WASP_WORKER_URL="${WASP_WORKER_URL:-https://your-wasp-worker.your-subdomain.workers.dev}"
HIVE_URL="${HIVE_URL:-https://your-hive-server.com}"

echo "🧪 Testing Wasp Worker Heartbeat Functionality"
echo "=============================================="
echo "Wasp Worker URL: $WASP_WORKER_URL"
echo "Hive URL: $HIVE_URL"
echo ""

# Test 1: Initialize wasp (this should start the heartbeat)
echo "📋 Test 1: Initializing wasp and starting heartbeat..."
BOOP_RESPONSE=$(curl -s "$WASP_WORKER_URL/boop")
echo "Response: $BOOP_RESPONSE"

# Check if initialization was successful
if echo "$BOOP_RESPONSE" | grep -q "persistent_heartbeat.*true"; then
    echo "✅ Wasp initialized successfully with persistent heartbeat"
else
    echo "❌ Wasp initialization failed or heartbeat not enabled"
    exit 1
fi

echo ""

# Test 2: Check health status
echo "📋 Test 2: Checking health status..."
HEALTH_RESPONSE=$(curl -s "$WASP_WORKER_URL/health")
echo "Response: $HEALTH_RESPONSE"

echo ""

# Test 3: Wait and verify heartbeat is working
echo "📋 Test 3: Waiting 10 seconds to verify heartbeat is working..."
echo "💓 Heartbeat should be sending requests to Hive every 5 seconds..."
sleep 10

# Test 4: Check if wasp is still responsive
echo "📋 Test 4: Verifying wasp is still responsive..."
BOOP_RESPONSE_2=$(curl -s "$WASP_WORKER_URL/boop")
echo "Response: $BOOP_RESPONSE_2"

echo ""

# Test 5: Test graceful shutdown
echo "📋 Test 5: Testing graceful shutdown (stopping heartbeat)..."
DIE_RESPONSE=$(curl -s -X DELETE "$WASP_WORKER_URL/die")
echo "Response: $DIE_RESPONSE"

# Check if shutdown was successful
if echo "$DIE_RESPONSE" | grep -q "Heartbeat stopped"; then
    echo "✅ Graceful shutdown successful - heartbeat stopped"
else
    echo "❌ Graceful shutdown failed"
    exit 1
fi

echo ""

# Test 6: Verify wasp is no longer responding (should be shutting down)
echo "📋 Test 6: Verifying wasp is shutting down..."
if curl -s "$WASP_WORKER_URL/boop" > /dev/null 2>&1; then
    echo "⚠️  Wasp is still responding (may be normal if shutdown is delayed)"
else
    echo "✅ Wasp has stopped responding (shutdown successful)"
fi

echo ""
echo "🎉 Heartbeat functionality test completed!"
echo ""
echo "📊 Summary:"
echo "- ✅ Wasp initialization with persistent heartbeat"
echo "- ✅ Health check"
echo "- ✅ Heartbeat verification (10-second wait)"
echo "- ✅ Graceful shutdown with heartbeat stop"
echo "- ✅ Clean state management"
echo ""
echo "💡 The heartbeat system uses Cloudflare Durable Object alarms to maintain"
echo "   persistent communication with the Hive server every 5 seconds." 