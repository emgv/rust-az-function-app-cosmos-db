# Rust Azure Function App Example
- Uses Azure SDK for Rust to implement an Http Trigger
- App Insights integration using a third party library axum-insights / opentelemetry_sdk
- Connects to Cosmos DB using DefaultAzureCredential locally or AppServiceManagedIdentityCredential on Azure
- Queries a Cosmos DB database

## Prerequisites
- Create an Azure Cosmos DB account / database / container with the partition key "/category"
- Insert some items in the container mentioned before and follow a document structure like the following:
```
{
  "id": "0001",
  "category": "gear-surf-surfboards",
  "name": "Surf board 9000",
  "quantity": 123,
  "price": 1900.84,
  "clearance": true
}
```
- To run locally, install Azure Functions Core Tools, then on powershell go to the project's root folder and type ```func start```, but before create a .env file at the project's root folder and set the following parameters:
  ```
  FUNCTIONS_CUSTOMHANDLER_PORT=<port-number>
  CFG__ENVIRONMENT=local
  CFG__COSMOS_AUTH_ENDPOINT="https://<cosmosdb-account>.documents.azure.com:443/"
  CFG__COSMOS_DBNAME=<cosmos-db-database-name>
  CFG__COSMOS_PRODUCTS_CONTAINER_NAME=<cosmos-db-container-name>
  CFG__APP_INSIGHTS_CONNECTION_STRING="<app-insights-connection-string>"
  CFG__APP_INSIGHTS_SERVICE_NAMESPACE=<app-insight-service-namespace>
  CFG__APP_INSIGHTS_SERVICE_NAME=<app-insight-service-name>
  CFG__APP_INSIGHTS_SERVICE_SERVER_NAME=<app-insight-server-name>
  ```
- To run on Azure you can setup the same parameters in the Funtion App's environtment variables, just remember to:
  - Change the parameter CFG__ENVIRONMENT to something different than "local"
  - Don't specify the parameter FUNCTIONS_CUSTOMHANDLER_PORT because it is meant to be used only locally
