mod wk;
mod weasyprint;

pub trait HtmlToPdfWriter {
  fn build(&self, html: &str);
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
