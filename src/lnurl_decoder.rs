use bech32::{self, FromBase32};
use std::{str};
use reqwest::{blocking};
use crate::lnurl_validator::{get_network_type, is_all_chars_same_case, is_ln_url};
use crate::network::Network;


fn decode(input: &str) -> String {
    if is_ln_url(input) {
        if is_all_chars_same_case(input) {
            let (_, data, _) = bech32::decode(input).unwrap();
            let data_u8 = Vec::<u8>::from_base32(&data);
            let url = String::from_utf8(data_u8.unwrap()).unwrap();

            if get_network_type(&url) != Network::Invalid {
                // TODO
                // If the query paramerts have "tag" then the url should be ınterpreded as a callback url
                // else GET url and JSON expected
                let json = fetch_json(&url);

                return json;
            }

            return "Network is invalid".to_string();
        }

        return "URL Characters".to_string();
    }

    return "Input is not LNURL".to_string();
}

fn fetch_json(url: &str) -> String {
    // TODO Make sure ignore http code.
    println!("URL: {}", url);
    let response = blocking::get(url);
    return response.unwrap().text().unwrap();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn validate_decoded_input() {
	    let decoded_string = decode("LNURL1DP68GURN8GHJ7AMPD3KX2AR0VEEKZAR0WD5XJTNRDAKJ7TNHV4KXCTTTDEHHWM30D3H82UNVWQHHQATZD35KXUNPV3SHYWPJXNCSL0");
        assert_eq!("https://walletofsatoshi.com/.well-known/lnurlp/publicradar82", decoded_string);
    }
}