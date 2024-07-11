use std::{
    collections::BTreeMap,
    env, fmt::Debug,
};
use axum::{routing::{get, post}, Json, Router};

use convex::{ConvexClient, ConvexError, FunctionResult, Value};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/exam/list", get(get_all_exam_handler));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener,app).await.unwrap();
}

fn get_deployment_url() -> String{
    dotenvy::from_filename(".env.local").ok();
    dotenvy::dotenv().ok();

    let deployment_url = env::var("CONVEX_URL").unwrap();

    return deployment_url.clone();
}

async fn get_client() -> ConvexClient{
    let deployment_url = get_deployment_url();
    let client = ConvexClient::new(&deployment_url).await.unwrap();
    return client
}

async fn root() -> &'static str {
    "Hello, World!"
}

/* async fn add_exam_handler() {
    let mut client = get_client().await;
    let _result = client.mutation("exam:testAdd", BTreeMap::new()).await.unwrap();
} */

async fn get_all_exam_handler() -> String{
    let mut client = get_client().await;
    let result = client.query("exam:getAllExams", BTreeMap::new())
        .await.unwrap();
    let string_value = match result {
        FunctionResult::Value(Value::Object(object)) => {
            let mut elements = String::new();
            for(key,value) in object {
                elements.push_str(&format!("{}: {:?}", key, value));
            }
            elements
        },
        FunctionResult::ErrorMessage(msg) => {
            return msg
        },
        FunctionResult::ConvexError(ConvexError { message: err, data }) => {
            return err
        },
        _ => todo!(),
    };
    return string_value
}

