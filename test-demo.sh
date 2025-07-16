#!/bin/bash
# Demo script to test claude-emotion functionality

echo "🎭 Claude Emotion Demo"
echo "====================="
echo

echo "1. Testing analytical emotion:"
echo "[analytical] I'll analyze this problem step by step." | ./target/release/claude-emotion
echo

echo "2. Testing helpful emotion:"
echo "[helpful] I'd be happy to help you with that!" | ./target/release/claude-emotion
echo

echo "3. Testing curious emotion:"
echo "[curious] That's an interesting question!" | ./target/release/claude-emotion
echo

echo "4. Testing uncertain emotion:"
echo "[uncertain] I'm not entirely sure about this." | ./target/release/claude-emotion
echo

echo "5. Testing apologetic emotion:"
echo "[apologetic] I apologize for the confusion." | ./target/release/claude-emotion
echo

echo "6. Testing excited emotion:"
echo "[excited] This is really cool!" | ./target/release/claude-emotion
echo

echo "7. Testing multi-line response:"
echo -e "[analytical] Let me break this down:\n\n1. First, we need to understand the requirements\n2. Then implement the solution\n3. Finally, test it thoroughly" | ./target/release/claude-emotion
echo

echo "✅ Demo complete!"
echo "To install: cargo install --path . (or use Homebrew when published)"
echo "To use: claude-emotion --install-shell"