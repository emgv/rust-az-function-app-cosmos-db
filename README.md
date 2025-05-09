# Rust Azure Function App Example
- Uses Azure SDK for Rust to implement an Http Trigger
- App Insights integration using a third party library axum-insights / opentelemetry_sdk
- Connects to Cosmos DB using DefaultAzureCredential locally or AppServiceManagedIdentityCredential on Azure
- Queries a Cosmos DB database

## Prerequisites
- Create an Azure Cosmos DB account / database / container with partition key /category
- To run locally, create a .env file at the project root folder and setup the following parameters:
  - FUNCTIONS_CUSTOMHANDLER_PORT=\<port-number\>
  - CFG__ENVIRONMENT=local
  - CFG__COSMOS_AUTH_ENDPOINT="https://\<cosmosdb-account\>.documents.azure.com:443/"
  - CFG__COSMOS_DBNAME=\<cosmos-db-database-name\>
  - CFG__COSMOS_PRODUCTS_CONTAINER_NAME=\<cosmos-db-container-name\>
  - CFG__APP_INSIGHTS_CONNECTION_STRING="\<app-insights-connection-string\>"
  - CFG__APP_INSIGHTS_SERVICE_NAMESPACE=\<app-insight-service-namespace\>
  - CFG__APP_INSIGHTS_SERVICE_NAME=\<app-insight-service-name\>
  - CFG__APP_INSIGHTS_SERVICE_SERVER_NAME=\<app-insight-server-name\>
- To run on azure you can setup the same parameters in the Funtion App's environtment variables, just remember to change the parameter CFG__ENVIRONMENT to something different than "local"
