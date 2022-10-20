use std::io::{BufReader, Read};

use actix_web::web::Bytes;
use actix_web::{post, App, HttpServer};
use actix_web::{web, Error, HttpResponse};
use serde::{Deserialize, Serialize};

use super::drivers;

#[derive(Serialize, Deserialize)]
struct PrintRequestQuery {
    driver: String,
}

#[post("/print")]
async fn print(
    bytes: Bytes,
    query: web::Query<PrintRequestQuery>,
    factory: web::Data<drivers::DriverFactory>
) -> Result<HttpResponse, Error> {
    let driver = factory.html_to_pdf(&query.driver[..]);
    match driver {
        None => Ok(HttpResponse::NotAcceptable().finish()),
        Some(driver) => {
            println!("Driver found, reading body");
            match String::from_utf8(bytes.to_vec()) {
                Ok(text) => {
                    println!("Body read! Converting to PDF");
                    let pdf = driver.html_to_pdf(&text[..]);
                    println!("Reading PDF into buffer");
                    let mut buf_reader = BufReader::new(pdf.expect("unable to read pdf"));
                    let mut tmp_vec: Vec<u8> = vec![];
                    buf_reader.read_to_end(&mut tmp_vec)?;
                    println!("Serving PDF");
                    Ok(
                        HttpResponse::Ok()
                            .append_header(("Content-Type", "application/pdf"))
                            .body(tmp_vec)
                    )
                },
                Err(_) => Ok(HttpResponse::BadRequest().finish())
            }
        }
    }
}

pub async fn start() -> std::io::Result<()> {

    let srv = HttpServer::new(move || {
        let factory = web::Data::new(drivers::DriverFactory::new());
        App::new()
            .service(print)
            .app_data(factory)
    })
        .bind(("127.0.0.1", 8888))?
        .workers(1)
        .run();

    println!("Printit started on http://0.0.0.0:8888");

    // run server until stopped (either by ctrl-c or stop endpoint)
    srv.await
}
