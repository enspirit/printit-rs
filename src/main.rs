mod api;

#[actix_web::main]
async fn main() {
    api::start().await.expect("Unable to start printit's api");
}
