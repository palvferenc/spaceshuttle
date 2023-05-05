use actix_web::web::Json;
use actix_web::{error, get, post, web, web::ServiceConfig, Result};
use rand::prelude::*;
use serde::Serialize;
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;
use sqlx::{Executor, FromRow, PgPool};

#[get("/")]
async fn attempts(state: web::Data<AppState>) -> Result<Json<Attempts>> {
    let attempts = 
    sqlx::query_as("SELECT * FROM (SELECT COUNT(successfull) as successfulls FROM attempt WHERE successfull = TRUE) AS successfulls, (SELECT COUNT(successfull) as faileds FROM attempt WHERE successfull = FALSE) AS faileds")
    .fetch_one(&state.pool)
    .await
    .map_err(|e| error::ErrorBadRequest(e.to_string()))?;

    Ok(Json(attempts))
}

#[post("/")]
async fn attempt(state: web::Data<AppState>) -> Result<Json<Attempt>> {
    let mut attempt = Attempt {
        id: 0,
        successfull: random(),
    };
    attempt =
        sqlx::query_as("INSERT INTO attempt(successfull) VALUES $1 Returning id, successfull")
            .bind(&attempt.successfull)
            .fetch_one(&state.pool)
            .await
            .map_err(|e| error::ErrorBadRequest(e.to_string()))?;

    Ok(Json(attempt))
}

#[derive(Clone)]
struct AppState {
    pool: PgPool,
}

#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_shared_db::Postgres] pool: PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    pool.execute(include_str!("../schema.sql"))
        .await
        .map_err(CustomError::new)?;

    let state = web::Data::new(AppState { pool });

    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(attempts).service(attempt).app_data(state);
    };

    Ok(config.into())
}

#[derive(Serialize, FromRow)]
struct Attempts {
    pub successfulls: i32,
    pub faileds: i32,
}

#[derive(Serialize, FromRow)]
struct Attempt {
    pub id: i32,
    pub successfull: bool,
}

