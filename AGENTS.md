# Codex project guide

PDF Workbench is a pre-alpha, Apache-2.0, local-first PDF desktop application. Its goal is a fast, no-frills Acrobat alternative, with Windows as the first supported platform.

## Architecture rules

- Keep application-facing contracts engine-neutral in `crates/document-core/`.
- Use qpdf only for structural work in `crates/qpdf-adapter/`; its pinned source is `vendor/qpdf/`.
- Keep rendering and interactive PDF work behind `crates/pdfium-adapter/`.
- Do not add PDF JavaScript/V8, XFA, telemetry, or hosted-service code to this repository.
- Treat PDFs, conversions, certificates, and signatures as hostile or sensitive inputs. Do not commit real user documents, keys, passwords, or personal data.

## Key paths

| Path | Purpose |
|---|---|
| `apps/cli/` | Headless operations and integration smoke tests |
| `apps/desktop/` | Future Windows desktop client boundary |
| `crates/document-core/` | Stable capability and operation contracts |
| `crates/qpdf-adapter/` | qpdf structural adapter and tests |
| `crates/pdfium-adapter/` | Future rendering and forms adapter |
| `scripts/` | Reproducible native-engine build scripts |
| `docs/architecture.md` | System boundaries and process model |
| `docs/adr/` | Accepted architecture decisions |
| `docs/security/` | Threat model and security design |
| `docs/roadmap.md` | Ordered delivery plan |
| `vendor/qpdf/` | Pinned `pdf-workbench-qpdf` git submodule |

## Before submitting

Run `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, and `cargo test --workspace`. Keep changes focused, add tests for behaviour, and update architecture/security/compatibility documentation when a boundary changes. See `CONTRIBUTING.md` and `SECURITY.md` for the full policies.
