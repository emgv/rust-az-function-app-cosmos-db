use std::collections::HashMap;

#[derive(Default, serde::Serialize, serde::Deserialize, Clone)]
pub struct WebError {
    message: String,
}

impl axum_insights::AppInsightsError for WebError {
    fn message(&self) -> Option<String> {
        Some(self.message.clone())
    }

    fn backtrace(&self) -> Option<String> {
        None
    }
}

pub fn build_layer(config: &super::app_config::Config) -> Result<axum_insights::AppInsightsLayer<WebError, WebError>, Box<dyn std::error::Error>> {
    Ok(
        axum_insights::AppInsights::default()
            // Accepts an optional connection string.  If None, then no telemetry is sent.
            .with_connection_string(config.app_insights_connection_string.clone())
            // Sets the service namespace and name.  Default is empty.
            .with_service_config(config.app_insights_service_namespace.clone(),
                                 config.app_insights_service_name.clone(),
                                 config.app_insights_service_server_name.clone())
            // Sets the HTTP client to use for sending telemetry.  Default is reqwest async client.
            .with_client(reqwest::Client::new())
            // Sets whether or not live metrics are collected.  Default is false.
            .with_live_metrics(true)
            // Sets the sample rate for telemetry.  Default is 1.0.
            .with_sample_rate(1.0)
            // Sets the minimum level for telemetry.  Default is INFO.
            .with_minimum_level(tracing::metadata::LevelFilter::INFO)
            // Sets the subscriber to use for telemetry.  Default is a new subscriber.
            .with_subscriber(tracing_subscriber::registry())
            // Sets the runtime to use for telemetry.  Default is Tokio.
            .with_runtime(opentelemetry_sdk::runtime::Tokio)
            // Sets whether or not to catch panics, and emit a trace for them.  Default is false.
            .with_catch_panic(true)
            // Sets whether or not to make this telemetry layer a noop.  Default is false.
            .with_noop(true)
            // Sets a function to extract extra fields from the request.  Default is no extra fields.
            .with_field_mapper(|parts| {
                let mut map = HashMap::new();
                map.insert("extra_field".to_owned(), "extra_value".to_owned());
                map
            })
            // Sets a function to extract extra fields from a panic.  Default is a default error.
            .with_panic_mapper(|panic| {
                (500, WebError { message: panic })
            })
            // Sets a function to determine the success-iness of a status.  Default is (100 - 399 => true).
            .with_success_filter(|status| {
                status.is_success() || status.is_redirection() || status.is_informational() || status == reqwest::StatusCode::NOT_FOUND
            })
            // Sets the common error type for the application, and will automatically extract information from handlers that return that error.
            .with_error_type::<WebError>()
            .build_and_set_global_default()
            .map_err(|err| format!("Error while initializing appinsights integration {:?}", err))?
            .layer())
}
