# Rust Azure Function App Example
- Uses Azure SDK for Rust to implement an Http Trigger
- App Insights integration using a third party library axum-insights / opentelemetry_sdk
- Connects to Cosmos DB using DefaultAzureCredential locally or AppServiceManagedIdentityCredential on Azure
- Queries a Cosmos DB database using the Azure SDK

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
- To run locally, install <ins>Azure Functions Core Tools</ins>, and then:
  1. Create a .env file at the project's root folder and add the following parameters:
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
  2. Build and run:
  ```
  cd <path-to-the-project-root-folder>
  cargo remove openssl
  rm handler handler.exe
  cargo build && cp target/debug/rs-function-app.exe handler.exe
  func start
  ```

- To deploy on Azure you can setup the same parameters in the Funtion App's environtment variables, just remember to:
  - Change the parameter CFG__ENVIRONMENT to something different than "local"
  - Don't specify the parameter FUNCTIONS_CUSTOMHANDLER_PORT because it is meant to be used only locally
  - To compile for a Linux Function App you can do it with wsl ubuntu:
    1. Install dependencies and build (wsl ubuntu shell):
    ```
    sudo apt install pkg-config libssl-dev musl-dev
    cd /mnt/c/path-to-the-project-root-folder
    cargo add openssl --features vendored
    TARGET_CC=x86_64-linux-musl-gcc cargo build --release --target x86_64-unknown-linux-musl
    rm handler handler.exe
    cp target/x86_64-unknown-linux-musl/release/rs-function-app handler
    ```
    2. The .funcignore file should look like this:
    ```
    .git*
    .vscode
    __azurite_db*__.json
    __blobstorage__
    __queuestorage__
    local.settings.json
    test
    target
    src
    Cargo.lock
    Cargo.toml
    .env
    .idea
    ```
    3. You can deploy using vscode, just follow the steps here https://learn.microsoft.com/en-us/azure/azure-functions/create-first-function-vs-code-other?tabs=rust%2Clinux#sign-in-to-azure
