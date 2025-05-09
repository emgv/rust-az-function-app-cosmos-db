#![allow(unused)]

mod app;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt()
        .with_ansi(false)
        .with_target(false)
        .compact()
        .init();
    
    match app::startup().await {
        Ok(_) => (),
        Err(error) => tracing::error!("Error while starting up the web app {:?}", error),
    }
}
