$ErrorActionPreference = "Stop"
Invoke-WebRequest -Uri https://astral.sh/uv/install.ps1 -OutFile install.ps1
./install.ps1
uv sync
Write-Host "Ace installed. Run: uv run python src/ace.py"