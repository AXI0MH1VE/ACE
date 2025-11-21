$ErrorActionPreference = "Stop"

Write-Host "Building AxiomHive CLI + Tauri app..."
cargo build
Push-Location src-tauri
cargo build
Pop-Location

Write-Host "Packaging public assets..."
if (-not (Test-Path dist)) { New-Item -ItemType Directory -Path dist | Out-Null }
Copy-Item -Recurse -Force public dist/public
Write-Host "Done. Produced debug artifacts in target/ and src-tauri/target/."
