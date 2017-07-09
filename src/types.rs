use serde_json::Value;
use serde::Serialize;

#[derive(Deserialize, Debug)]
pub struct Input<T> {
    pub event: T,
    pub context: Context,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Context {
    #[serde(rename = "invokeid")]
    pub invoke_id: String,
    #[serde(rename = "awsRequestId")]
    pub aws_request_id: String,
    #[serde(rename = "functionName")]
    pub function_name: String,
    #[serde(rename = "functionVersion")]
    pub function_version: String,
    #[serde(rename = "logGroupName")]
    pub log_group_name: String,
    #[serde(rename = "logStreamName")]
    pub log_stream_name: Option<String>,
    #[serde(rename = "memoryLimitInMB")]
    pub memory_limit_in_mb: String,
    #[serde(rename = "isDefaultFunctionVersion")]
    pub is_default_function_version: Option<bool>,
    #[serde(rename = "clientContext")]
    pub client_context: Option<Value>,
    pub identity: Option<Identity>,
    #[serde(rename = "invokedFunctionARN")]
    pub invoked_function_arn: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Identity {
    #[serde(rename = "cognitoIdentityId")]
    pub cognito_identity_id: String,
    #[serde(rename = "cognitoIdentityIdPoolId")]
    pub cognito_identity_id_pool_id: String,
}

#[derive(Serialize, Debug)]
pub enum Output<T: Serialize> {
    #[serde(rename = "error")]
    Error(String),
    #[serde(rename = "value")]
    Value(T),
}
