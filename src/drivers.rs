use std::io::Read;
mod wk;
mod weasyprint;

pub trait HtmlToPdfWriter {
  fn html_to_pdf(&self, html: &str) -> Result<Box<dyn Read>, std::io::Error>;
}

pub struct DriverFactory {
    wk: wk::WkHtmlToPdf,
    weasyprint: weasyprint::WeasyPrint
}

impl DriverFactory {
    pub fn new() -> Self {
        Self {
            wk: wk::WkHtmlToPdf::new(),
            weasyprint: weasyprint::WeasyPrint {}
        }
    }
    pub fn html_to_pdf(&self, s: &str) -> Option<Box<&dyn HtmlToPdfWriter>> {
        match s {
            "wk" => Some(Box::new(&self.wk)),
            "weasyprint" => Some(Box::new(&self.weasyprint)),
            _ => None
        }
    }
}
