# ADR 0002: PDF engine boundaries

- Status: accepted
- Date: 2026-07-16

## Context

No single selected permissively licensed engine covers every required workflow. qpdf is strong at content-preserving structural transformations but explicitly does not render PDFs or extract text. PDFium provides rendering and interactive document APIs and can be built without JavaScript/V8 and XFA.

MuPDF was considered but rejected because its AGPL/commercial licensing does not match the Apache-2.0 core decision.

## Decision

- qpdf owns merge, split, encryption, linearization, normalization, and related structural operations.
- PDFium owns rendering, text extraction/search, navigation, AcroForm interaction, annotations, and signature inspection.
- Engine-specific code stays behind separate adapter crates/processes.
- `pdf-workbench-qpdf` is pinned as `vendor/qpdf`; patches should be minimal and upstreamable where practical.
- PDFium will initially be pinned reproducibly from its active official GoogleSource repository before its first binding is merged.
- A `pdf-workbench-pdfium` mirror/fork will be created only when a downstream patch is actually required. It must mirror the active GoogleSource upstream rather than an archived GitHub mirror.

## Consequences

- Cross-engine round-trip and differential tests are mandatory.
- The application core must not expose native engine types.
- Advanced content editing, form creation, redaction, and signing require project-owned higher-level logic and dedicated validation.
- Avoiding a premature PDFium mirror reduces synchronization and security-update maintenance.
