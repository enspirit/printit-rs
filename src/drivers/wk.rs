use std::io::Read;
use wkhtmltopdf::PdfApplication;
use super::HtmlToPdfWriter;

pub struct WkHtmlToPdf;

impl HtmlToPdfWriter for WkHtmlToPdf {
  fn html_to_pdf(&self, html: &str) -> Result<Box<dyn Read>, std::io::Error> {

    let pdf_app = PdfApplication::new().expect("Failed to init PDF application");
    let pdfout = pdf_app.builder()
      .build_from_html(html).expect("could not generate pdf");

    // pdfout.save("/tmp/test.pdf").expect("could not save pdf");

    println!("Building html with WK");
    Ok(Box::new(pdfout))
  }
}

