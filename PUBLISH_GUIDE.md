# Publishing Guide: cardano-base-rust

This guide walks through publishing this Pure Rust implementation to `FractionEstate/cardano-base-rust`.

## Current Status

✅ **Code Complete:** 100% Pure Rust, 148 tests passing
✅ **Documentation:** Comprehensive docs/ structure with wiki sync
✅ **No Dependencies:** 0 C files, 0 Haskell files remaining
✅ **Ready to Publish**

## Pre-Publication Checklist

### 1. Verify All Tests Pass

```bash
cargo test --workspace
cargo clippy --workspace -- -D warnings
cargo fmt --check

```

### 2. Update Repository-Specific Files

Files that need updating before publishing:

#### `.github/workflows/sync-wiki.yml`
Update the wiki URL to point to the new repository.

#### `README.md`
Update wiki links from `../../wiki/` to point to the new repository.

#### `Cargo.toml` (workspace)
Verify all package metadata points to new repository.

#### `CODEOWNERS`
Update to reflect FractionEstate ownership.

### 3. Commit All Changes

```bash

# Stage all Rust code
git add Cargo.toml Cargo.lock
git add */Cargo.toml
git add */src/ */tests/ */build.rs

# Stage documentation
git add docs/
git add README.md */README.md
git add CHANGELOG.md */CHANGELOG.md

# Stage GitHub configuration
git add .github/

# Stage deleted Haskell/C files
git add -u

# Stage new files
git add .gitignore
git add regenerate_vectors.sh

# Review what will be committed
git status

# Commit with descriptive message
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

```

## Publishing Steps

### Step 1: Create New Repository on GitHub

1. Go to <https://github.com/FractionEstate>
2. Click "New repository"
3. Repository name: `cardano-base-rust`
4. Description: "Pure Rust implementation of Cardano Base libraries - 0 C dependencies, 148 tests passing"
5. Visibility: **Public**
6. Do NOT initialize with README, .gitignore, or license (we have these)
7. Click "Create repository"

### Step 2: Update Remote URLs

```bash

# Remove current origin (optional - keeps upstream)
git remote remove origin

# Add new origin
git remote add origin <https://github.com/FractionEstate/cardano-base-rust.git>
# Verify remotes
git remote -v

```

Should show:

```
origin <https://github.com/FractionEstate/cardano-base-rust.git> (fetch)
origin <https://github.com/FractionEstate/cardano-base-rust.git> (push)
upstream <https://github.com/IntersectMBO/cardano-base.git> (fetch)
upstream <https://github.com/IntersectMBO/cardano-base.git> (push)

```

### Step 3: Update Repository References

Before pushing, update these files to point to the new repository:

#### Update `.github/workflows/sync-wiki.yml`:

```yaml

# Line ~15: Update wiki checkout URL

- name: Checkout Wiki

  uses: actions/checkout@v4
  with:
    repository: FractionEstate/cardano-base-rust.wiki
    path: wiki

```

#### Update `README.md`:
Replace wiki links from `../../wiki/` to:

```markdown
[Documentation Wiki](https://github.com/FractionEstate/cardano-base-rust/wiki)
[Package Overview](https://github.com/FractionEstate/cardano-base-rust/wiki/API-Packages)
[VRF API](https://github.com/FractionEstate/cardano-base-rust/wiki/API-VRF-API)

```

#### Update `docs/COMPLETION_REPORT.md`:
Replace `<owner>` placeholders with `FractionEstate`.

#### Update `CODEOWNERS`:

```

# Default owners for everything

* @FractionEstate/cardano-base-maintainers

# Or specific individuals:

* @your-username

```

#### Commit Updates:

```bash
git add .github/workflows/sync-wiki.yml README.md docs/COMPLETION_REPORT.md CODEOWNERS
git commit -m "chore: Update repository URLs for cardano-base-rust"

```

### Step 4: Push to New Repository

```bash

# Push master branch
git push -u origin master

# Push any tags (if you have them)
git push origin --tags

```

### Step 5: Configure GitHub Repository Settings

After pushing, configure the repository on GitHub:

#### General Settings

- Description: "Pure Rust implementation of Cardano Base libraries"
- Website: (optional - link to documentation or Cardano)
- Topics: Add tags like `rust`, `cardano`, `blockchain`, `cryptography`, `vrf`, `zero-dependencies`

#### Enable GitHub Wiki

1. Go to Settings → Features
2. Enable "Wikis"
3. The wiki will auto-populate from docs/ via the sync workflow

#### Enable GitHub Actions

1. Go to Actions tab
2. Click "I understand my workflows, go ahead and enable them"
3. The sync-wiki.yml workflow should appear

#### Branch Protection (Recommended)

1. Go to Settings → Branches
2. Add rule for `master`:
   - Require pull request reviews before merging
   - Require status checks to pass (CI tests)
   - Require branches to be up to date

#### Set Up Topics/Tags
Add these topics to improve discoverability:

- `rust`
- `cardano`
- `blockchain`
- `cryptography`
- `vrf`
- `pure-rust`
- `zero-dependencies`
- `haskell-to-rust`
- `cardano-base`

### Step 6: Trigger Wiki Sync

