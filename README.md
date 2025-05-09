# Rust Azure Function App Example
- Uses Azure SDK for Rust to implement an Http Trigger
- App Insights integration using a third party library axum-insights / opentelemetry_sdk
- Connects to Cosmos DB using DefaultAzureCredential locally or AppServiceManagedIdentityCredential on Azure
- Queries a Cosmos DB database
