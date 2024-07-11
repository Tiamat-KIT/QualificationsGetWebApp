use std::{
    collections::BTreeMap,
    env,
};
use axum::{routing::get,routing::post, Router};

use convex::ConvexClient;

#[tokio::main]
async fn main() {
    let deployment_url = get_deployment_url();
    let mut client = ConvexClient::new(&deployment_url).await.unwrap();
    async fn add_handler() -> String {
        let result = client.mutation("exam:testAdd", BTreeMap::new()).await.unwrap();
        return result
    }
    let app = Router::new().route("/add",get());
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000");
    axum::serve(listener,app).await.unwrap();
}

fn get_deployment_url() -> String{
    dotenvy::from_filename(".env.local").ok();
    dotenvy::dotenv().ok();

    let deployment_url = env::var("CONVEX_URL").unwrap();

    return deployment_url.clone();
}