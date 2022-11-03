use reqwest::Url;
use crate::network::Network;

struct Scheme;
impl Scheme {
    const HTTP: & 'static str = "http";
    const HTTPS: & 'static str = "https";
}

struct TopLevelDomain;
impl TopLevelDomain {
    const ONION: & 'static str = ".onion";
}

pub fn validate_ln_url(url: &str) -> bool {
    return url.contains("lnurl");
}

pub fn get_network_type(ln_url: &str) -> Network {
    let parsed_url = Url::parse(ln_url).unwrap();
    let scheme = parsed_url.scheme();
    let domain = parsed_url.domain().unwrap();
    let is_domain_onion = domain.ends_with(TopLevelDomain::ONION);

    if is_domain_onion {
        if scheme == Scheme::HTTP {
            return Network::Onion
        }
    } else if scheme == Scheme::HTTPS {
        return Network::Clearnet
    }

    return Network::Invalid
}

pub fn is_all_chars_same_case(url: &str) -> bool {
    if let Some(first_char) = url.chars().next() {
        let expected_format: bool = first_char.is_uppercase();
        return url.find(|c:char| c.is_uppercase() != expected_format && c.is_ascii_alphabetic()).is_none();
    }

    return false;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_chars() {
        assert!(!is_all_chars_same_case("aaB"));
        assert!(is_all_chars_same_case("ABCOOIKL991ÜŞSA"));
        assert!(is_all_chars_same_case("abcdefg1"));
        assert!(is_all_chars_same_case("BBB"));
    }

    #[test]
    fn test_urls() {
        let clear_net_uppercase = get_network_type("HTTPS://WWW.POLATALEMDAR.COM");
        let clear_net = get_network_type("https://www.memati.com");
        let invalid_onion = get_network_type("https://karabasan.onion");
        let onion = get_network_type("http://toper.onion");
        let onion_uppercase = get_network_type("HTTP://SABAME.ONION");
        let invalid_clear_net = get_network_type("http://www.gulluerhan.com");

        assert_eq!(clear_net_uppercase, Network::Clearnet);
        assert_eq!(clear_net, Network::Clearnet);
        assert_eq!(invalid_clear_net, Network::Invalid);
        assert_eq!(onion, Network::Onion);
        assert_eq!(onion_uppercase, Network::Onion);
        assert_eq!(invalid_onion, Network::Invalid);
    }
}