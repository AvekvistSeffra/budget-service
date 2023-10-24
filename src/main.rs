use anyhow::Context;
use anyhow::Result;
use axum::{routing::get, Router};
use config::Config;
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod controllers;
mod database;
mod error;
mod models;

use controllers::{expenses, incomes, people, savings};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "budget_service_postgres=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config =
        Config::load("config.yaml").with_context(|| format!("failed to load configuration"))?;

    let db_connection_string = config.database.connection_string();

    let pool = PgPoolOptions::new()
        .max_connections(config.database.pool_size)
        .acquire_timeout(std::time::Duration::from_secs(config.database.timeout))
        .connect(&db_connection_string)
        .await
        .with_context(|| format!("failed to connect to database: {}", db_connection_string))?;

    let app = Router::new()
        .route("/expenses", get(expenses::get).post(expenses::post))
        .route(
            "/expenses/:id",
            get(expenses::get_one)
                .put(expenses::put)
                .delete(expenses::delete),
        )
        .route("/incomes", get(incomes::get).post(incomes::post))
        .route(
            "/incomes/:id",
            get(incomes::get_one)
                .put(incomes::put)
                .delete(incomes::delete),
        )
        .route("/people", get(people::get).post(people::post))
        .route(
            "/people/:id",
            get(people::get_one).put(people::put).delete(people::delete),
        )
        .route("/savings", get(savings::get))
        .route("/savings/:id", get(savings::get_one))
        .with_state(pool);

    let addr = SocketAddr::from((config.address.ip, config.address.port));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
