use std::io::Read;

use wkhtmltopdf::*;
use super::HtmlToPdfWriter;

pub struct WkHtmlToPdf;

impl HtmlToPdfWriter for WkHtmlToPdf {
  fn html_to_pdf(&self, html: &str) {

    // let pdf_app = PdfApplication::new().expect("Failed to init PDF application");
    // let mut pdfout = pdf_app.builder()
    //   .orientation(Orientation::Landscape)
    //   .margin(Size::Inches(2))
    //   .title("Awesome Foo")
    //   .build_from_html(html);

    println!("Building html with WK");
  }
}

