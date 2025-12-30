# Pull Request

## Description

<!-- Provide a clear description of the changes -->

## Type of Change

<!-- Check all that apply -->

- [ ] 🐛 Bug fix (non-breaking change fixing an issue)
- [ ] ✨ New feature (non-breaking change adding functionality)
- [ ] 💥 Breaking change (fix or feature causing existing functionality to change)
- [ ] 📚 Documentation update
- [ ] ⚡ Performance improvement
- [ ] ♻️ Code refactoring
- [ ] 🧪 Test addition/modification
- [ ] 🔧 Build/CI changes

## Related Issues

<!-- Link related issues: Fixes #123, Closes #456, Related to #789 -->

Fixes #

## Changes Made

<!-- List the specific changes -->

-
-
-

## Testing

<!-- Describe the tests you ran -->

- [ ] Unit tests pass (`cargo test`)
- [ ] Benchmarks run (`cargo bench --no-default-features`)
- [ ] Examples work (`cargo run --example ...`)
- [ ] Manual testing performed

### Test Details

<!-- Provide test output or describe manual testing -->

```bash
# Test commands and output
cargo test
```

## Performance Impact

<!-- For performance-related changes -->

### Benchmark Results

```bash
# Before changes
cargo bench --bench infrastructure_test -- --save-baseline before

# After changes
cargo bench --bench infrastructure_test -- --baseline before
```

**Performance change**: <!-- e.g., "5% faster" or "No measurable change" -->

- [ ] No performance regression
- [ ] Performance improvement documented
- [ ] Performance regression justified

## Breaking Changes

<!-- If applicable, describe breaking changes and migration path -->

**Breaking changes**: <!-- Yes/No -->

**Migration guide**: <!-- If yes, provide migration steps -->

## Checklist

<!-- Check all that apply -->

- [ ] Code follows project style guidelines
- [ ] Self-review completed
- [ ] Comments added for complex code
- [ ] Documentation updated
- [ ] No new warnings introduced
- [ ] Tests added for new features
- [ ] All tests passing
- [ ] CHANGELOG.md updated
- [ ] Performance guidelines followed (see `.cursor/rules/performance-optimization.mdc`)

## Screenshots

<!-- If applicable, add screenshots -->

## Additional Notes

<!-- Any additional information for reviewers -->

---

**Reviewers**: Please check performance benchmarks if this PR affects hot paths.

