#!/bin/sh

# Install git hooks for Mingot development
# Run this script once after cloning the repository

set -e

echo "🔧 Installing git hooks..."

# Check if .git directory exists
if [ ! -d ".git" ]; then
    echo "❌ Error: .git directory not found. Are you in the repository root?"
    exit 1
fi

# Copy hooks to .git/hooks
cp .githooks/hooks/pre-commit .git/hooks/pre-commit
chmod +x .git/hooks/pre-commit

echo "✅ Git hooks installed successfully!"
echo ""
echo "The following checks will run before each commit:"
echo "  📝 Code formatting (cargo fmt)"
echo "  🔎 Linting (cargo clippy)"
echo "  🧪 Tests (cargo test)"
echo ""
echo "To skip hooks on a specific commit, use: git commit --no-verify"
