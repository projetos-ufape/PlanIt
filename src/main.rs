mod config;
mod helpers;
mod modules;

use axum::{extract::Json, routing::get, Router};
use dotenv::dotenv;
use env_logger::Env;
use log::info;
use modules::{auth, category, goal, task, user};
use mongodb::Database;
use std::env;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use mongodb::bson::oid::ObjectId;

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

struct AppState {
    mongodb: Database,
    user_id: ObjectId,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let app_host: String = env::var("APP_HOST").unwrap_or_else(|_| String::from("127.0.0.1:8080"));
    let mongodb = config::mongodb::get_database().await;

    let user_id = ObjectId::new();

    let state = Arc::new(AppState { mongodb, user_id });
    let app = Router::new()
        .route(
            "/",
            get(|| async { Json(format!("PlanIt v{}", VERSION.unwrap_or("unknown"))) }),
        )
        .nest("/", auth::handles())
        .nest("/", user::handles())
        .nest("/", category::handles())
        .nest("/", goal::handles())
        .nest("/", task::handles())
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = TcpListener::bind(app_host)
        .await
        .expect("Unable to connect to the server");

    info!("Web Server running at {}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .await
        .expect("Error serving application");
}