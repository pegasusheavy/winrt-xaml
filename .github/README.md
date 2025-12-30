# GitHub CI/CD Configuration

This directory contains GitHub Actions workflows for continuous integration and performance monitoring.

## 🔄 Workflows

### 1. Benchmark Performance (`benchmark.yml`)

**Triggers**: Pull requests, pushes to main/master, manual dispatch

**Jobs**:
- `benchmark`: Runs all benchmarks and uploads results
- `benchmark-comparison`: Compares PR performance against base branch

**Features**:
- Automatic benchmark execution on PRs
- Performance comparison comments
- Artifact uploads for detailed analysis
- Alert on >10% performance regression

### 2. Performance Tracking (`performance-tracking.yml`)

**Triggers**: Pushes to main/master, weekly schedule (Monday 00:00 UTC), manual dispatch

**Jobs**:
- `track-performance`: Records performance metrics over time
- `regression-check`: Detects regressions between commits

**Features**:
- Historical performance tracking
- Automatic regression detection
- Performance trends generation
- Weekly scheduled runs for stability monitoring

### 3. Performance Dashboard (`benchmark-dashboard.yml`)

**Triggers**: Pushes to main/master, manual dispatch

**Jobs**:
- `generate-dashboard`: Creates and deploys performance dashboard

**Features**:
- Visual performance dashboard
- Real-time metrics display
- GitHub Pages deployment
- Criterion HTML report integration

## 📊 Viewing Results

### Local Development

```powershell
# Run benchmarks locally
.\scripts\benchmark-local.ps1

# Compare with baseline
.\scripts\benchmark-local.ps1 -Compare -Baseline main

# Save new baseline
.\scripts\benchmark-local.ps1 -Save my-baseline

# Quick run (infrastructure only)
.\scripts\benchmark-local.ps1 -Quick
```

### CI Results

1. **PR Comments**: Automatic comments on PRs with benchmark results
2. **Artifacts**: Download detailed results from GitHub Actions artifacts
3. **Dashboard**: View live dashboard at `https://<org>.github.io/<repo>/`
4. **Criterion Reports**: HTML reports in artifacts under `benchmark-results`

## 🎯 Performance Targets

| Metric | Target | Current | Status |
|--------|--------|---------|--------|
| State Read | < 15ns | 11.5ns | ✅ |
| State Write | < 15ns | 10.7ns | ✅ |
| Vec Creation | < 100ns | 56ns | ✅ |
| Arc Access | < 10ns | 0.2ns | ✅ |
| Lock Acquisition | < 15ns | 12ns | ✅ |

## 🚨 Alerts

### Regression Thresholds

- **Warning**: >5% performance decrease
- **Alert**: >10% performance decrease
- **Fail**: Configurable per workflow

### Alert Actions

1. PR comment with details
2. Artifact upload with comparison
3. Optional workflow failure
4. Notification to maintainers

## 📝 CODEOWNERS

Performance-related files are automatically reviewed by maintainers:
- `/benches/`
- `*benchmark*.md`
- `*performance*.md`
- `*optimization*.md`
- `.github/workflows/benchmark*.yml`
- `.github/workflows/performance*.yml`

## 🔧 Configuration

### Enabling GitHub Pages

1. Go to repository Settings
2. Navigate to Pages
3. Source: Deploy from branch
4. Branch: `gh-pages`
5. Folder: `/ (root)`
6. Save

### Required Secrets

No additional secrets required beyond `GITHUB_TOKEN` (automatically provided).

### Optional Configuration

**benchmark-action/github-action-benchmark**:
- Adjust `alert-threshold` in `benchmark.yml`
- Modify `fail-on-alert` behavior
- Configure auto-push settings

## 📚 Documentation

- [OPTIMIZATION_GUIDE.md](../OPTIMIZATION_GUIDE.md) - Optimization patterns
- [BENCHMARK_SUMMARY.md](../BENCHMARK_SUMMARY.md) - Benchmark overview
- [README_BENCHMARKS.md](../README_BENCHMARKS.md) - Quick reference

## 🤝 Contributing

When making changes that may affect performance:

1. Run benchmarks locally first
2. Compare against main branch
3. Document intentional regressions
4. Update performance targets if needed

## 📞 Support

For questions about CI/CD or benchmarking:
- Check workflow logs in Actions tab
- Review documentation in `/docs`
- Open an issue for CI-specific problems

---

**Last Updated**: December 29, 2025
**Workflows**: 3 active
**Status**: ✅ Operational

