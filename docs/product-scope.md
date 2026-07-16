# Product scope

## Product promise

> Open instantly. Keep documents local. Do serious PDF work without ceremony.

PDF Workbench aims to become a performant, full-featured, no-frills alternative for common professional PDF workflows.

## Target capabilities

### Read and navigate

- PDF 1.x and PDF 2.0 rendering
- PDF/A rendering with separate conformance validation
- password-protected PDFs
- text selection, search, thumbnails, outlines, named destinations, links, attachments, and layers
- large, linearized, damaged, and image-heavy documents

### Assemble and convert

- combine, split, insert, extract, reorder, duplicate, delete, rotate, and crop
- text, image, DOCX, ODF/ODT, HTML, and EML conversion
- scan import, OCR, deskew, and searchable text layers
- compression, optimization, normalization, metadata cleanup, headers/footers, Bates numbers, and watermarks

### Forms and review

- fill, create, edit, export, import, and flatten AcroForms
- FDF/XFDF form-data exchange
- standard PDF annotations, stamps, measurements, filters, summaries, and revision comparison
- XFA detection with a clear unsupported warning; no XFA or PDF JavaScript runtime

### Redaction and security

- content-removing text, image, area, page, regex, and OCR-assisted redaction
- metadata, attachment, action, script, layer, and prior-revision sanitization
- adversarial post-export verification and full-rewrite semantics
- modern PDF encryption and clearly described advisory permission flags

### Certificates and signing

- inspect and validate certificate signatures, chains, timestamps, revocation status, and post-sign changes
- visible/invisible local signatures using PFX/P12, Windows certificate store, and later PKCS#11
- PAdES and RFC 3161 interoperability goals

### Automation and optional AI

- versioned local operation API, CLI, batch jobs, watched folders, and templates
- deep links that cannot bypass file/network safety controls
- optional local or BYOK providers for cited Q&A, extraction, field suggestions, comparison, and redaction suggestions
- deterministic user approval for every AI-proposed document mutation

## Explicit non-goals

- PDF JavaScript or form scripting
- dynamic XFA creation or execution
- social feeds, reactions, profiles, or community features
- mandatory accounts or cloud storage for local work
- automatic embedded-program, attachment, external-file, or remote-content execution
- hosted signing code in this repository
