# ADR 0001: Licensing and repository boundaries

- Status: accepted
- Date: 2026-07-16

## Context

PDF Workbench is primarily a personal open-source project. Broad reuse and commercial embedding are acceptable. Hosted services may eventually require monetization and a different source-sharing policy.

## Decision

1. `pdf-workbench` is licensed under Apache License 2.0.
2. Product code, engine-neutral core contracts, desktop code, CLI code, and local workers live here.
3. `pdf-workbench-qpdf` remains a narrow fork of qpdf under qpdf's existing licence terms.
4. Hosted/cloud service code will live in a separate repository. It may use AGPL-3.0 or remain private, but that decision does not alter this repository's licence.
5. Shared interfaces must be versioned so a service can consume released Apache-2.0 core artefacts without copying cloud concerns into the core.

## Consequences

- Closed-source forks and commercial products may reuse the core.
- Contributions intentionally submitted here are Apache-2.0 unless explicitly agreed otherwise before inclusion.
- Dependency and attribution records are required before redistribution.
- AGPL dependencies are not accepted into the core without superseding this decision through a new ADR.
