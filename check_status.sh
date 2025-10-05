#!/bin/bash
# Quick status check for cardano-base-rust project

echo "========================================="
echo "Cardano Base Rust - Project Status Check"
echo "========================================="
echo

echo "📦 Building project..."
if cargo build --features serde 2>&1 | tail -1 | grep -q "Finished"; then
    echo "✅ Build successful"
else
    echo "❌ Build failed"
fi
echo

echo "🧪 Running DirectSerialise tests..."
TEST_OUTPUT=$(cargo test --test direct_serialise_impls --features serde 2>&1 | grep "test result")
if echo "$TEST_OUTPUT" | grep -q "9 passed"; then
    echo "✅ DirectSerialise: 9/9 tests passing"
else
    echo "❌ DirectSerialise: Tests failing"
fi
echo

echo "🔍 Running all tests..."
ALL_TESTS=$(cargo test --features serde --lib --tests 2>&1 | grep "test result" | tail -1)
echo "📊 $ALL_TESTS"
echo

echo "📋 Session Progress:"
echo "  ✅ Session 3: Phase 2 + Phase 3 Infrastructure"
echo "  ✅ Session 4: Sum KES Blocker Resolved"
echo "  ✅ Session 5: DirectSerialise Optimization"
echo

echo "📝 Documentation Created:"
echo "  ✅ DIRECTSERIALISE_OPTIMIZATION_COMPLETE.md"
echo "  ✅ SESSION5_SUMMARY.md"
echo "  ✅ SESSION5_FINAL_SUMMARY.md"
echo "  ✅ PHASE3_HASKELL_INTEGRATION_GUIDE.md"
echo "  ✅ SUM_KES_BLOCKER_RESOLVED.md"
echo

echo "⏳ Pending:"
echo "  ⏸️  Phase 3 Haskell Integration (external dependency)"
echo

echo "========================================="
echo "Status Check Complete"
echo "========================================="
