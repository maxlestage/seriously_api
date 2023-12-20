use axum::{routing::get, Router};

use migration::{sea_orm, Migrator, MigratorTrait};
use sea_orm::Database;

#[tokio::main]
pub async fn main() {
    dotenvy::dotenv().ok();
    let connection = dotenvy::var("DATABASE_URL").expect("DATABASE_URL not set");

    let connection: sea_orm::prelude::DatabaseConnection =
        sea_orm::Database::connect(connection).await.unwrap();

    Migrator::up(&connection, None).await;

    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
