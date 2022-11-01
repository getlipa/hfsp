
pub fn validate(string: String) -> Option<String> {
    Some("invalid checksum".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_empty() {
	assert_eq!(validate("".to_string()).unwrap(), "invalid");
    }
}
