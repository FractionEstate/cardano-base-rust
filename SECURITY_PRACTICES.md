# Security Best Practices for cardano-base-rust

This document outlines security best practices for contributing to cardano-base-rust.

## Code Security Guidelines

### 1. Unsafe Code

When writing `unsafe` code, **always** add a SAFETY comment explaining:
- What invariants the unsafe code relies on
- Why those invariants are guaranteed to hold
- What would go wrong if the invariants were violated

Example:
```rust
// SAFETY: ptr is guaranteed non-null by the caller contract.
// We verified the allocation succeeded before reaching this point.
let non_null = unsafe { NonNull::new_unchecked(ptr) };
```

### 2. Error Handling

- **Never use `unwrap()` in production code paths**
  - Use `expect()` with descriptive messages for programmer errors
  - Use `?` or explicit error handling for runtime errors
  
- **Never panic on user input**
  - Always validate and return `Result` types
  - Use the `thiserror` crate for custom error types

Good:
```rust
pub fn parse_key(bytes: &[u8]) -> Result<Key, KeyError> {
    if bytes.len() != KEY_SIZE {
        return Err(KeyError::InvalidLength {
            expected: KEY_SIZE,
            actual: bytes.len(),
        });
    }
    // ... safe parsing logic
}
```

Bad:
```rust
pub fn parse_key(bytes: &[u8]) -> Key {
    assert_eq!(bytes.len(), KEY_SIZE); // DON'T PANIC ON INPUT
    // ...
}
```

### 3. Memory Safety

- **Always zero sensitive data before deallocation**
  - Use `MLockedBytes` for cryptographic secrets
  - Use `ptr::write_bytes(ptr, 0, len)` in Drop implementations
  
- **Be careful with memory locking**
  - Check ulimit settings don't prevent mlock
  - Handle mlock failures gracefully
  - Always munlock before freeing

### 4. Cryptographic Code

- **Never roll your own crypto**
  - Use established libraries (curve25519-dalek, ed25519-dalek, etc.)
  - Follow IETF standards and RFCs
  
- **Constant-time operations**
  - Use the `subtle` crate for equality checks on secrets
  - Avoid branches on secret data
  - Be aware of timing side-channels

Example:
```rust
use subtle::ConstantTimeEq;

// Good: constant-time comparison
if key1.ct_eq(key2).into() {
    // ...
}

// Bad: timing attack vulnerable
if key1 == key2 {  // DON'T DO THIS
    // ...
}
```

### 5. Integer Overflow

- Be aware of integer overflow in arithmetic
- Use checked arithmetic when dealing with sizes:
  ```rust
  let total = size1.checked_add(size2)
      .ok_or(Error::SizeOverflow)?;
  ```

### 6. Input Validation

- **Validate all inputs** at API boundaries
- Check length constraints before allocation
- Reject invalid data early with descriptive errors

### 7. Testing Security-Critical Code

- Write unit tests for error conditions
- Test with invalid/malformed inputs
- Add fuzzing targets for parsers
- Use test vectors from standards

Example test structure:
```rust
#[cfg(test)]
mod security_tests {
    use super::*;

    #[test]
    fn rejects_invalid_length() {
        let result = parse_key(&[0u8; 31]); // wrong size
        assert!(matches!(result, Err(KeyError::InvalidLength { .. })));
    }

    #[test]
    fn zeros_memory_on_drop() {
        let mut bytes = MLockedBytes::new(32).unwrap();
        bytes.as_mut_slice().fill(0xff);
        let ptr = bytes.as_ptr();
        drop(bytes);
        // Memory should be zeroed (requires careful testing)
    }
}
```

## Code Review Checklist

Before submitting a PR with security-sensitive code, verify:

- [ ] All `unsafe` blocks have SAFETY comments
- [ ] No `unwrap()` calls in production paths
- [ ] All user inputs are validated
- [ ] Sensitive data is zeroed on drop
- [ ] Error types are descriptive and don't leak secrets
- [ ] Constant-time operations for secrets
- [ ] Integer overflow checks where needed
- [ ] Tests cover error conditions
- [ ] No new compiler warnings
- [ ] Clippy passes with no warnings

## Reporting Security Issues

**DO NOT** open public issues for security vulnerabilities.

Instead, email: security@intersectmbo.org

Include:
- Description of the vulnerability
- Steps to reproduce
- Affected versions
- Potential impact

See [SECURITY.md](../SECURITY.md) for full details.

## Dependencies

### Adding New Dependencies

When adding dependencies:
1. Check the crate's security audit history
2. Verify the license is compatible (see deny.toml)
3. Review the crate's use of `unsafe` code
4. Check for active maintenance
5. Run `cargo audit` after adding

### Updating Dependencies

- Regularly update dependencies for security patches
- Test thoroughly after updates
- Check CHANGELOG for breaking changes
- Use `cargo outdated` to find updates

## Tools

### Required Tools

Install these tools for security checking:

```bash
# Security audit
cargo install cargo-audit

# License checking
cargo install cargo-deny

# Code coverage
cargo install cargo-tarpaulin

# Fuzzing (optional)
cargo install cargo-fuzz
```

### Running Security Checks

```bash
# Run all checks
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo audit
cargo deny check

# Generate coverage report
cargo tarpaulin --workspace --out Html
```

## Resources

- [Rust Security Guidelines](https://anssi-fr.github.io/rust-guide/)
- [OWASP Cryptographic Storage Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Cryptographic_Storage_Cheat_Sheet.html)
- [Secure Rust Guidelines](https://github.com/ANSSI-FR/rust-guide)
- [The Rustonomicon (Unsafe Rust)](https://doc.rust-lang.org/nomicon/)

## Contact

For security questions or concerns:
- Email: security@intersectmbo.org
- See [CONTRIBUTING.md](../CONTRIBUTING.md) for general contribution guidelines
