$ErrorActionPreference = 'Stop'

$packageName = 'waspswithbazookas'
$toolsDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"

# Remove binaries
$binaries = @("hive.exe", "wasp.exe", "test-dummy.exe")
foreach ($binary in $binaries) {
    $binaryPath = Join-Path $toolsDir $binary
    if (Test-Path $binaryPath) {
        Remove-Item $binaryPath -Force
    }
} 