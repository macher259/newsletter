use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct SubscriberName(String);

impl SubscriberName {
    const NAME_LENGTH_LIMIT: usize = 256;
    const FORBIDDEN_SYMBOLS: [char; 9] = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];

    pub fn parse(name: String) -> Result<Self, String> {
        let is_empty_or_whitespace = name.trim().is_empty();

        let is_too_long = name.graphemes(true).count() > Self::NAME_LENGTH_LIMIT;

        let contains_forbidden_symbols =
            is_too_long || name.chars().any(|c| Self::FORBIDDEN_SYMBOLS.contains(&c));

        if is_empty_or_whitespace || is_too_long || contains_forbidden_symbols {
            Err(format!("{} is not a valid subscriber name.", name))
        } else {
            Ok(Self(name))
        }
    }
}

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::SubscriberName;

    fn check_valid_name(name: &str) {
        let name = name.to_owned();
        assert!(SubscriberName::parse(name).is_ok());
    }

    fn check_invalid_name(name: &str) {
        let name = name.to_owned();
        assert!(SubscriberName::parse(name).is_err());
    }

    #[test]
    fn valid_name_is_accepted() {
        check_valid_name("Maciuś");
    }

    #[test]
    fn empty_name_is_rejected() {
        check_invalid_name("");
    }

    #[test]
    fn only_whitespace_is_rejected() {
        check_invalid_name("                ");
    }

    #[test]
    fn max_length_name_is_accepted() {
        check_valid_name(&"ą".repeat(SubscriberName::NAME_LENGTH_LIMIT));
    }

    #[test]
    fn longer_than_max_length_name_is_rejected() {
        check_invalid_name(&"ą".repeat(SubscriberName::NAME_LENGTH_LIMIT + 1));
    }

    #[test]
    fn names_containing_forbidden_symbols_are_rejected() {
        for name in SubscriberName::FORBIDDEN_SYMBOLS {
            check_invalid_name(&name.to_string());
        }
    }
}
