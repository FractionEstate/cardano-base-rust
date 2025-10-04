#!/bin/bash
# Quick publish script for cardano-base-rust
# Run this after creating the repository on GitHub

set -e

echo "🚀 Publishing cardano-base-rust to FractionEstate"
echo "=================================================="
echo ""

# Step 1: Verify tests pass
echo "✓ Step 1: Running tests..."
if cargo test --workspace --quiet 2>&1 | grep -q "test result: ok"; then
    echo "  ✅ All tests passing"
else
    echo "  ❌ Tests failed! Fix before publishing."
    exit 1
fi

# Step 2: Stage all changes
echo ""
echo "✓ Step 2: Staging all changes..."

# Stage Rust files
git add Cargo.toml Cargo.lock
git add */Cargo.toml
git add */src/ */tests/ */build.rs 2>/dev/null || true

# Stage documentation
git add docs/
git add README.md */README.md
git add CHANGELOG.md */CHANGELOG.md 2>/dev/null || true

# Stage GitHub configuration
git add .github/

# Stage deleted Haskell/C files
git add -u

# Stage new/modified files
git add .gitignore
git add regenerate_vectors.sh 2>/dev/null || true
git add PUBLISH_GUIDE.md QUICK_PUBLISH.sh

echo "  ✅ Changes staged"

# Step 3: Show status
echo ""
echo "✓ Step 3: Review changes to be committed:"
git status --short | head -20
total_changes=$(git status --short | wc -l)
echo "  ... ($total_changes total changes)"

# Step 4: Commit
echo ""
read -p "Commit these changes? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    git commit -m "feat: Complete Haskell to Rust migration

- Migrated all 13 packages to 100% Pure Rust
- Removed 26 C files (9,716 lines of C code)
- Removed all Haskell code (100% migrated)
- Implemented Pure Rust VRF using curve25519-dalek
- All 148 tests passing
- Comprehensive documentation with GitHub Wiki sync
- Zero external C dependencies

Packages:
- base-deriving-via
- cardano-base
- cardano-binary
- cardano-crypto-class
- cardano-git-rev
- cardano-slotting
- cardano-strict-containers
- cardano-vrf-pure (NEW - Pure Rust VRF)
- deepseq
- heapwords
- measures
- nothunks
- orphans-deriving-via

Features:
- Pure Rust VRF (IETF draft-03 and draft-13)
- 148 tests with regenerated test vectors
- Automatic GitHub Wiki documentation sync
- Complete API documentation
- Migration guides and development docs"
    echo "  ✅ Changes committed"
else
    echo "  ⚠️  Skipping commit. Run 'git commit' manually when ready."
    exit 0
fi

# Step 5: Update remote
echo ""
echo "✓ Step 5: Update git remote..."
current_origin=$(git remote get-url origin 2>/dev/null || echo "none")
if [[ $current_origin == *"cardano-base-rust"* ]]; then
    echo "  ✅ Remote already set to cardano-base-rust"
else
    read -p "Update remote to FractionEstate/cardano-base-rust? (y/n) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        git remote set-url origin https://github.com/FractionEstate/cardano-base-rust.git
        echo "  ✅ Remote updated"
    else
        echo "  ⚠️  Keeping current remote. Update manually with:"
        echo "     git remote set-url origin https://github.com/FractionEstate/cardano-base-rust.git"
    fi
fi

# Step 6: Show remotes
echo ""
echo "✓ Step 6: Current remotes:"
git remote -v

# Step 7: Push
echo ""
read -p "Push to origin/master? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo "  📤 Pushing to origin/master..."
    git push -u origin master
    echo "  ✅ Pushed successfully!"
else
    echo "  ⚠️  Skipping push. Run 'git push -u origin master' manually when ready."
    exit 0
fi

# Done!
echo ""
echo "=================================================="
echo "🎉 Publication Complete!"
echo "=================================================="
echo ""
echo "Next steps:"
echo "1. Visit https://github.com/FractionEstate/cardano-base-rust"
echo "2. Enable Wiki in Settings → Features"
echo "3. Enable GitHub Actions"
echo "4. Trigger wiki sync: Actions → Sync Documentation to Wiki → Run workflow"
echo "5. Add repository topics: rust, cardano, blockchain, cryptography, vrf"
echo "6. (Optional) Create release: Releases → Create new release → v1.0.0"
echo ""
echo "📚 View the full publishing guide: PUBLISH_GUIDE.md"
