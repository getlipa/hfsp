use bech32::{self, FromBase32};
use std::{str};

fn decode(input: &str) -> String {
    let (hrp, data) = bech32::decode(&input).unwrap();
    let base32_data = Vec::<u8>::from_base32(&data);
    return String::from_utf8(base32_data.unwrap()).unwrap();
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