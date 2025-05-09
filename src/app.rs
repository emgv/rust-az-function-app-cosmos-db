use std::sync::Arc;

mod function_handler;
mod app_insights;
mod app_config;

pub async fn startup() -> Result<(), Box<dyn std::error::Error>> {

    let config = Arc::new(app_config::Config::new()?);
    let func_handler_port = config.func_handler_port;
    let app_insight_layer = app_insights::build_layer(&config)?;

    let app = axum::Router::new()
        .route("/api/rust-hello-world", axum::routing::get(function_handler::cosmos))
        .with_state(config)
        .layer(app_insight_layer);

    let app_address = &format!("{}:{}", std::net::Ipv4Addr::LOCALHOST, func_handler_port);
    let listener = tokio::net::TcpListener::bind(app_address)
        .await?;

    tracing::info!("Listening on {}", app_address);
    axum::serve(listener, app).await?;
    Ok(())
}