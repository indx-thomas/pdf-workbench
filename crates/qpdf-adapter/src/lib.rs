//! Boundary for structural operations implemented through the pinned qpdf fork.

#![forbid(unsafe_code)]

use pdf_workbench_core::{BackendDescriptor, Capability};
use std::{
    error::Error,
    fmt, fs, io,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

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

/// A validated qpdf command-line executable.
///
/// The adapter currently invokes qpdf as a child process. This keeps the first
/// native integration narrow while the isolated worker protocol is designed.
#[derive(Clone, Debug)]
pub struct Qpdf {
    executable: PathBuf,
}

/// The non-error result of qpdf's structural check.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QpdfCheckOutcome {
    /// qpdf found no errors or warnings.
    Clean,
    /// qpdf found recoverable warnings but no errors.
    Warnings,
}

/// Captured output from qpdf's structural check.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QpdfCheckReport {
    outcome: QpdfCheckOutcome,
    stdout: String,
    stderr: String,
}

fn classify_check(
    exit_code: Option<i32>,
    stdout: String,
    stderr: String,
) -> Result<QpdfCheckReport, QpdfError> {
    let outcome = match exit_code {
        Some(0) => QpdfCheckOutcome::Clean,
        Some(3) => QpdfCheckOutcome::Warnings,
        exit_code => {
            return Err(QpdfError::CheckFailed {
                exit_code,
                stdout,
                stderr,
            });
        }
    };

    Ok(QpdfCheckReport {
        outcome,
        stdout,
        stderr,
    })
}

impl QpdfCheckReport {
    /// Returns whether qpdf found a clean file or recoverable warnings.
    #[must_use]
    pub const fn outcome(&self) -> QpdfCheckOutcome {
        self.outcome
    }

    /// Returns qpdf's standard output.
    #[must_use]
    pub fn stdout(&self) -> &str {
        &self.stdout
    }

    /// Returns qpdf's standard error.
    #[must_use]
    pub fn stderr(&self) -> &str {
        &self.stderr
    }
}

/// Failure to configure or run qpdf's structural check.
#[derive(Debug)]
pub enum QpdfError {
    /// The configured qpdf executable could not be resolved.
    ExecutableUnavailable {
        /// The path supplied by the caller.
        path: PathBuf,
        /// The underlying filesystem error.
        source: io::Error,
    },
    /// The configured qpdf path does not refer to a regular file.
    ExecutableNotFile {
        /// The resolved path supplied by the caller.
        path: PathBuf,
    },
    /// The PDF input could not be resolved.
    InputUnavailable {
        /// The path supplied by the caller.
        path: PathBuf,
        /// The underlying filesystem error.
        source: io::Error,
    },
    /// The PDF input path does not refer to a regular file.
    InputNotFile {
        /// The resolved input path.
        path: PathBuf,
    },
    /// The operating system could not start or wait for qpdf.
    InvocationFailed {
        /// The validated qpdf executable path.
        executable: PathBuf,
        /// The underlying process error.
        source: io::Error,
    },
    /// qpdf reported a structural error or terminated without an exit code.
    CheckFailed {
        /// qpdf's exit code, or `None` if the process was terminated by a signal.
        exit_code: Option<i32>,
        /// Captured standard output.
        stdout: String,
        /// Captured standard error.
        stderr: String,
    },
}

impl fmt::Display for QpdfError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExecutableUnavailable { path, source } => {
                write!(
                    formatter,
                    "qpdf executable is unavailable at {}: {source}",
                    path.display()
                )
            }
            Self::ExecutableNotFile { path } => {
                write!(
                    formatter,
                    "qpdf executable path is not a file: {}",
                    path.display()
                )
            }
            Self::InputUnavailable { path, source } => {
                write!(
                    formatter,
                    "PDF input is unavailable at {}: {source}",
                    path.display()
                )
            }
            Self::InputNotFile { path } => {
                write!(
                    formatter,
                    "PDF input path is not a file: {}",
                    path.display()
                )
            }
            Self::InvocationFailed { executable, source } => {
                write!(
                    formatter,
                    "failed to invoke qpdf at {}: {source}",
                    executable.display()
                )
            }
            Self::CheckFailed {
                exit_code, stderr, ..
            } => {
                let status = exit_code.map_or_else(
                    || "without an exit code".to_owned(),
                    |code| format!("with exit code {code}"),
                );
                let detail = stderr.trim();

                if detail.is_empty() {
                    write!(formatter, "qpdf structural check failed {status}")
                } else {
                    write!(formatter, "qpdf structural check failed {status}: {detail}")
                }
            }
        }
    }
}

