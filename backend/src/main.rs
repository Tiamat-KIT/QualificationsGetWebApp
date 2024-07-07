use std::{
    collections::BTreeMap,
    env,
};

use convex::ConvexClient;

#[tokio::main]
async fn main() {
    let deployment_url = get_deployment_url();
    let mut client = ConvexClient::new(&deployment_url).await.unwrap();
    let result = client.mutation("exam:testAdd", BTreeMap::new()).await.unwrap();
    println!("{result:#?}");
}

fn get_deployment_url() -> String{
    dotenvy::from_filename(".env.local").ok();
    dotenvy::dotenv().ok();

    let deployment_url = env::var("CONVEX_URL").unwrap();

    return deployment_url.clone();
}