# Compare two benchmark baselines
# Usage: .\scripts\compare-benchmarks.ps1 -Base <baseline1> -Compare <baseline2>

param(
    [Parameter(Mandatory=$true)]
    [string]$Base,

    [Parameter(Mandatory=$true)]
    [string]$Compare,

    [string]$Benchmark = "infrastructure_test"
)

$ErrorActionPreference = "Stop"

Write-Host "📊 Benchmark Comparison Tool" -ForegroundColor Cyan
Write-Host "=============================" -ForegroundColor Cyan
Write-Host ""
Write-Host "Comparing:" -ForegroundColor White
Write-Host "  Base:    $Base" -ForegroundColor Gray
Write-Host "  Compare: $Compare" -ForegroundColor Gray
Write-Host "  Benchmark: $Benchmark" -ForegroundColor Gray
Write-Host ""

# Check if baselines exist
$basePath = "target\criterion\$Benchmark\$Base"
$comparePath = "target\criterion\$Benchmark\$Compare"

if (-not (Test-Path $basePath)) {
    Write-Host "❌ Base baseline not found: $basePath" -ForegroundColor Red
    Write-Host "Available baselines:" -ForegroundColor Yellow
    Get-ChildItem "target\criterion\$Benchmark" -Directory | ForEach-Object { Write-Host "  • $($_.Name)" -ForegroundColor Gray }
    exit 1
}

if (-not (Test-Path $comparePath)) {
    Write-Host "❌ Compare baseline not found: $comparePath" -ForegroundColor Red
    Write-Host "Available baselines:" -ForegroundColor Yellow
    Get-ChildItem "target\criterion\$Benchmark" -Directory | ForEach-Object { Write-Host "  • $($_.Name)" -ForegroundColor Gray }
    exit 1
}

# Run comparison
Write-Host "🔬 Running comparison..." -ForegroundColor Yellow
Write-Host ""

cargo bench --bench $Benchmark --no-default-features -- --baseline $Base --save-baseline $Compare > comparison-output.txt 2>&1

# Parse and display results
$output = Get-Content comparison-output.txt -Raw
Write-Host $output

# Analyze changes
Write-Host ""
Write-Host "📈 Analysis Summary" -ForegroundColor Cyan
Write-Host "===================" -ForegroundColor Cyan
Write-Host ""

$improvements = ($output | Select-String -Pattern "change:.*\-[0-9]+\.[0-9]+%" -AllMatches).Matches.Count
$regressions = ($output | Select-String -Pattern "change:.*\+[0-9]+\.[0-9]+%" -AllMatches).Matches.Count
$noChange = ($output | Select-String -Pattern "No change" -AllMatches).Matches.Count

Write-Host "Improvements: $improvements" -ForegroundColor Green
Write-Host "Regressions:  $regressions" -ForegroundColor $(if ($regressions -gt 0) { "Red" } else { "Gray" })
Write-Host "No Change:    $noChange" -ForegroundColor Gray
Write-Host ""

if ($regressions -gt 0) {
    Write-Host "⚠️ Performance regressions detected!" -ForegroundColor Yellow
    Write-Host "Review the detailed output above for specifics." -ForegroundColor Yellow
} elseif ($improvements -gt 0) {
    Write-Host "✅ Performance improvements detected!" -ForegroundColor Green
} else {
    Write-Host "ℹ️ No significant performance changes" -ForegroundColor Cyan
}

Write-Host ""
Write-Host "Full output saved to: comparison-output.txt" -ForegroundColor Gray
Write-Host ""

