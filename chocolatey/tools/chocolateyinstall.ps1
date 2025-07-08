$ErrorActionPreference = 'Stop'

$packageName = 'waspswithbazookas'
$toolsDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"
$url = 'https://github.com/Phara0h/WaspsWithBazookas/releases/download/v2.0.0/waspswithbazookas-windows-x86_64.tar.gz'
$checksum = 'PLACEHOLDER_SHA256'
$checksumType = 'sha256'

# Download and extract
$tempDir = Join-Path $env:TEMP $packageName
New-Item -ItemType Directory -Path $tempDir -Force | Out-Null

try {
    $file = Join-Path $tempDir "waspswithbazookas.tar.gz"
    Get-ChocolateyWebFile -PackageName $packageName -FileFullPath $file -Url $url -Checksum $checksum -ChecksumType $checksumType
    
    # Extract using 7-Zip if available, otherwise use tar
    $7zPath = Get-Command 7z -ErrorAction SilentlyContinue
    if ($7zPath) {
        & 7z x $file -o"$tempDir" -y | Out-Null
        & 7z x (Join-Path $tempDir "waspswithbazookas.tar") -o"$tempDir" -y | Out-Null
    } else {
        tar -xzf $file -C $tempDir
    }
    
    # Install binaries
    $binaries = @("hive.exe", "wasp.exe", "test-dummy.exe")
    foreach ($binary in $binaries) {
        $source = Join-Path $tempDir $binary
        if (Test-Path $source) {
            Copy-Item $source $toolsDir -Force
        }
    }
} finally {
    Remove-Item $tempDir -Recurse -Force -ErrorAction SilentlyContinue
} 