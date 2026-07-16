# ADR 0003: Local-first process model

- Status: accepted
- Date: 2026-07-16

## Context

Performance, privacy, and resilience are primary product goals. PDFs and conversion inputs are also hostile-input surfaces. A monolithic UI process would make parser faults, expensive work, and security isolation harder to control.

## Decision

- Local reading and editing require neither an account nor a server upload.
- The desktop shell communicates with a local document service through authenticated named pipes on Windows and equivalent local IPC elsewhere.
- Native engines and conversion workers are isolated from the UI and restricted where feasible.
- Long-running work is asynchronous, prioritized, cancellable, and observable.
- Network access is denied to document workers by default.
- Remote AI and future hosted workflows require an explicit user action and visible data-routing disclosure.

## Consequences

- IPC contracts require versioning and compatibility tests.
- Document bytes should move by handles or bounded streams rather than unnecessary full copies.
- Crash recovery and job supervision are core platform responsibilities.
