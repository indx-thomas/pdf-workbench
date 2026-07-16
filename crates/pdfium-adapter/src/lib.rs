//! Boundary for rendering and interactive document operations implemented with PDFium.

#![forbid(unsafe_code)]

use pdf_workbench_core::{BackendDescriptor, Capability};

/// Official PDFium source repository.
pub const UPSTREAM_REPOSITORY: &str = "https://pdfium.googlesource.com/pdfium";

/// Required build flag: PDF JavaScript execution is intentionally excluded.
pub const ENABLE_V8: bool = false;

/// Required build flag: dynamic XFA is intentionally excluded.
pub const ENABLE_XFA: bool = false;

const CAPABILITIES: &[Capability] = &[
    Capability::Render,
    Capability::TextExtraction,
    Capability::Search,
    Capability::FormFill,
    Capability::Annotations,
    Capability::SignatureInspection,
];

/// Returns PDFium's deliberately interactive responsibility in PDF Workbench.
#[must_use]
pub const fn descriptor() -> BackendDescriptor {
    BackendDescriptor {
        name: "PDFium",
        purpose: "rendering, navigation, text, forms, annotations, and inspection",
        capabilities: CAPABILITIES,
    }
}

#[cfg(test)]
mod tests {
    use super::{descriptor, ENABLE_V8, ENABLE_XFA};
    use pdf_workbench_core::Capability;

    #[test]
    fn active_content_is_disabled_by_contract() {
        assert!(!ENABLE_V8);
        assert!(!ENABLE_XFA);
    }

    #[test]
    fn pdfium_owns_interactive_capabilities() {
        let backend = descriptor();

        assert!(backend.supports(Capability::Render));
        assert!(backend.supports(Capability::FormFill));
        assert!(!backend.supports(Capability::Encryption));
    }
}
