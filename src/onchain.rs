use bitcoin::Address;
use std::str::FromStr;

#[allow(dead_code)]
pub fn validate(string: String) -> Option<String> {
    Address::from_str(&string)
        .err()
        .map(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_empty() {
        assert_eq!(
            validate("".to_string()).unwrap(),
            "base58 address encoding error"
        );
    }

    #[test]
    fn validate_missing_char() {
        assert_eq!(
            validate("bc1p5cyxnuxmeuwuvkwfem96lqzszd02n6xdcjrs20cac6yqjjwudpxqkedrc".to_string()).unwrap(),
            "bech32 address encoding error"
        );
    }

    #[test]
    fn validate_p2pkh() {
        assert!(
            validate("15e15hWo6CShMgbAfo8c2Ykj4C6BLq6Not".to_string()).is_none()
        );
    }

    #[test]
    fn validate_p2sh() {
        assert!(
            validate("35PBEaofpUeH8VnnNSorM1QZsadrZoQp4N".to_string()).is_none()
        );
    }

    #[test]
    fn validate_p2wpkh() {
        assert!(
            validate("bc1q42lja79elem0anu8q8s3h2n687re9jax556pcc".to_string()).is_none()
        );
    }

    #[test]
    fn validate_p2wsh() {
        assert!(
            validate("bc1qrp33g0q5c5txsp9arysrx4k6zdkfs4nce4xj0gdcccefvpysxf3qccfmv3".to_string()).is_none()
        );
    }

    #[test]
    fn validate_p2tr() {
        assert!(
            validate("bc1p5cyxnuxmeuwuvkwfem96lqzszd02n6xdcjrs20cac6yqjjwudpxqkedrcr".to_string()).is_none()
        );
    }
}