use wkhtmltopdf;
use super::HtmlToPdfWriter;

pub struct WkHtmlToPdf;

impl HtmlToPdfWriter for WkHtmlToPdf {
  fn html_to_pdf(&self, _html: &str) {
    println!("Building html with WK");
  }
}

