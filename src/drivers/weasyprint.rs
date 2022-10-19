pub struct WeasyPrint;
use super::HtmlToPdfWriter;

impl HtmlToPdfWriter for WeasyPrint {
  fn html_to_pdf(&self, _html: &str) {
    println!("Building html with Weasyprint");
  }
}

