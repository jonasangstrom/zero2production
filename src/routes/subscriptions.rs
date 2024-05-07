use actix_web::{web, HttpResponse};
use chrono::Utc;
use serde;
use sqlx;
use sqlx::PgPool;
use tracing;
use unicode_segmentation::UnicodeSegmentation;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

#[tracing::instrument(
name = "Adding a new subscriber",
skip(form, pool),
fields(
subscriber_email = %form.email,
subscriber_name = %form.name
)
)]
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    if is_valid_name(&form.name) != true {
        return HttpResponse::BadRequest().finish();
    }
    match insert_subscriber(&pool, &form).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(form, pool)
)]
pub async fn insert_subscriber(pool: &PgPool, form: &FormData) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}

pub fn is_valid_name(name: &str) -> bool {
    let not_empty_or_whitespace = name.trim().is_empty();
    if not_empty_or_whitespace {
        return false;
    }
    let name_too_long = name.graphemes(true).count() > 256;
    if name_too_long {
        return false;
    }
    let forbidden_characters = ['/', '{', '}', '\\', '(', ')', '<', '>', '*'];
    let name_contains_bad_chars = name.chars().any(|c| forbidden_characters.contains(&c));
    if name_contains_bad_chars {
        return false;
    }
    true
}
