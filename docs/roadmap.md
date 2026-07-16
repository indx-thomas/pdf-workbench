# Roadmap

Dates are deliberately omitted until the native-engine integration spike produces measured evidence.

## Phase 0 — Foundation

- [x] Select Apache License 2.0 for the application and core
- [x] Establish a clean qpdf fork and pin it as a submodule
- [x] Assign qpdf structural and PDFium interactive responsibilities
- [x] Exclude JavaScript/V8, XFA, and hosted services from the core
- [ ] Build qpdf on Windows and expose one safe adapter operation
- [ ] Build PDFium on Windows with V8/XFA disabled and render one page
- [ ] Establish sanitized PDF fixtures and expected-results manifests
- [ ] Establish fixed-hardware startup, first-render, scroll, memory, and cancellation benchmarks
- [ ] Validate the local process-isolation and IPC approach

## Phase 1 — Fast reader platform

- Local service, authenticated IPC, job scheduling, cancellation, and privacy-safe diagnostics
- Open/render/search/select/copy, thumbnails, bookmarks, links, metadata, passwords, and print
- Atomic Save As, original preservation, tabs, keyboard navigation, and session recovery
- Portable and per-user Windows builds with side-by-side, no-reboot updates

## Phase 2 — Document workbench

- Combine, split, reorder, rotate, insert, extract, delete, and crop
- Fill/save/flatten AcroForms
- Standard annotations and review export
- Image/text conversion and isolated DOCX/ODF/HTML/EML conversion
- Batch and CLI parity

## Phase 3 — Trust tools

- Form field creation
- OCR and scan import
- Redaction and sanitization with adversarial verification
- Encryption and permission controls
- PDF/A validation and controlled conformance creation
- Signature inspection, validation, local signing, and timestamps

## Phase 4 — Authoring and automation

- Existing text/image editing with documented fidelity boundaries
- Templates, deep links, watched folders, and stable automation API
- Document comparison, Bates numbering, watermarks, and optimization
- Optional local/BYOK AI suggestions
- Linux and macOS packaging after Windows behaviour is stable

## Separate hosted track

Browser signing, envelopes, recipients, delivery, audit evidence, accounts, retention, and webhooks require a separate repository, licence, threat model, privacy review, and operational plan.
