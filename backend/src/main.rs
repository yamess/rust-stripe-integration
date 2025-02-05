use crate::infra::web;

mod infra;
mod prelude;
mod domain;
mod handlers;
mod routes;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    web::run().await
}
