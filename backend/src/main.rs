use crate::infra::web;

mod application;
mod domain;
mod infra;
mod prelude;
mod presentation;
mod schema;
mod shared;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    web::run().await
}
