use actix_web::{web, HttpResponse};
use chrono::Utc;
use log;
use serde;
use sqlx;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: web::Form<FormData>, db_pool: web::Data<PgPool>) -> HttpResponse {
    let request_id = Uuid::new_v4();
    log::info!(
        "{} Saving new user {} {} to database.",
        request_id,
        form.name,
        form.email
    );
    let result = sqlx::query!(
        r#"
    INSERT INTO subscriptions (id, email, name, subscribed_at)
    VALUES ($1, $2, $3, $4)
    "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(db_pool.get_ref());
    match result.await {
        Ok(_) => {
            log::info!("{} Added new user to database. =D", request_id);
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            log::error!("{} Failed to execute query: {:?}", request_id, e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
