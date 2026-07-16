//! Minimal headless entry point for architecture and integration smoke tests.

use pdf_workbench_core::BackendDescriptor;

fn print_backend(backend: &BackendDescriptor) {
    println!("{}: {}", backend.name, backend.purpose);
    for capability in backend.capabilities {
        println!("  - {capability}");
    }
}

fn main() {
    if std::env::args().any(|argument| argument == "--capabilities") {
        print_backend(&pdf_workbench_qpdf::descriptor());
        print_backend(&pdf_workbench_pdfium::descriptor());
        return;
    }

    println!("PDF Workbench pre-alpha scaffold");
    println!("Run with --capabilities to inspect the planned engine boundaries.");
}
