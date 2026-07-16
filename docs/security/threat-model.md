# Initial threat model

Status: foundation draft. Update this document before each security-sensitive feature ships.

## Protected assets

- document contents and metadata;
- passwords, API keys, certificates, and private keys;
- integrity of redacted and signed output;
- user filesystem and operating-system account;
- update and release integrity; and
- privacy of recent-file, cache, OCR, and diagnostic data.

## Trust boundaries

1. Desktop UI to local document service
2. Local service to qpdf/PDFium native code
3. Local service to conversion, OCR, and validation workers
4. Application to filesystem, clipboard, printer, scanner, certificate store, and external URLs
5. Application to an explicitly selected AI or hosted provider
6. Updater to release infrastructure

## Principal threats and initial controls

| Threat | Initial control direction |
|---|---|
| Parser memory corruption | Process isolation, current pinned engines, fuzzing, sanitizers, rapid security updates |
| Decompression or resource exhaustion | Input limits, timeouts, bounded caches, job cancellation, worker termination |
| Script/action execution | PDFium without V8/XFA; no automatic launch, attachment, or external-file actions |
| Silent data exfiltration | Workers have no network by default; remote operations disclose provider and payload |
| Redaction recovery | Remove underlying objects, full rewrite, scan prior revisions/attachments/layers, adversarial verification |
| Signature misrepresentation | Separate cryptographic validity, trust chain, revocation, timestamp, and post-sign changes in UI |
| Private-key disclosure | OS credential/certificate facilities; non-exportable keys where supported; no sensitive logging |
| Unsafe deep links | Opaque identifiers, allowlisted actions, confirmation, no path/content secrets in URLs |
| Malicious updates | Signed releases, checksums, least-privilege updater, atomic side-by-side activation, rollback |
| Sensitive diagnostics | Structured errors without document text; explicit opt-in for sanitized diagnostic bundles |

## Security release gates

Redaction, sanitization, certificate signing, signature validation, encryption, deep links, updates, and hosted data routing must each have:

- a feature-specific threat-model update;
- negative and adversarial fixtures;
- automated verification of security invariants;
- conservative user-facing failure states; and
- independent review before a production-safety claim.
