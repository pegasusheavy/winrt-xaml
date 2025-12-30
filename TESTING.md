# Testing Guide

## Running Tests

### All Tests

```powershell
cargo test --tests
```

### Specific Test Module

```powershell
# Error handling tests
cargo test --test error_tests

# Event system tests
cargo test --test events_tests

# Control tests
cargo test --test controls_tests

# Layout tests
cargo test --test layout_tests

# Media tests
cargo test --test media_tests

# Resource tests
cargo test --test resources_tests

# Integration tests
cargo test --test integration_tests
```

### With Output

```powershell
cargo test --tests -- --nocapture
```

## Test Coverage

### Test Statistics

- **Total Tests**: 85
- **Error Tests**: 7
- **Event Tests**: 15
- **Control Tests**: 27
- **Layout Tests**: 11
- **Media Tests**: 7
- **Resource Tests**: 8
- **Integration Tests**: 10

### Test Categories

#### Unit Tests
- `tests/error_tests.rs` - Error handling and Result types
- `tests/events_tests.rs` - Event subscription and invocation
- `tests/media_tests.rs` - Color and brush types
- `tests/resources_tests.rs` - Resource dictionary management

#### Component Tests
- `tests/controls_tests.rs` - UI control creation and configuration
- `tests/layout_tests.rs` - Layout panel behavior

#### Integration Tests
- `tests/integration_tests.rs` - Full application workflow tests

## Test Status

✅ **All 85 tests passing**

### Coverage by Module

| Module | Tests | Status |
|--------|-------|--------|
| Error Handling | 7 | ✅ Passing |
| Event System | 15 | ✅ Passing |
| Controls | 27 | ✅ Passing |
| Layout | 11 | ✅ Passing |
| Media | 7 | ✅ Passing |
| Resources | 8 | ✅ Passing |
| Integration | 10 | ✅ Passing |
| **Total** | **85** | **✅ All Passing** |

## Writing New Tests

### Test Structure

```rust
use winrt_xaml::prelude::*;

#[test]
fn test_my_feature() {
    // Arrange
    let control = Button::new().unwrap();

    // Act
    control.set_content("Test").unwrap();

    // Assert
    assert_eq!(control.content(), "Test");
}
```

### Testing Events

```rust
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

#[test]
fn test_event_handling() {
    let button = Button::new().unwrap();
    let counter = Arc::new(AtomicUsize::new(0));
    let counter_clone = counter.clone();

    button.click().subscribe(move |_| {
        counter_clone.fetch_add(1, Ordering::SeqCst);
    });

    assert_eq!(button.click().subscriber_count(), 1);
}
```

### Testing Fluent API

```rust
#[test]
fn test_fluent_builder() {
    let panel = StackPanel::new()
        .unwrap()
        .with_orientation(Orientation::Horizontal)
        .with_spacing(10)
        .with_padding((5, 5, 5, 5));

    assert_eq!(panel.orientation(), Orientation::Horizontal);
    assert_eq!(panel.spacing(), 10);
}
```

## CI/CD Integration

Tests are automatically run on:
- Pull requests
- Pushes to main branch
- Weekly scheduled runs

See `.github/workflows/benchmark.yml` for CI configuration.

## Known Limitations

### Windows-Only Tests

These tests require a Windows environment and will not run on Linux/macOS:
- Integration tests that create actual windows
- Tests that interact with Win32 APIs
- Visual rendering tests (future)

### Feature Gating

Tests can be run without any feature flags.

## Performance Tests

For performance benchmarking, see:
- `benches/controls_bench.rs` - Control creation benchmarks
- `benches/layout_bench.rs` - Layout calculation benchmarks
- `benches/xaml_parser_bench.rs` - XAML parsing benchmarks
- `benches/optimized_patterns.rs` - Optimization pattern benchmarks

Run benchmarks with:

```powershell
cargo bench
```

## Memory Testing

Memory profiling is available via `dhat`:

```powershell
cargo run --bin memory_profile --features "dhat-heap"
```

## Test Best Practices

1. **Feature Gate Tests**: Tests no longer require feature gates
2. **Isolate Tests**: Each test should be independent and not rely on state from other tests
3. **Clear Naming**: Use descriptive test names that indicate what is being tested
4. **Arrange-Act-Assert**: Follow AAA pattern for test structure
5. **Test Both Success and Failure**: Test happy paths and error conditions
6. **Document Edge Cases**: Add comments for non-obvious test scenarios

## Troubleshooting

### Tests Not Running

Run tests directly:

```powershell
cargo test --tests
```

### Win32 Conflicts

If tests fail due to Win32 window class conflicts, run with single thread:

```powershell
cargo test --tests -- --test-threads=1
```

### Compilation Errors

If examples cause compilation issues during testing, ensure `autobins = false` is set in `Cargo.toml`:

```toml
[package]
autobins = false
```

---

**Last Updated**: December 30, 2025

