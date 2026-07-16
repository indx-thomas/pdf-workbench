# Contributing

PDF Workbench is an early personal open-source project. Small, focused changes with tests and a clear rationale are welcome.

## Before opening a change

1. Check the roadmap and existing issues.
2. Open an issue before large architecture, format-support, cryptography, redaction, or hosted-service work.
3. Never include confidential PDFs, certificates, private keys, passwords, personal information, or proprietary test files.
4. Report security-sensitive findings privately according to `SECURITY.md`.

## Development workflow

```bash
git clone --recurse-submodules https://github.com/indx-thomas/pdf-workbench.git
cd pdf-workbench
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Windows contributors can build the pinned native qpdf executable with `./scripts/build-qpdf.ps1`; see `docs/qpdf-integration.md` for prerequisites and the opt-in integration test.

Pull requests should:

- solve one coherent problem;
- explain user and developer impact;
- include or update tests;
- update compatibility, security, and architecture documentation when behaviour changes;
- avoid unrelated formatting or dependency changes; and
- identify any input that crosses a trust boundary or performs network/file-system access.

## Engine boundaries

- Product code belongs in `pdf-workbench`.
- qpdf changes belong in `pdf-workbench-qpdf` and should remain narrowly scoped and suitable for upstream contribution when possible.
- PDFium integration stays behind `pdfium-adapter`.
- Hosted/cloud service code does not belong in this repository.

## Licensing contributions

Unless explicitly marked otherwise before submission, contributions intentionally submitted for inclusion are provided under Apache License 2.0, consistent with section 5 of that licence. No contributor licence agreement is currently required.
