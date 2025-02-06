use crate::infra::web;

mod infra;
mod prelude;
mod domain;
mod application;
mod presentation;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    web::run().await
}
