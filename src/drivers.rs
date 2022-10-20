use std::io::Read;
mod wk;
mod weasyprint;

pub trait HtmlToPdfWriter {
  fn html_to_pdf(&self, html: &str) -> Result<Box<dyn Read>, std::io::Error>;
}

pub struct DriverFactory;
impl DriverFactory {
    pub fn html_to_pdf(s: &str) -> Option<Box<dyn HtmlToPdfWriter>> {
        match s {
            "wk" => Some(Box::new(wk::WkHtmlToPdf {})),
            "weasyprint" => Some(Box::new(weasyprint::WeasyPrint {})),
            _ => None
        }
    }
}
