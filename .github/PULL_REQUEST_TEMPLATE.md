# Pull Request

## Description

<!-- Provide a clear and concise description of your changes -->

## Motivation and Context

<!-- Why is this change required? What problem does it solve? -->
<!-- If it fixes an open issue, please link to the issue here -->

Fixes #(issue)

## Type of Change

<!-- Please check all that apply -->

- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] Documentation update
- [ ] Performance improvement
- [ ] Code refactoring
- [ ] Test addition or update

## Changes Made

<!-- List the main changes in your PR -->

-
-
-

## Testing

<!-- Describe the tests you ran to verify your changes -->

### Test Coverage

- [ ] All new code has unit tests
- [ ] All existing tests pass: `cargo test --workspace`
- [ ] Added integration tests if applicable
- [ ] Property-based tests added if applicable

### Test Commands Run

```bash
# Paste the test commands you ran
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all --check
```

## Documentation

- [ ] Code is documented with doc comments
- [ ] README updated if needed
- [ ] CHANGELOG.md updated
- [ ] API documentation generated: `cargo doc --workspace --no-deps`

## Security

- [ ] No unsafe code added (or justified if necessary)
- [ ] Security implications considered
- [ ] Cryptographic changes reviewed (if applicable)
- [ ] No secrets or sensitive data in code

## Checklist

- [ ] My code follows the project's style guidelines
- [ ] I have performed a self-review of my code
- [ ] I have commented my code, particularly in hard-to-understand areas
- [ ] My changes generate no new warnings
- [ ] I have added tests that prove my fix is effective or that my feature works
- [ ] New and existing unit tests pass locally with my changes
- [ ] Any dependent changes have been merged and published

## Additional Notes

<!-- Any additional information for reviewers -->
