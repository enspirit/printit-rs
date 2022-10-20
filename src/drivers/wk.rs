use wkhtmltopdf::*;
use super::HtmlToPdfWriter;

pub struct WkHtmlToPdf;

impl HtmlToPdfWriter for WkHtmlToPdf {
  fn html_to_pdf(&self, html: &str) {

    let pdf_app = PdfApplication::new().expect("Failed to init PDF application");
    let mut pdfout = pdf_app.builder()
      .build_from_html(html).expect("could not generate pdf");

    pdfout.save("/tmp/test.pdf").expect("could not save pdf");

    println!("Building html with WK");
  }
}