```bash

# Make a minor change to docs/ to trigger the workflow
echo "" >> docs/README.md
git add docs/README.md
git commit -m "docs: Trigger wiki sync"
git push origin master

```

Or manually trigger via GitHub:

1. Go to Actions → Sync Documentation to Wiki
2. Click "Run workflow"
3. Select `master` branch
4. Click "Run workflow"

### Step 7: Create Initial Release (Optional)

Create a GitHub Release to mark this milestone:

1. Go to Releases → "Create a new release"
2. Tag version: `v1.0.0` (or appropriate version)
3. Release title: "Pure Rust Migration Complete - v1.0.0"
4. Description:

   ```markdown
   # 🎉 Pure Rust Implementation Complete

   First stable release of cardano-base as 100% Pure Rust.

   ## 🚀 Highlights

   - ✅ 100% Pure Rust implementation
   - ✅ 0 C dependencies (removed 26 C files)
   - ✅ 0 Haskell code (100% migrated)
   - ✅ 148 tests passing (100% success rate)
   - ✅ Pure Rust VRF using curve25519-dalek
   - ✅ Comprehensive documentation with GitHub Wiki

   ## 📦 Packages
   All 13 packages migrated:

   - base-deriving-via
   - cardano-base
   - cardano-binary
   - cardano-crypto-class
   - cardano-git-rev
   - cardano-slotting
   - cardano-strict-containers
   - **cardano-vrf-pure** (NEW - Pure Rust VRF)
   - deepseq
   - heapwords
   - measures
   - nothunks
   - orphans-deriving-via

   ## 📚 Documentation

   - [Full Documentation Wiki](https://github.com/FractionEstate/cardano-base-rust/wiki)
   - [Migration Summary](https://github.com/FractionEstate/cardano-base-rust/wiki/Migration-Summary)
   - [VRF API Reference](https://github.com/FractionEstate/cardano-base-rust/wiki/API-VRF-API)
   - [Testing Guide](https://github.com/FractionEstate/cardano-base-rust/wiki/Development-Testing-Guide)

   ```

5. Click "Publish release"

## Post-Publication Tasks

### 1. Verify Everything Works

```bash

# Clone the new repository
cd /tmp
git clone <https://github.com/FractionEstate/cardano-base-rust.git>
cd cardano-base-rust

# Build and test
cargo build --workspace
cargo test --workspace

# Check wiki is populated
# Visit: <https://github.com/FractionEstate/cardano-base-rust/wiki>
```

### 2. Update README Badges (Optional)

Add CI badges to README.md:

```markdown
[![CI](https://github.com/FractionEstate/cardano-base-rust/workflows/CI/badge.svg)](https://github.com/FractionEstate/cardano-base-rust/actions)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

```

### 3. Announce the Release

Consider announcing on:

- Cardano forums/communities
- Reddit (r/cardano)
- Twitter/X
- Your organization's channels

### 4. Set Up Crates.io Publishing (Future)

When ready to publish packages to crates.io:

1. Update `Cargo.toml` files with:
   - Correct version numbers
   - Repository URL
   - License information
   - Keywords and categories

2. Login to crates.io:

   ```bash
   cargo login <your-api-token>

   ```

3. Publish packages (in dependency order):

   ```bash
   # Start with packages that have no internal dependencies
   cargo publish --package heapwords
   cargo publish --package measures
   cargo publish --package deepseq
   # ... continue with dependent packages

   ```

## Troubleshooting

### Wiki Not Syncing

- Check Actions tab for workflow errors
- Verify wiki is enabled in repository settings
- Manually trigger workflow: Actions → Sync Documentation to Wiki → Run workflow

### CI Tests Failing

- Check `.github/workflows/ci.yml` exists
- Verify all dependencies are properly specified in Cargo.toml files
- Check GitHub Actions logs for specific errors

### Permission Denied on Push

- Verify you have write access to FractionEstate/cardano-base-rust
- Check your GitHub authentication (SSH keys or token)
- Try re-adding remote: `git remote set-url origin <https://github.com/FractionEstate/cardano-base-rust.git`>
## Important Notes

### Maintain Attribution
Keep the original LICENSE, NOTICE, and copyright headers to properly attribute the original Haskell implementation from IntersectMBO.

### Upstream Relationship
The `upstream` remote points to the original Haskell repository. You can pull updates if needed:

```bash
git fetch upstream
git log upstream/master

```

### Version Management
Start with `v1.0.0` to indicate this is the first stable Pure Rust release. Use semantic versioning for future releases.

## Quick Command Reference

```bash

# Create new repo on GitHub first, then:

# Update references to new repo
# (Edit files as described above)

# Commit everything
git add -A
git commit -m "feat: Complete Haskell to Rust migration"

# Update remote
git remote set-url origin <https://github.com/FractionEstate/cardano-base-rust.git>
# Push to new repository
git push -u origin master

# Enable wiki and trigger sync
# (Via GitHub UI or push change to docs/)

```

---

**Status:** Ready to publish! 🚀

For questions or issues during publishing, refer to:

- [GitHub Docs: Creating a Repository](https://docs.github.com/en/repositories/creating-and-managing-repositories)
- [Cargo Book: Publishing on crates.io](https://doc.rust-lang.org/cargo/reference/publishing.html)
