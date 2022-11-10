use lightning_invoice::Invoice;
use std::str::FromStr;

#[allow(dead_code)]
pub fn validate(string: String) -> Option<String> {
    Invoice::from_str(chomp_prefix(&string))
        .err()
        .map(|e| e.to_string())
}

fn chomp_prefix(string: &str) -> &str {
    let prefix = "lightning:";
    if let Some(string) = string.strip_prefix(prefix) {
        string
    } else {
        string
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_invoice() {
        assert_eq!(
            validate("".to_string()).unwrap(),
            "Invalid bech32: missing human-readable separator, \"1\""
        );
        let valid = "lnbc20m1pvjluezsp5zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zygshp58yjmdan79s6qqdhdzgynm4zwqd5d7xmw5fk98klysy043l2ahrqspp5qqqsyqcyq5rqwzqfqqqsyqcyq5rqwzqfqqqsyqcyq5rqwzqfqypqfp4qrp33g0q5c5txsp9arysrx4k6zdkfs4nce4xj0gdcccefvpysxf3q9qrsgq9vlvyj8cqvq6ggvpwd53jncp9nwc47xlrsnenq2zp70fq83qlgesn4u3uyf4tesfkkwwfg3qs54qe426hp3tz7z6sweqdjg05axsrjqp9yrrwc";
        assert!(validate(valid.to_string()).is_none());
        assert!(validate(format!("lightning:{}", valid)).is_none());
    }
}
