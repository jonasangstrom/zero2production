use crate::domain::SubscriberEmail;
use reqwest::Client;

pub struct EmailClient {
    sender: SubscriberEmail,
    base_url: String,
    http_client: Client,
}

impl EmailClient {
    pub fn new(sender: SubscriberEmail, base_url: String) -> Self {
        Self {
            sender,
            base_url,
            http_client: Client::new(),
        }
    }

    pub async fn send_email(
        &self,
        recipient: SubscriberEmail,
        subject: &str,
        html_content: &str,
        text_content: &str,
    ) -> Result<(), String> {
        todo!()
    }
}
