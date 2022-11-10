#[allow(dead_code)]
pub fn validate(_string: String) -> Option<String> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_empty() {
        assert_eq!(validate("".to_string()).unwrap(), "invalid");
    }
}
