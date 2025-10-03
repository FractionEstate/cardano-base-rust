# Pre-Commit Checklist

Use this checklist before committing security-sensitive code to cardano-base-rust.

## Code Quality

- [ ] Code compiles without warnings (`cargo build --workspace`)
- [ ] All tests pass (`cargo test --workspace`)
- [ ] Code is formatted (`cargo fmt --all`)
- [ ] Clippy passes with no warnings (`cargo clippy --workspace --all-targets -- -D warnings`)
- [ ] No new dependencies without review

## Security

- [ ] All `unsafe` blocks have SAFETY comments explaining invariants
- [ ] No `unwrap()` calls in production code paths (use `expect()` with messages)
- [ ] All user inputs are validated before use
- [ ] Error messages don't leak sensitive information
- [ ] Sensitive data (keys, seeds) use `MLockedBytes` or similar
- [ ] Memory is zeroed before deallocation (if applicable)
- [ ] Constant-time operations used for secret comparisons
- [ ] Integer arithmetic checked for overflow
- [ ] No new `#[allow(clippy::...)]` without justification

## Testing

- [ ] Unit tests added for new functionality
- [ ] Error cases tested with invalid inputs
- [ ] Security-critical code has dedicated test coverage
- [ ] Test vectors validated (if applicable)
- [ ] No test-only code in production paths

## Documentation

- [ ] Public APIs have doc comments
- [ ] Complex algorithms explained
- [ ] Security assumptions documented
- [ ] CHANGELOG.md updated (if applicable)
- [ ] README.md updated (if public API changed)

## Cryptographic Code (if applicable)

- [ ] Using established libraries (no custom crypto)
- [ ] Following IETF/NIST standards
- [ ] Constant-time operations for secrets
- [ ] Test vectors from standards included
- [ ] Side-channel considerations documented

## Git Hygiene

- [ ] Commit message is clear and descriptive
- [ ] No credentials or secrets in commit
- [ ] Commit is atomic (one logical change)
- [ ] Branch is up to date with master

## Before PR

- [ ] Run full CI locally (see .github/workflows/ci.yml)
- [ ] Security audit passes (`cargo audit`)
- [ ] License check passes (`cargo deny check`)
- [ ] Code review requested from appropriate team member
- [ ] PR description explains changes and rationale

## Special Considerations

### For Unsafe Code Changes

- [ ] Miri test passes (if applicable): `cargo +nightly miri test`
- [ ] Memory sanitizer clean (if applicable): `RUSTFLAGS="-Z sanitizer=address" cargo test`
- [ ] Reviewed by senior team member

### For VRF/Crypto Changes

- [ ] Reviewed by cryptography expert (@iquerejeta)
- [ ] Test vectors regenerated/validated
- [ ] Performance impact measured

### For Public API Changes

- [ ] Backwards compatibility considered
- [ ] Deprecation warnings added (if breaking)
- [ ] Migration guide provided (if needed)
- [ ] Version bump planned

---

**Remember**: When in doubt, ask for review! Security is everyone's responsibility.
