# Compatibility policy

This document describes intended scope, not current production support. The project is pre-alpha until release notes say otherwise.

| Format or feature | Intended treatment |
|---|---|
| PDF 1.x | Render, inspect, and edit within engine capabilities |
| PDF 2.0 / ISO 32000-2 | Target modern normative behaviour and current errata |
| PDF/A | Render normally; validate claims separately; controlled creation before arbitrary remediation |
| AcroForms | Fill first, then create/edit/import/export/flatten |
| FDF/XFDF | Target interoperable form and annotation data exchange |
| XFA/XDP | Detect and warn; no dynamic execution or creation |
| PDF JavaScript | Never execute |
| PDF signatures | Inspect/validate before implementing creation; warn before invalidating changes |
| Embedded files | Inspect/export with explicit user action; never auto-open |
| Launch/external actions | Do not execute automatically |
| Multimedia/Flash | Not supported |

Each release must publish a tested compatibility matrix based on sanitized fixtures rather than feature claims inferred solely from an underlying engine.
