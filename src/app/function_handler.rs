use std::sync::Arc;
use super::app_config::Config as AppConfig;
use axum::extract::State as AxumState;
use axum::response::{Html, Json, IntoResponse};
use axum_insights::exports::opentelemetry::global::ObjectSafeSpan;
use azure_core::credentials::TokenCredential as AzCoreTokenCredential;
use azure_identity::{DefaultAzureCredential, AppServiceManagedIdentityCredential, TokenCredentialOptions};
use futures::{Stream, StreamExt};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Item {
    pub id: String,
    pub category: String,
    pub name: String,
    pub quantity: i32,
    pub price: f64,
    pub clearance: bool,
}

pub async fn cosmos(AxumState(config): AxumState<Arc<AppConfig>>) -> axum::response::Response<axum::body::Body> {

    return match read_cosmos(&config).await {
        Err(e) => {
            tracing::error!("{:?}", e);

            return
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    [(axum::http::header::CONTENT_TYPE, "text/html")],
                     "<h6>An error occurred while getting the products<h6>",
                ).into_response();
        },
        Ok(items) => items.into_response(),
    };
}

async fn read_cosmos(app_config: &AppConfig) -> Result<Json<Vec<Item>>, Box<dyn std::error::Error>> {

    let credential: Arc<dyn AzCoreTokenCredential> = match app_config.is_local_env {
        true => {
            tracing::info!("Using DefaultAzureCredential for local environment");
            DefaultAzureCredential::new()?
        },
        _ => {
            tracing::info!("Using AppServiceManagedIdentityCredential for az cloud environment");
            AppServiceManagedIdentityCredential::new(TokenCredentialOptions::default())?
        }
    };

    let client = azure_data_cosmos::CosmosClient::new(app_config.cosmos_auth_endpoint.as_str(), credential, None)?;
    let database = client.database_client(app_config.cosmos_dbname.as_str());
    let container = database.container_client(app_config.cosmos_products_container_name.as_str());
    let item_partition_key = "gear-surf-surfboards";

    tracing::info!("Querying the products for partition key: {}...", item_partition_key);
    let query = azure_data_cosmos::Query::from("SELECT * FROM c WHERE c.category = @category")
        .with_parameter("@category", item_partition_key)?;

    let mut pager = container.query_items::<Item>(
        query,
        item_partition_key,
        None)?;

    let mut products: Vec<Item> = vec![];

    while let Some(page_response) = pager.next().await {

        let page = page_response?.into_body().await?;

        for item in page.items {
            tracing::info!("Retrieved product with name: {}", item.name);
            products.push(item);
        }
    }

    tracing::info!("Got #{} products", products.len());
    Ok(Json(products))
}