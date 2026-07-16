//! Headless entry point for architecture and native-engine integration tests.

use pdf_workbench_core::BackendDescriptor;
use pdf_workbench_qpdf::{Qpdf, QpdfCheckOutcome};
use std::{
    env,
    ffi::{OsStr, OsString},
    path::Path,
    process::ExitCode,
};

fn print_backend(backend: &BackendDescriptor) {
    println!("{}: {}", backend.name, backend.purpose);
    for capability in backend.capabilities {
        println!("  - {capability}");
    }
}

fn print_usage() {
    eprintln!("Usage:");
    eprintln!("  pdf-workbench-cli --capabilities");
    eprintln!("  pdf-workbench-cli check <file.pdf>");
    eprintln!();
    eprintln!(
        "The check command requires PDF_WORKBENCH_QPDF_BIN to contain an explicit qpdf path."
    );
}

fn print_qpdf_output(stdout: &str, stderr: &str) {
    if !stdout.is_empty() {
        print!("{stdout}");
    }
    if !stderr.is_empty() {
        eprint!("{stderr}");
    }
}

fn check_pdf(input: &OsStr) -> ExitCode {
    let Some(executable) = env::var_os("PDF_WORKBENCH_QPDF_BIN") else {
        eprintln!("PDF_WORKBENCH_QPDF_BIN is not set; provide the explicit qpdf executable path.");
        return ExitCode::from(2);
    };

    let qpdf = match Qpdf::new(executable) {
        Ok(qpdf) => qpdf,
        Err(error) => {
            eprintln!("error: {error}");
            return ExitCode::from(2);
        }
    };

    match qpdf.check(Path::new(input)) {
        Ok(report) => {
            print_qpdf_output(report.stdout(), report.stderr());
            match report.outcome() {
                QpdfCheckOutcome::Clean => ExitCode::SUCCESS,
                QpdfCheckOutcome::Warnings => ExitCode::from(3),
            }
        }
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::from(2)
        }
    }
}

fn main() -> ExitCode {
    let arguments = env::args_os().skip(1).collect::<Vec<OsString>>();

    match arguments.as_slice() {
        [argument] if argument == OsStr::new("--capabilities") => {
            print_backend(&pdf_workbench_qpdf::descriptor());
            print_backend(&pdf_workbench_pdfium::descriptor());
            ExitCode::SUCCESS
        }
        [command, input] if command == OsStr::new("check") => check_pdf(input),
        [] => {
            println!("PDF Workbench pre-alpha scaffold");
            print_usage();
            ExitCode::SUCCESS
        }
        _ => {
            print_usage();
            ExitCode::from(2)
        }
    }
}
