#!/bin/bash
# Complete the repository push
# The README has been pushed via API, now we need to force push the local commit

set -e

echo "🚀 Completing repository push..."
echo "================================"
echo ""

# The GitHub API created an initial commit with README
# We need to fetch it and then push our complete commit

echo "✓ Step 1: Fetching initial commit from GitHub..."
git fetch origin master

echo ""
echo "✓ Step 2: Creating merge commit..."
# Allow unrelated histories since API created a separate initial commit
git pull origin master --allow-unrelated-histories --no-edit || true

echo ""
echo "✓ Step 3: Pushing complete repository..."
# This should work now that histories are merged
git push origin master

echo ""
echo "=========================================="
echo "✅ Repository push complete!"
echo "=========================================="
echo ""
echo "Visit your repository:"
echo "https://github.com/FractionEstate/cardano-base-rust"
echo ""
echo "Next steps:"
echo "1. Enable Wiki in Settings → Features"
echo "2. Enable GitHub Actions"
echo "3. Trigger wiki sync: Actions → 'Sync Documentation to Wiki' → Run workflow"
echo "4. Add repository topics: rust, cardano, blockchain, cryptography, vrf"
echo ""
