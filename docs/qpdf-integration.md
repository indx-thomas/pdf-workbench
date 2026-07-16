# qpdf integration

The first native-engine slice builds the pinned qpdf fork on Windows and exposes qpdf's read-only structural check through `qpdf-adapter` and the CLI. It proves the source pin, native build, process boundary, non-Unicode path handling, exit semantics, and CI path before any document mutation is added.

## Build on Windows

Prerequisites are a Visual Studio installation with C++ tools, CMake, PowerShell 7, and vcpkg. Set `VCPKG_ROOT` or `VCPKG_INSTALLATION_ROOT`, then run:

```powershell
git submodule update --init --recursive
./scripts/build-qpdf.ps1
```

The script builds a static qpdf executable at `build/qpdf-install/bin/qpdf.exe`. It uses the pinned submodule, zlib, libjpeg-turbo, and qpdf's native crypto provider. Validated dependency paths are passed explicitly because qpdf's current CMake discovery does not consume vcpkg package targets. The crypto-provider decision must be reviewed before encryption features are considered production-ready.

CMake selects the newest supported installed Visual Studio generator. To select one explicitly, pass `-Generator`, for example `-Generator "Visual Studio 17 2022"`.

## Run a structural check

```powershell
$env:PDF_WORKBENCH_QPDF_BIN = (Resolve-Path ./build/qpdf-install/bin/qpdf.exe).Path
cargo run -p pdf-workbench-cli -- check ./document.pdf
```

The adapter passes arguments directly to the executable without a shell and requires explicit, resolvable executable and input paths.

| CLI exit code | Meaning |
|---:|---|
| `0` | qpdf found no structural errors or warnings |
| `2` | configuration, invocation, or qpdf structural failure |
| `3` | qpdf found recoverable warnings but no errors |

`--check` is a syntax and structure check. A clean result is not a PDF/A conformance result, a malware verdict, or proof that page content is semantically correct.

## Transitional limitations

This spike launches qpdf directly and captures its output. Before untrusted production documents are supported, invocation must move into the restricted local worker with a deadline, cancellation, resource limits, bounded diagnostic output, and privacy-safe logging. Mutation operations also require atomic output and original-file preservation contracts.
