#!/usr/bin/env powershell
# Simple build test script
Write-Host "Testing cargo build..."
$result = cargo build 2>&1
if ($LASTEXITCODE -eq 0) {
    Write-Host "✅ BUILD SUCCESSFUL!" -ForegroundColor Green
} else {
    Write-Host "❌ BUILD FAILED" -ForegroundColor Red
    Write-Host $result
}
