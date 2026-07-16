//! Native qpdf integration checks, opt-in because they require a built executable.

use pdf_workbench_qpdf::{Qpdf, QpdfCheckOutcome};
use std::env;

#[test]
#[ignore = "requires PDF_WORKBENCH_QPDF_BIN and PDF_WORKBENCH_QPDF_TEST_PDF"]
fn checks_a_known_pdf_with_the_native_executable() {
    let executable = env::var_os("PDF_WORKBENCH_QPDF_BIN")
        .expect("PDF_WORKBENCH_QPDF_BIN must identify the qpdf executable");
    let input = env::var_os("PDF_WORKBENCH_QPDF_TEST_PDF")
        .expect("PDF_WORKBENCH_QPDF_TEST_PDF must identify a sanitized fixture");
    let qpdf = Qpdf::new(executable).expect("the configured qpdf executable must be valid");

    let report = qpdf
        .check(input)
        .expect("qpdf must accept the known-good fixture");

    assert_eq!(report.outcome(), QpdfCheckOutcome::Clean);
    assert!(report.stderr().is_empty());
}
