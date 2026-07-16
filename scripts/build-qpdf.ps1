[CmdletBinding()]
param(
    [string] $SourceDir,
    [string] $BuildDir,
    [string] $InstallDir,
    [ValidateSet("Debug", "Release", "RelWithDebInfo", "MinSizeRel")]
    [string] $Configuration = "Release",
    [string] $Triplet = "x64-windows-static"
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$repositoryRoot = Split-Path -Parent $PSScriptRoot
if (-not $SourceDir) {
    $SourceDir = Join-Path $repositoryRoot "vendor/qpdf"
}
if (-not $BuildDir) {
    $BuildDir = Join-Path $repositoryRoot "build/qpdf"
}
if (-not $InstallDir) {
    $InstallDir = Join-Path $repositoryRoot "build/qpdf-install"
}

$SourceDir = [IO.Path]::GetFullPath($SourceDir)
$BuildDir = [IO.Path]::GetFullPath($BuildDir)
$InstallDir = [IO.Path]::GetFullPath($InstallDir)

if (-not (Test-Path (Join-Path $SourceDir "CMakeLists.txt") -PathType Leaf)) {
    throw "qpdf source was not found at '$SourceDir'. Run 'git submodule update --init --recursive'."
}

$vcpkgRoots = @(
    $env:VCPKG_ROOT,
    $env:VCPKG_INSTALLATION_ROOT,
    "C:\vcpkg"
) | Where-Object { $_ } | Select-Object -Unique

$vcpkgRoot = $vcpkgRoots |
    Where-Object { Test-Path (Join-Path $_ "scripts/buildsystems/vcpkg.cmake") -PathType Leaf } |
    Select-Object -First 1

if (-not $vcpkgRoot) {
    throw "vcpkg was not found. Set VCPKG_ROOT or VCPKG_INSTALLATION_ROOT."
}

$vcpkg = Join-Path $vcpkgRoot "vcpkg.exe"
if (-not (Test-Path $vcpkg -PathType Leaf)) {
    throw "vcpkg.exe was not found at '$vcpkg'. Bootstrap the selected vcpkg installation."
}

Write-Host "Installing qpdf dependencies for $Triplet"
& $vcpkg install "zlib:$Triplet" "libjpeg-turbo:$Triplet"
if ($LASTEXITCODE -ne 0) {
    throw "vcpkg dependency installation failed with exit code $LASTEXITCODE."
}

$toolchain = Join-Path $vcpkgRoot "scripts/buildsystems/vcpkg.cmake"
$cmakeArguments = @(
    "-S", $SourceDir,
    "-B", $BuildDir,
    "-G", "Visual Studio 17 2022",
    "-A", "x64",
    "-DCMAKE_TOOLCHAIN_FILE=$toolchain",
    "-DVCPKG_TARGET_TRIPLET=$Triplet",
    "-DCMAKE_INSTALL_PREFIX=$InstallDir",
    "-DBUILD_SHARED_LIBS=OFF",
    "-DBUILD_STATIC_LIBS=ON",
    "-DUSE_IMPLICIT_CRYPTO=OFF",
    "-DREQUIRE_CRYPTO_NATIVE=ON",
    "-DBUILD_DOC=OFF",
    "-DGENERATE_AUTO_JOB=OFF",
    "-DINSTALL_MANUAL=OFF",
    "-DINSTALL_EXAMPLES=OFF",
    "-DINSTALL_PKGCONFIG=OFF",
    "-DINSTALL_CMAKE_PACKAGE=OFF"
)

Write-Host "Configuring qpdf from $SourceDir"
& cmake @cmakeArguments
if ($LASTEXITCODE -ne 0) {
    throw "qpdf configuration failed with exit code $LASTEXITCODE."
}

Write-Host "Building qpdf ($Configuration)"
& cmake --build $BuildDir --config $Configuration --parallel --target qpdf
if ($LASTEXITCODE -ne 0) {
    throw "qpdf build failed with exit code $LASTEXITCODE."
}

Write-Host "Installing the qpdf command-line component"
& cmake --install $BuildDir --config $Configuration --component cli
if ($LASTEXITCODE -ne 0) {
    throw "qpdf installation failed with exit code $LASTEXITCODE."
}

$qpdf = Join-Path $InstallDir "bin/qpdf.exe"
if (-not (Test-Path $qpdf -PathType Leaf)) {
    throw "The qpdf executable was not installed at '$qpdf'."
}

& $qpdf --version
if ($LASTEXITCODE -ne 0) {
    throw "The installed qpdf executable failed its version smoke test."
}

Write-Output ([IO.Path]::GetFullPath($qpdf))
