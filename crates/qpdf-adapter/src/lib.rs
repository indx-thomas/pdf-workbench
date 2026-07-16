//! Boundary for structural operations implemented through the pinned qpdf fork.

#![forbid(unsafe_code)]

use pdf_workbench_core::{BackendDescriptor, Capability};

/// Upstream qpdf source repository.
pub const UPSTREAM_REPOSITORY: &str = "https://github.com/qpdf/qpdf";

/// PDF Workbench's clean qpdf fork.
pub const FORK_REPOSITORY: &str = "https://github.com/indx-thomas/pdf-workbench-qpdf";

/// qpdf revision pinned by the `vendor/qpdf` submodule in this scaffold.
pub const PINNED_REVISION: &str = "8ff6b5c4fca59e38b147aebddeb54341fc313ed1";

const CAPABILITIES: &[Capability] = &[
    Capability::Merge,
    Capability::Split,
    Capability::Encryption,
    Capability::Linearization,
    Capability::Normalization,
];

/// Returns qpdf's deliberately narrow responsibility in PDF Workbench.
#[must_use]
pub const fn descriptor() -> BackendDescriptor {
    BackendDescriptor {
        name: "qpdf",
        purpose: "content-preserving structural PDF transformations",
        capabilities: CAPABILITIES,
    }
}

#[cfg(test)]
mod tests {
    use super::descriptor;
    use pdf_workbench_core::Capability;

    #[test]
    fn qpdf_is_not_treated_as_a_renderer() {
        let backend = descriptor();

        assert!(backend.supports(Capability::Merge));
        assert!(!backend.supports(Capability::Render));
        assert!(!backend.supports(Capability::TextExtraction));
    }
}