impl Error for QpdfError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::ExecutableUnavailable { source, .. }
            | Self::InputUnavailable { source, .. }
            | Self::InvocationFailed { source, .. } => Some(source),
            Self::ExecutableNotFile { .. }
            | Self::InputNotFile { .. }
            | Self::CheckFailed { .. } => None,
        }
    }
}

impl Qpdf {
    /// Resolves and validates an explicit qpdf executable path.
    ///
    /// Requiring a file path avoids an implicit executable search through
    /// `PATH`, which is important once the adapter runs inside a restricted
    /// local worker.
    pub fn new(executable: impl AsRef<Path>) -> Result<Self, QpdfError> {
        let requested = executable.as_ref();
        let executable =
            fs::canonicalize(requested).map_err(|source| QpdfError::ExecutableUnavailable {
                path: requested.to_owned(),
                source,
            })?;

        if !executable.is_file() {
            return Err(QpdfError::ExecutableNotFile { path: executable });
        }

        Ok(Self { executable })
    }

    /// Returns the canonical path to the configured qpdf executable.
    #[must_use]
    pub fn executable(&self) -> &Path {
        &self.executable
    }

    /// Performs qpdf's read-only structural check on a PDF file.
    ///
    /// Exit code `0` is clean, and qpdf's documented warning-only exit code
    /// `3` is returned as [`QpdfCheckOutcome::Warnings`]. Other statuses are
    /// failures. This operation does not claim PDF conformance or content-level
    /// validity beyond qpdf's structural checks.
    pub fn check(&self, input: impl AsRef<Path>) -> Result<QpdfCheckReport, QpdfError> {
        let requested = input.as_ref();
        let input = fs::canonicalize(requested).map_err(|source| QpdfError::InputUnavailable {
            path: requested.to_owned(),
            source,
        })?;

        if !input.is_file() {
            return Err(QpdfError::InputNotFile { path: input });
        }

        let output = Command::new(&self.executable)
            .arg("--check")
            // `canonicalize` returns a verbatim `\\?\` path on Windows, which
            // qpdf 12.4 does not parse correctly as a command-line argument.
            // The canonical path is used for validation; the original OS
            // string preserves non-Unicode support without the verbatim prefix.
            .arg(requested)
            .stdin(Stdio::null())
            .output()
            .map_err(|source| QpdfError::InvocationFailed {
                executable: self.executable.clone(),
                source,
            })?;

        let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
        let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
        classify_check(output.status.code(), stdout, stderr)
    }
}

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
    use super::{Qpdf, QpdfCheckOutcome, QpdfError, classify_check, descriptor};
    use pdf_workbench_core::Capability;
    use std::{env, process};

    #[test]
    fn qpdf_is_not_treated_as_a_renderer() {
        let backend = descriptor();

        assert!(backend.supports(Capability::Merge));
        assert!(!backend.supports(Capability::Render));
        assert!(!backend.supports(Capability::TextExtraction));
    }

    #[test]
    fn rejects_an_unavailable_executable() {
        let path = env::temp_dir().join(format!(
            "pdf-workbench-{}-missing-qpdf-executable",
            process::id()
        ));

        let error = Qpdf::new(&path).expect_err("a missing executable must be rejected");

        assert!(matches!(
            error,
            QpdfError::ExecutableUnavailable { path: error_path, .. } if error_path == path
        ));
    }

    #[test]
    fn rejects_an_unavailable_input_before_invocation() {
        let executable = env::current_exe().expect("the test executable must be available");
        let qpdf = Qpdf::new(executable).expect("the test executable is a regular file");
        let path =
            env::temp_dir().join(format!("pdf-workbench-{}-missing-input.pdf", process::id()));

        let error = qpdf
            .check(&path)
            .expect_err("a missing input must be rejected");

        assert!(matches!(
            error,
            QpdfError::InputUnavailable { path: error_path, .. } if error_path == path
        ));
    }

    #[test]
    fn preserves_qpdf_warning_semantics() {
        let clean = classify_check(Some(0), "clean".to_owned(), String::new())
            .expect("exit code zero must be clean");
        let warning = classify_check(Some(3), String::new(), "warning".to_owned())
            .expect("exit code three must be recoverable");
        let failure = classify_check(Some(2), String::new(), "error".to_owned())
            .expect_err("exit code two must fail");

        assert_eq!(clean.outcome(), QpdfCheckOutcome::Clean);
        assert_eq!(warning.outcome(), QpdfCheckOutcome::Warnings);
        assert!(matches!(
            failure,
            QpdfError::CheckFailed {
                exit_code: Some(2),
                ..
            }
        ));
    }
}
