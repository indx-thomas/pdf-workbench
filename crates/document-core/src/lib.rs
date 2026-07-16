//! Engine-neutral contracts shared by the desktop, CLI, and future local service.

#![forbid(unsafe_code)]

use std::fmt::{Display, Formatter};

/// A capability assigned to one document backend by the architecture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Capability {
    /// Render PDF pages into displayable output.
    Render,
    /// Extract text for selection, indexing, and accessibility.
    TextExtraction,
    /// Search document text.
    Search,
    /// Read and fill AcroForm fields.
    FormFill,
    /// Read and write review annotations.
    Annotations,
    /// Inspect embedded PDF signature information.
    SignatureInspection,
    /// Combine pages or documents.
    Merge,
    /// Extract or divide pages into documents.
    Split,
    /// Apply supported PDF encryption settings.
    Encryption,
    /// Produce a linearized PDF for progressive access.
    Linearization,
    /// Normalize or structurally rewrite a PDF.
    Normalization,
}

impl Display for Capability {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Render => "render",
            Self::TextExtraction => "text extraction",
            Self::Search => "search",
            Self::FormFill => "AcroForm fill",
            Self::Annotations => "annotations",
            Self::SignatureInspection => "signature inspection",
            Self::Merge => "merge",
            Self::Split => "split",
            Self::Encryption => "encryption",
            Self::Linearization => "linearization",
            Self::Normalization => "normalization",
        };

        formatter.write_str(value)
    }
}

/// Describes the deliberately limited responsibility of one backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BackendDescriptor {
    /// Human-readable backend name.
    pub name: &'static str,
    /// Short statement of the backend's architectural role.
    pub purpose: &'static str,
    /// Capabilities assigned to the backend.
    pub capabilities: &'static [Capability],
}

impl BackendDescriptor {
    /// Returns whether this backend is assigned a capability.
    #[must_use]
    pub fn supports(&self, capability: Capability) -> bool {
        self.capabilities.contains(&capability)
    }
}

#[cfg(test)]
mod tests {
    use super::{BackendDescriptor, Capability};

    #[test]
    fn descriptor_reports_only_declared_capabilities() {
        let backend = BackendDescriptor {
            name: "test",
            purpose: "test backend",
            capabilities: &[Capability::Render],
        };

        assert!(backend.supports(Capability::Render));
        assert!(!backend.supports(Capability::Merge));
    }
}
