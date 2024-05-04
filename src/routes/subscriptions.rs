use actix_web::{web, HttpResponse};
use chrono::Utc;
use serde;
use sqlx;
use sqlx::PgPool;
use tracing;
use tracing::Instrument;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: web::Form<FormData>, db_pool: web::Data<PgPool>) -> HttpResponse {
    let request_id = Uuid::new_v4();
    let tracing_span = tracing::info_span!(
        "{} Saving new user {} {} to database.",
        %request_id,
        subscriver_name = form.name,
        subscriber_email = form.email
    );
    let _request_span_guard = tracing_span.enter();
    let query_span = tracing::info_span!("Saving new subscriber in database.");
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
    match result.instrument(query_span).await {
        Ok(_) => {
            tracing::info!("{} Added new user to database. =D", request_id);
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!("{} Failed to execute query: {:?}", request_id, e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
