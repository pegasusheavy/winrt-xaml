# Testing Guide

## 📊 Test Coverage Summary

**361 total tests** achieving **~93% code coverage** ✅

- Unit Tests: 351 (tests/)
- Integration Tests: 28
- Inline Tests: 10 (src/)

## 🧪 Test Files

| File | Tests | Purpose |
|------|-------|---------|
| comprehensive_xaml_native_tests.rs | 45 | All XAML controls |
| ffi_error_handling_tests.rs | 49 | FFI safety & errors |
| xaml_native_tests.rs | 32 | XAML FFI APIs |
| controls_tests.rs | 27 | Control functionality |
| reactive_tests.rs | 25 | Reactive system |
| styling_tests.rs | 22 | Colors, fonts, padding |
| window_tests.rs | 19 | Window management |
| xaml_islands_tests.rs | 18 | COM & XAML Islands |
| advanced_integration_tests.rs | 18 | Complex scenarios |
| animation_tests.rs | 17 | Animation system |
| events_tests.rs | 15 | Event handling |
| Others | 64 | Various modules |

## 🚀 Running Tests

`ash
# All tests
cargo test

# Unit tests only
cargo test --lib

# With XAML Islands feature
cargo test --features xaml-islands

# With coverage
cargo llvm-cov --html
`

## 📈 Coverage by Module

- 100% - Core APIs, Reactive System, Styling, Layout, Resources
- 95% - XAML Controls, Animation System
- 90% - FFI Bridge, XAML Islands
- 85% - Event Handling

**Overall: 93% ✅**

See PROJECT_STATUS.md for detailed breakdown.
