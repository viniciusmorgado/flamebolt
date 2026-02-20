# Build frontend (Trunk) then backend (Cargo)
$ErrorActionPreference = "Stop"
$flameboltDir = Join-Path $PSScriptRoot "flamebolt"

Push-Location $flameboltDir
try {
    trunk build
} finally {
    Pop-Location
}

Push-Location $PSScriptRoot
try {
    cargo build
} finally {
    Pop-Location
}
