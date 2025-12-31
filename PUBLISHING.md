# Publishing Guide - winrt-xaml v0.3.0

This guide covers the steps to publish `winrt-xaml` and `winrt-xaml-macros` to crates.io.

## Pre-Publication Checklist

### ✅ Completed
- [x] Version bumped to 0.3.0 in all Cargo.toml files
- [x] CHANGELOG.md created with complete release notes
- [x] LICENSE-MIT and LICENSE-APACHE added
- [x] Package metadata updated (description, repository, keywords, etc.)
- [x] All examples working and tested
- [x] All tests passing (144 tests)
- [x] All clippy warnings fixed
- [x] Documentation complete and up-to-date
- [x] README.md updated
- [x] Release branch created
- [x] Main branch created and tagged (v0.3.0)

### 📋 Pre-Publish Verification

Before publishing, verify the following:

```bash
# 1. Clean build succeeds
cargo clean
cargo build --release --features xaml-islands

# 2. All tests pass
cargo test --features xaml-islands

# 3. All examples compile
cargo build --examples --features xaml-islands

# 4. Documentation builds
cargo doc --no-deps --features xaml-islands

# 5. Package can be created
cargo package --allow-dirty
```

## Publishing Steps

### Step 1: Login to crates.io

```bash
cargo login
```

You'll need your crates.io API token. Get it from: https://crates.io/settings/tokens

### Step 2: Publish the Macro Crate First

The main crate depends on `winrt-xaml-macros`, so publish it first:

```bash
cd winrt-xaml-macros
cargo publish --dry-run
# Review the output, then:
cargo publish
cd ..
```

**Wait 2-3 minutes** for the crate to be available on crates.io before proceeding.

### Step 3: Update Main Crate Dependency

Update `Cargo.toml` to use the published macro crate:

```toml
[dependencies]
winrt-xaml-macros = { version = "0.3.0", path = "winrt-xaml-macros" }
```

Change to:

```toml
[dependencies]
winrt-xaml-macros = "0.3.0"
```

Commit this change:

```bash
git add Cargo.toml
git commit -m "chore: Use published winrt-xaml-macros from crates.io"
git push origin main
git push origin develop
```

### Step 4: Publish the Main Crate

```bash
cargo publish --dry-run
# Review the output, verify all files are included
cargo publish
```

### Step 5: Create GitHub Release

Go to: https://github.com/pegasusheavy/winrt-xaml/releases/new

**Tag**: v0.3.0
**Title**: Release v0.3.0 - Reactive Data Binding & Enhanced Controls

**Description**: (Copy from CHANGELOG.md or use the PR body)

```markdown
## Release v0.3.0

Production-ready Rust library for creating modern Windows UIs using WinRT and XAML with reactive data binding.

### 🎯 Major Features

#### Reactive Data Binding System
- **Property<T>**: Observable values with subscriber notifications
- **ObservableCollection<T>**: Reactive collections with change tracking
- **Computed<T>**: Derived values that automatically update
- Thread-safe reactive primitives using Arc<Mutex<_>>

#### New Controls
- CheckBox, ComboBox, Slider, ProgressBar, RadioButton

#### Compile-Time XAML
- `xaml!` procedural macro for zero-overhead XAML parsing
- Compile-time error checking
- Serde-compliant deserialization

### 📊 Performance
- Property operations: 30-80ns
- Notifications (10 subscribers): <1µs
- 144 unit tests, 75-100% coverage

### 📦 Installation

```toml
[dependencies]
winrt-xaml = { version = "0.3.0", features = ["xaml-islands"] }
```

See [CHANGELOG.md](CHANGELOG.md) for complete details.
```

**Assets**: The GitHub action will automatically attach the source code archives.

### Step 6: Announce the Release

Consider announcing on:
- Reddit: /r/rust
- Twitter/X: #rustlang
- This Week in Rust newsletter
- Rust Users forum

### Step 7: Update Documentation Site

If you have a docs site (e.g., docs.rs or custom), ensure it's updated:

```bash
# docs.rs will automatically build documentation
# Verify at: https://docs.rs/winrt-xaml/0.3.0/
```

## Post-Publication Checklist

- [ ] Verify crate appears on crates.io: https://crates.io/crates/winrt-xaml
- [ ] Verify documentation on docs.rs: https://docs.rs/winrt-xaml
- [ ] Test installation in a fresh project
- [ ] Update GitHub repository settings to use `main` as default branch
- [ ] Create announcement blog post/tweet
- [ ] Monitor for issues/feedback

## Troubleshooting

### "crate not found" errors

If users report they can't find the crate immediately after publishing, it may take a few minutes for crates.io to index it. Ask them to wait 2-3 minutes and try again.

### Missing files in published crate

Ensure your `.gitignore` doesn't exclude necessary files. Check with:

```bash
cargo package --list
```

### Documentation build failures on docs.rs

Check the docs.rs build log. Common issues:
- Missing feature flags
- Platform-specific dependencies
- Network access during build (not allowed)

For Windows-specific crates, add this to `Cargo.toml`:

```toml
[package.metadata.docs.rs]
default-target = "x86_64-pc-windows-msvc"
targets = ["x86_64-pc-windows-msvc"]
features = ["xaml-islands"]
```

## Version Bumping for Future Releases

For the next release:

```bash
# Update version in Cargo.toml files
# Update CHANGELOG.md
# Create release branch
git checkout -b release/v0.4.0
git push -u origin release/v0.4.0
# Create PR to main
# After merge:
git checkout main
git tag -a v0.4.0 -m "Release v0.4.0"
git push origin v0.4.0
# Publish to crates.io
```

## Support

- GitHub Issues: https://github.com/pegasusheavy/winrt-xaml/issues
- Discussions: https://github.com/pegasusheavy/winrt-xaml/discussions
- Email: [Your support email]

## License

This project is licensed under MIT OR Apache-2.0.
