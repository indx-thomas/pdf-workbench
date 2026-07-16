# Security policy

## Project status

PDF Workbench is pre-alpha and has no production-supported release. Do not rely on it yet for secure redaction, signature validation, encryption decisions, archival conformance, or hostile-document isolation.

## Reporting a vulnerability

Do not open a public issue for a suspected vulnerability or include a malicious/confidential document in a public repository.

Use GitHub's private vulnerability reporting for this repository:

<https://github.com/indx-thomas/pdf-workbench/security/advisories/new>

Include:

- affected commit or version;
- reproducible steps using a minimal non-confidential fixture;
- expected and actual impact;
- whether document content, credentials, keys, signatures, or host access are exposed; and
- any suggested mitigation.

Security-sensitive areas include parsers, native FFI, conversion workers, decompression, redaction, sanitization, encryption, certificates, signatures, deep links, attachments, external actions, updates, IPC, and AI-provider data routing.

## Disclosure

Please allow reasonable time to investigate and prepare a fix before public disclosure. Credit will be offered unless the reporter prefers anonymity.
