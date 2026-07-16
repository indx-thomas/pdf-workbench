# PDF Workbench

[![CI](https://github.com/indx-thomas/pdf-workbench/actions/workflows/ci.yml/badge.svg)](https://github.com/indx-thomas/pdf-workbench/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

PDF Workbench is an experimental, local-first, high-performance PDF application for Windows. The goal is a practical, no-frills alternative for reading, assembling, converting, reviewing, redacting, securing, and signing PDF documents without requiring a cloud account.

> [!WARNING]
> This project is pre-alpha. It does not yet safely process production documents, perform verified redaction, or validate digital signatures.

## Product principles

- Local document work does not require an account or upload.
- Startup, first-page rendering, scrolling, cancellation, memory use, and updates have measurable performance budgets.
- Untrusted document content never silently executes code or accesses the network.
- Redaction, signature, and conformance indicators are evidence-based and conservative.
- Desktop, CLI, and future automation use the same versioned core operations.
- AI is optional, bring-your-own-provider, and outside the critical document path.

## Architecture

PDF Workbench deliberately uses separate engines for separate jobs:

| Component | Responsibility |
|---|---|
| `document-core` | Stable application-facing capabilities and operation contracts |
| `qpdf-adapter` | Structural operations such as merge, split, encryption, linearization, and normalization |
| `pdfium-adapter` | Rendering, text/search, navigation, AcroForms, annotations, and signature inspection |
| Desktop application | Commands, document canvas, overlays, accessibility, and local job UI |
| Isolated workers | Office/email conversion, OCR, and standards validation |

qpdf does not render pages or extract text, so it is not the viewer engine. The project pins the clean [`pdf-workbench-qpdf`](https://github.com/indx-thomas/pdf-workbench-qpdf) fork as a submodule while keeping application-specific code in this repository.

See [Architecture](docs/architecture.md), [Product scope](docs/product-scope.md), and the [Roadmap](docs/roadmap.md).

## Repository layout

```text
apps/
  cli/                 Minimal headless entry point
  desktop/             Desktop application boundary
crates/
  document-core/       Engine-neutral contracts
  pdfium-adapter/      Rendering and interactive PDF boundary
  qpdf-adapter/        Structural PDF boundary
docs/
  adr/                 Architecture decisions
  security/            Threat model and security design
scripts/               Reproducible native-engine builds
vendor/
  qpdf/                Pinned qpdf fork submodule
```

Cloud signing and other hosted services are intentionally excluded. If developed, they will live in a separate repository with their own licence, security model, and operational lifecycle.

## Development

Prerequisites:

- Git with submodule support
- Current stable Rust toolchain
- Native qpdf/PDFium prerequisites once their bindings are implemented

```bash
git clone --recurse-submodules https://github.com/indx-thomas/pdf-workbench.git
cd pdf-workbench
cargo test --workspace
cargo run -p pdf-workbench-cli -- --capabilities
```

On Windows, the first native integration builds the pinned qpdf fork and exposes a read-only structural check:

```powershell
./scripts/build-qpdf.ps1
$env:PDF_WORKBENCH_QPDF_BIN = (Resolve-Path ./build/qpdf-install/bin/qpdf.exe).Path
cargo run -p pdf-workbench-cli -- check ./document.pdf
```

See [qpdf integration](docs/qpdf-integration.md) for prerequisites, exit semantics, and current isolation limitations. The desktop shell and PDFium integration are not implemented yet.

## Contributing and security

See [CONTRIBUTING.md](CONTRIBUTING.md) before proposing changes. Please report suspected security issues privately according to [SECURITY.md](SECURITY.md), especially issues involving redaction, signatures, encryption, parsing, or unintended document disclosure.

## Licence

PDF Workbench is licensed under the [Apache License 2.0](LICENSE). Third-party components retain their own licences and notices; see [THIRD_PARTY.md](THIRD_PARTY.md).
