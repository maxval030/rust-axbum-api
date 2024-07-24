use mongodb::{Client, Database};
use tracing::log::info;

pub async fn db_connect() -> mongodb::error::Result<Database> {
    let uri = "mongodb://root:password@0.0.0.0:27017";

    let client = Client::with_uri_str(uri).await?;

    let db = client.database("axum-rest-api");

    info!("Connected to MongoDB");

    Ok(db)
}
