use std::io::Read;
use super::HtmlToPdfWriter;
pub struct WeasyPrint;

impl HtmlToPdfWriter for WeasyPrint {
  fn html_to_pdf(&self, _html: &str) -> Result<Box<dyn Read>, std::io::Error> {
    println!("Building html with Weasyprint");
    panic!("oops")
  }
}

