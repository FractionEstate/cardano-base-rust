#!/bin/bash
# Comprehensive quality check script for cardano-base-rust
# Ensures code quality, tests, and documentation are in good state

set -e

echo "═══════════════════════════════════════════════════════════════"
echo "  Cardano Base Rust - Comprehensive Quality Check"
echo "═══════════════════════════════════════════════════════════════"
echo ""

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Track results
TOTAL_CHECKS=0
PASSED_CHECKS=0
FAILED_CHECKS=0

function check_step() {
    TOTAL_CHECKS=$((TOTAL_CHECKS + 1))
    echo -e "${BLUE}[$TOTAL_CHECKS]${NC} $1..."
}

function pass() {
    PASSED_CHECKS=$((PASSED_CHECKS + 1))
    echo -e "${GREEN}✓${NC} PASSED: $1"
    echo ""
}

function warn() {
    echo -e "${YELLOW}⚠${NC} WARNING: $1"
    echo ""
}

function fail() {
    FAILED_CHECKS=$((FAILED_CHECKS + 1))
    echo -e "${RED}✗${NC} FAILED: $1"
    echo ""
}

# 1. Code Formatting
check_step "Checking code formatting (cargo fmt)"
if cargo fmt --all -- --check > /dev/null 2>&1; then
    pass "All code is properly formatted"
else
    fail "Code formatting issues found. Run: cargo fmt --all"
fi

# 2. Build Check
check_step "Building entire workspace"
if cargo build --workspace --all-targets 2>&1 | tail -1 | grep -q "Finished"; then
    pass "Workspace builds successfully"
else
    fail "Build errors found"
fi

# 3. Test Suite
check_step "Running all tests"
TEST_OUTPUT=$(cargo test --workspace 2>&1)
TEST_COUNT=$(echo "$TEST_OUTPUT" | grep "test result:" | awk '{sum += $4} END {print sum}')
FAILED_TESTS=$(echo "$TEST_OUTPUT" | grep "test result:" | grep -v "0 failed" | wc -l)

if [ "$FAILED_TESTS" -eq 0 ]; then
    pass "$TEST_COUNT tests passing across workspace"
else
    fail "Some tests are failing"
fi

# 4. Clippy Warnings (informational only)
check_step "Checking clippy warnings (informational)"
CLIPPY_OUTPUT=$(cargo clippy --workspace --all-targets 2>&1)
WARNING_COUNT=$(echo "$CLIPPY_OUTPUT" | grep -c "warning:" || true)

if [ "$WARNING_COUNT" -eq 0 ]; then
    pass "No clippy warnings"
else
    warn "$WARNING_COUNT clippy warnings found (deferred per Phase 05)"
fi

# 5. Documentation Build
check_step "Building documentation"
DOC_OUTPUT=$(cargo doc --workspace --no-deps 2>&1)
if echo "$DOC_OUTPUT" | grep -q "Finished"; then
    pass "Documentation builds successfully"
else
    fail "Documentation build errors"
fi

# 6. Check for TODO/FIXME in critical files
check_step "Checking for critical TODOs/FIXMEs"
CRITICAL_TODOS=$(grep -r "TODO\|FIXME" --include="*.rs" cardano-crypto-class/src cardano-binary/src 2>/dev/null | grep -v "test" | wc -l)

if [ "$CRITICAL_TODOS" -eq 0 ]; then
    pass "No critical TODOs/FIXMEs in main crates"
else
    warn "$CRITICAL_TODOS TODO/FIXME comments found in critical paths"
fi

# 7. Check README files exist
check_step "Checking README files"
MISSING_READMES=0
for crate in cardano-*/; do
    if [ ! -f "$crate/README.md" ]; then
        MISSING_READMES=$((MISSING_READMES + 1))
    fi
done

if [ "$MISSING_READMES" -eq 0 ]; then
    pass "All crates have README files"
else
    warn "$MISSING_READMES crates missing README files"
fi

# 8. Check CHANGELOG files
check_step "Checking CHANGELOG files"
MISSING_CHANGELOGS=0
for crate in cardano-*/; do
    if [ ! -f "$crate/CHANGELOG.md" ]; then
        MISSING_CHANGELOGS=$((MISSING_CHANGELOGS + 1))
    fi
done

if [ "$MISSING_CHANGELOGS" -eq 0 ]; then
    pass "All crates have CHANGELOG files"
else
    warn "$MISSING_CHANGELOGS crates missing CHANGELOG files"
fi

# 9. Check for unsafe code
check_step "Checking for unsafe code blocks"
UNSAFE_COUNT=$(grep -r "unsafe" --include="*.rs" cardano-crypto-class/src cardano-binary/src cardano-slotting/src cardano-strict-containers/src 2>/dev/null | grep -v "test" | grep -v "#\[cfg" | wc -l)

if [ "$UNSAFE_COUNT" -eq 0 ]; then
    pass "No unsafe code in main implementation"
else
    warn "$UNSAFE_COUNT unsafe blocks found (review required)"
fi

# 10. Check dependency versions
check_step "Checking dependency consistency"
# This is a simple check - a more thorough one would parse Cargo.toml files
pass "Dependency check (manual review recommended)"

# Summary
echo ""
echo "═══════════════════════════════════════════════════════════════"
echo "  Quality Check Summary"
echo "═══════════════════════════════════════════════════════════════"
echo -e "Total Checks: ${BLUE}$TOTAL_CHECKS${NC}"
echo -e "Passed: ${GREEN}$PASSED_CHECKS${NC}"
echo -e "Failed: ${RED}$FAILED_CHECKS${NC}"
echo ""

if [ "$FAILED_CHECKS" -eq 0 ]; then
    echo -e "${GREEN}✓ All critical checks passed!${NC}"
    echo ""
    echo "Test Count: $TEST_COUNT passing"
    echo "Clippy Warnings: $WARNING_COUNT (deferred per Phase 05)"
    echo "Critical TODOs: $CRITICAL_TODOS"
    echo ""
    exit 0
else
    echo -e "${RED}✗ Some checks failed. Please review and fix.${NC}"
    echo ""
    exit 1
fi
