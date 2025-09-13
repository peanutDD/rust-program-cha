#!/bin/bash

# Script to restart rust-analyzer and clear caches

echo "🔄 Restarting rust-analyzer..."

# Kill any existing rust-analyzer processes
pkill -f rust-analyzer

# Clear rust-analyzer cache
rm -rf ~/.cache/rust-analyzer
rm -rf .rust-analyzer

# Clean cargo cache
cargo clean

echo "✅ rust-analyzer restart complete!"
echo "📝 Please restart VS Code or reload the window for changes to take effect."
echo "💡 Use Cmd+Shift+P -> 'Developer: Reload Window' in VS Code"