use wkhtmltopdf;
use super::HtmlToPdfWriter;

pub struct WkHtmlToPdf;

impl HtmlToPdfWriter for WkHtmlToPdf {
  fn build(&self, _html: &str) {
    println!("Building html with WK");
  }
}

