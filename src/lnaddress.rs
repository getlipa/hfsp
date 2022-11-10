use email_address_parser::EmailAddress;
use regex::Regex;

#[allow(dead_code)]
pub fn validate(string: &str) -> Option<String> {
    let result = EmailAddress::parse(string, None);
    if let Some(address) = result {
        let re = Regex::new(r"^[a-z0-9_.-]+$").unwrap();
        if re.is_match(address.get_local_part()) {
            let url = construct_url(address.get_local_part(), address.get_domain());
            println!("GET {}", url);
            return None;
        }
        return Some("It is a valid email address, but not lightning address".to_string());
    }

    Some("Invalid lightning address".to_string())
}

#[allow(dead_code)]
fn construct_url(username: &str, domain: &str) -> String {
    let protocol = if domain.ends_with(".onion") {
        "http"
    } else {
        "https"
    };
    format!("{}://{}/.well-known/lnurlp/{}", protocol, domain, username)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_email() {
        assert_eq!(validate("andrei@example"), None);
        assert_eq!(validate("andrei@example.com"), None);
        assert_eq!(validate("andrei.s_a@example.onion"), None);

        assert!(validate("andrei&@example").is_some());
        assert!(validate("Andrei+something@example").is_some());
        assert!(validate("andrei+something@example").is_some());
        assert!(validate("andrei-@example").is_some());
        assert!(validate("andrei@e df").is_some());
    }
}
