# Local benchmark runner script
# Usage: .\scripts\benchmark-local.ps1 [-Compare] [-Baseline <name>] [-Save <name>]

param(
    [switch]$Compare,
    [string]$Baseline = "main",
    [string]$Save = "",
    [switch]$All,
    [switch]$Quick
)

$ErrorActionPreference = "Stop"

Write-Host "🚀 WinRT-XAML Local Benchmark Runner" -ForegroundColor Cyan
Write-Host "=====================================" -ForegroundColor Cyan
Write-Host ""

# Check if we're in the right directory
if (-not (Test-Path "Cargo.toml")) {
    Write-Host "❌ Error: Must be run from project root" -ForegroundColor Red
    exit 1
}

# Build the project first
Write-Host "📦 Building project..." -ForegroundColor Yellow
cargo build --release --no-default-features
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Build failed!" -ForegroundColor Red
    exit 1
}

# Determine which benchmarks to run
$benchmarks = @()
if ($All) {
    $benchmarks = @("infrastructure_test", "optimized_patterns")
} elseif ($Quick) {
    $benchmarks = @("infrastructure_test")
} else {
    $benchmarks = @("infrastructure_test", "optimized_patterns")
}

# Run benchmarks
foreach ($bench in $benchmarks) {
    Write-Host ""
    Write-Host "🔬 Running benchmark: $bench" -ForegroundColor Green
    Write-Host "----------------------------------------" -ForegroundColor Gray

    $args = @("bench", "--bench", $bench, "--no-default-features", "--")

    if ($Save -ne "") {
        $args += "--save-baseline"
        $args += $Save
        Write-Host "💾 Saving baseline as: $Save" -ForegroundColor Yellow
    }

    if ($Compare) {
        $args += "--baseline"
        $args += $Baseline
        Write-Host "📊 Comparing against baseline: $Baseline" -ForegroundColor Yellow
    }

    & cargo $args

    if ($LASTEXITCODE -ne 0) {
        Write-Host "⚠️ Benchmark failed: $bench" -ForegroundColor Yellow
    } else {
        Write-Host "✅ Benchmark completed: $bench" -ForegroundColor Green
    }
}

# Open HTML report if available
Write-Host ""
Write-Host "📊 Opening benchmark report..." -ForegroundColor Cyan
$reportPath = "target\criterion\report\index.html"
if (Test-Path $reportPath) {
    Start-Process $reportPath
    Write-Host "✅ Report opened in browser" -ForegroundColor Green
} else {
    Write-Host "⚠️ Report not found at: $reportPath" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "✨ Benchmark run complete!" -ForegroundColor Cyan
Write-Host ""
Write-Host "Next steps:" -ForegroundColor White
Write-Host "  • Review HTML reports in target/criterion/" -ForegroundColor Gray
Write-Host "  • Compare results using --Compare flag" -ForegroundColor Gray
Write-Host "  • Save baselines using --Save <name>" -ForegroundColor Gray
Write-Host ""

