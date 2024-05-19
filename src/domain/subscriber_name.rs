use unicode_segmentation::UnicodeSegmentation;
pub struct SubscriberName(String);

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
impl SubscriberName {
    pub fn parse(s: String) -> Result<SubscriberName, String> {
        let is_empty_or_whitespace = s.trim().is_empty();
        let name_too_long = s.graphemes(true).count() > 256;
        let forbidden_characters = ['/', '{', '}', '\\', '(', ')', '<', '>', '*'];
        let name_contains_bad_chars = s.chars().any(|c| forbidden_characters.contains(&c));
        if is_empty_or_whitespace || name_too_long || name_contains_bad_chars {
            Err(format!("{} is not a valid subscriber name.", s))
        } else {
            Ok(Self(s))
        }
    }
}
