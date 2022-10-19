pub struct WeasyPrint;
use super::HtmlToPdfWriter;

impl HtmlToPdfWriter for WeasyPrint {
  fn build(&self, _html: &str) {
    println!("Building html with Weasyprint");
  }
}

