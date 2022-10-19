pub trait HtmlToPdfWriter {
  fn build(&self, html: &str);
}

enum HtmlToPdfWriterDrivers {
  WkHtmlToPdf,
  WeasyPrint
}

pub struct WkHtmlToPdf;

impl HtmlToPdfWriter for WkHtmlToPdf {
  fn build(&self, html: &str) {
    println!("Building html with WK");
  }
}

pub struct WeasyPrint;

impl HtmlToPdfWriter for WeasyPrint {
  fn build(&self, html: &str) {
    println!("Building html with WK");
  }
}

pub struct DriverFactory;
impl DriverFactory {
    pub fn html_to_pdf(s: &str) -> Option<Box<dyn HtmlToPdfWriter>> {
        match s {
            "wk" => Some(Box::new(WkHtmlToPdf {})),
            "weasyprint" => Some(Box::new(WeasyPrint {})),
            _ => None
        }
    }
}
