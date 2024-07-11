use std::{
    collections::BTreeMap,
    env, fmt::Debug,
};
use axum::{routing::{get, post}, Json, Router};

use convex::{ConvexClient, FunctionResult};

#[tokio::main]
async fn main() {
    async fn add_handler() {
        let deployment_url = get_deployment_url();
        let mut client = ConvexClient::new(&deployment_url).await.unwrap();
        let _result = client.mutation("exam:testAdd", BTreeMap::new()).await.unwrap();
    }
    let app = Router::new().route("/add",get(add_handler));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener,app).await.unwrap();
}

fn get_deployment_url() -> String{
    dotenvy::from_filename(".env.local").ok();
    dotenvy::dotenv().ok();

    let deployment_url = env::var("CONVEX_URL").unwrap();

    return deployment_url.clone();
}
