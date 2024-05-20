use validator::validate_email;
#[derive(Debug)]
pub struct SubscriberEmail(String);

impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
impl SubscriberEmail {
    pub fn parse(s: String) -> Result<SubscriberEmail, String> {
        match validate_email(&s) {
            true => return Ok(Self(s)),
            false => return Err(format!("bad email")),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::SubscriberEmail;
    use claims::assert_err;
    use fake::faker::internet::en::SafeEmail;
    use fake::Fake;
    #[test]
    fn empty_string_is_rejected() {
        let email = "".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }
    #[test]
    fn email_missing_at_symbol_is_rejected() {
        let email = "ursuladomain.com".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }
    #[test]
    fn email_missing_subject_is_rejected() {
        let email = "@domain.com".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }
    #[test]
    fn nice_email() {
        let email: String = SafeEmail().fake();

        let parsed_email = match SubscriberEmail::parse(email.clone()) {
            Ok(pe) => pe,
            Err(_) => panic!(),
        };
        assert_eq!(parsed_email.as_ref(), &email);
    }
}
