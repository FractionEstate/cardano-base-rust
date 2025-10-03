#!/bin/bash
set -e

echo "🚀 Pushing files to GitHub using API..."

# Get list of all files
FILES=$(git ls-files)

# Create initial commit with essential files
echo "📦 Step 1: Pushing core files..."

# Push README and main docs first
cat > /tmp/files1.txt << 'EOF'
README.md
LICENSE
NOTICE
CHANGELOG.md
CODE-OF-CONDUCT.md
CONTRIBUTING.md
SECURITY.md
CODEOWNERS
RELEASING.md
.gitignore
Cargo.toml
Cargo.lock
PUBLISH_GUIDE.md
QUICK_PUBLISH.sh
PUSH_STATUS.md
EOF

echo "Core files ready to push"
