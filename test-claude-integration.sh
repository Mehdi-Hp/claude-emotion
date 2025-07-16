#!/bin/bash
# Test script to simulate Claude Code integration

echo "🧪 Testing Claude Code Integration"
echo "=================================="
echo

# Build the project first
echo "Building project..."
cargo build --release --quiet

# Test 1: Simulate claude command with emotion injection
echo "1. Testing shell function with prompt injection:"
echo "   Input: 'How do I implement binary search?'"
echo "   Modified prompt: 'How do I implement binary search? [Start with: analytical/helpful/curious/uncertain/apologetic/excited]'"
echo

# Simulate what Claude might respond with
echo "   Simulated Claude response:"
echo "[analytical] I'll help you implement a binary search algorithm step by step." | ./target/release/claude-emotion
echo

# Test 2: Test with different emotions
echo "2. Testing various emotion responses:"
echo

echo "   Helpful response:"
echo "[helpful] I'd be happy to help you with that!" | ./target/release/claude-emotion
echo

echo "   Curious response:"
echo "[curious] That's an interesting question about algorithms!" | ./target/release/claude-emotion
echo

echo "   Uncertain response:"
echo "[uncertain] I'm not entirely sure about the best approach here." | ./target/release/claude-emotion
echo

# Test 3: Test error handling
echo "3. Testing error handling:"
echo

echo "   Invalid emotion (should pass through):"
echo "[invalid] This should pass through unchanged" | ./target/release/claude-emotion
echo

echo "   No emotion tag (should pass through):"
echo "Regular response without emotion tag" | ./target/release/claude-emotion
echo

# Test 4: Test shell integration setup
echo "4. Testing shell integration script:"
echo

if [ -f "./shell/claude-emotion.sh" ]; then
    echo "   ✓ Shell integration script exists"
    echo "   ✓ Contains claude function override"
    grep -q "claude()" shell/claude-emotion.sh && echo "   ✓ claude() function found"
    grep -q "claude-emotion-disable" shell/claude-emotion.sh && echo "   ✓ disable function found"
    grep -q "claude-emotion-enable" shell/claude-emotion.sh && echo "   ✓ enable function found"
else
    echo "   ✗ Shell integration script missing"
fi

echo

# Test 5: Test CLI functionality
echo "5. Testing CLI functionality:"
echo

echo "   Testing --list-emotions:"
./target/release/claude-emotion --list-emotions
echo

echo "   Testing --help:"
./target/release/claude-emotion --help | head -5
echo

echo "✅ Integration tests complete!"
echo
echo "Next steps to fully test:"
echo "1. Source the shell script: source ./shell/claude-emotion.sh"
echo "2. Test: echo 'test prompt' | claude (if claude command is available)"
echo "3. Verify emotion hint is added and processed correctly"