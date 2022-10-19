use actix_web::{dev::ServerHandle, post, App, HttpServer, Responder};
use actix_web::{get, web, Error, HttpResponse};
use futures::StreamExt;
use serde::{Deserialize, Serialize};

use super::drivers;

#[derive(Serialize, Deserialize)]
struct PrintRequestQuery {
    driver: String,
}

#[post("/print")]
async fn print(
    mut body: web::Payload,
    query: web::Query<PrintRequestQuery>,
) -> Result<HttpResponse, Error> {
    let driver = drivers::DriverFactory::html_to_pdf(&query.driver[..]);
    match driver {
        None => Ok(HttpResponse::NotAcceptable().finish()),
        Some(driver) => {
            let mut bytes = web::BytesMut::new();
            while let Some(item) = body.next().await {
                let item = item?;
                println!("Chunk: {:?}", &item);
                bytes.extend_from_slice(&item);
            }
            Ok(HttpResponse::Ok().finish())
        }
    }
}

pub async fn start() -> std::io::Result<()> {
    // start server as normal but don't .await after .run() yet
    let srv = HttpServer::new(|| App::new().service(print))
        .bind(("127.0.0.1", 8888))?
        .workers(2)
        .run();

    println!("Printit started on http://0.0.0.0:8888");

    // run server until stopped (either by ctrl-c or stop endpoint)
    srv.await
}
