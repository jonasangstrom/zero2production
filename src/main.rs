use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::email_client::EmailClient;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read config. ;)");
    let sender_email = configuration
        .email_client
        .sender()
        .expect("Bad config for sender email.");
    let email_client = EmailClient::new(sender_email, configuration.email_client.base_url);
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("create pg pool failed.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool, email_client)?.await?;
    Ok(())
}
