
#[derive(Clone)]
pub struct Config {
    pub env: String,
    pub is_local_env: bool,
    pub func_handler_port: u32,
    pub cosmos_auth_endpoint: String,
    pub cosmos_dbname: String,
    pub cosmos_products_container_name: String,
    pub app_insights_connection_string: String,
    pub app_insights_service_namespace: String,
    pub app_insights_service_name: String,
    pub app_insights_service_server_name: String,
}

impl Config {
    const LOCAL_ENV_NAME: &'static str = "LOCAL";

    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {

        let func_handler_port_number: u32 = std::env::var("FUNCTIONS_CUSTOMHANDLER_PORT")
            .map_err(|_| "Could not get var FUNCTIONS_CUSTOMHANDLER_PORT")?
            .parse()
            .map_err(|_| "Wrong FUNCTIONS_CUSTOMHANDLER_PORT, must be an integer")?;

        let env = std::env::var("CFG__ENVIRONMENT")
            .map_err(|_| "Could not get var CFG__ENVIRONMENT")?;

        let is_local_env = env.eq_ignore_ascii_case(Self::LOCAL_ENV_NAME);

        Ok(Self {
            env,
            is_local_env,
            func_handler_port: func_handler_port_number,
            cosmos_auth_endpoint: std::env::var("CFG__COSMOS_AUTH_ENDPOINT")
                .map_err(|_| "Could not get var CFG__COSMOS_AUTH_ENDPOINT")?,
            cosmos_dbname: std::env::var("CFG__COSMOS_DBNAME")
                .map_err(|_| "Could not get var CFG__COSMOS_DBNAME")?,
            cosmos_products_container_name: std::env::var("CFG__COSMOS_PRODUCTS_CONTAINER_NAME")
                .map_err(|_| "Could not get var CFG__COSMOS_PRODUCTS_CONTAINER_NAME")?,
            app_insights_connection_string: std::env::var("CFG__APP_INSIGHTS_CONNECTION_STRING")
                .map_err(|_| "Could not get var CFG__APP_INSIGHTS_CONNECTION_STRING")?,
            app_insights_service_namespace: std::env::var("CFG__APP_INSIGHTS_SERVICE_NAMESPACE")
                .map_err(|_| "Could not get var CFG__APP_INSIGHTS_SERVICE_NAMESPACE")?,
            app_insights_service_name: std::env::var("CFG__APP_INSIGHTS_SERVICE_NAME")
                .map_err(|_| "Could not get var CFG__APP_INSIGHTS_SERVICE_NAME")?,
            app_insights_service_server_name: std::env::var("CFG__APP_INSIGHTS_SERVICE_SERVER_NAME")
                .map_err(|_| "Could not get var CFG__APP_INSIGHTS_SERVICE_SERVER_NAME")?,
        })
    }
}


