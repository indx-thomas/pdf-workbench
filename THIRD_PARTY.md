# Third-party software

PDF Workbench is Apache-2.0 licensed. Third-party components remain under their respective licences.

## Included source dependency

### qpdf

- Project: <https://github.com/qpdf/qpdf>
- Project fork: <https://github.com/indx-thomas/pdf-workbench-qpdf>
- Pinned revision: `8ff6b5c4fca59e38b147aebddeb54341fc313ed1`
- Licence: Apache License 2.0; upstream also documents an Artistic License 2.0 option for historical compatibility
- Location: `vendor/qpdf` Git submodule

Redistributions that include qpdf must retain qpdf's licence, copyright, attribution, and `NOTICE.md` content as required by its upstream repository.

## Planned integrations

PDFium, LibreOffice, Tesseract, and veraPDF are architectural candidates but are not vendored or redistributed by this initial scaffold. Their exact versions, build options, licences, notices, and redistribution requirements must be recorded here before inclusion in a release.
