## What changed

Describe the coherent change in this pull request.

## Why

Explain the user or developer problem being addressed.

## Security and privacy

- What untrusted input or trust boundary is affected?
- Does any document content, credential, key, path, or metadata leave the device?
- Could the change affect redaction, signatures, encryption, parsing, updates, or external actions?

## Validation

- [ ] `cargo fmt --all --check`
- [ ] `cargo clippy --workspace --all-targets -- -D warnings`
- [ ] `cargo test --workspace`
- [ ] Documentation and compatibility notes updated where needed
