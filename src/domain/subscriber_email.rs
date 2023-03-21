#[derive(Debug)]
pub struct SubscriberEmail(String);

impl SubscriberEmail {
    pub fn parse(email: String) -> Result<Self, String> {
        if validator::validate_email(&email) {
            Ok(Self(email))
        } else {
            Err(format!("{} is not a valid subscriber email.", email))
        }
    }
}

impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for SubscriberEmail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::SubscriberEmail;

    fn check_incorrect_email(email: &str) {
        let email = email.to_owned();
        assert!(SubscriberEmail::parse(email).is_err());
    }

    #[test]
    fn empty_string_is_rejected() {
        check_incorrect_email("");
    }

    #[test]
    fn email_missing_at_symbol_is_rejected() {
        check_incorrect_email("johndoedomain.com");
    }

    #[test]
    fn email_missing_subject_is_rejected() {
        check_incorrect_email("@domain.com");
    }

    #[test]
    fn correct_mail_is_accepted() {
        let email = "john.doe@domain.com".to_owned();
        assert!(SubscriberEmail::parse(email).is_ok());
    }
}
