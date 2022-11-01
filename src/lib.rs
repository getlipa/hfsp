#[derive(Debug)]
struct Address {
    address_type: AddressTypes,
    description: String,
    network: NetworkTypes,
}

impl Address {
    fn set_address_type(&mut self, addressType: AddressTypes) {
        self.address_type = addressType;
    }

    fn set_network_type(&mut self, network: NetworkTypes) {
        self.network = network;
    }
}

#[derive(Debug)]
enum AddressTypes {
    BOLT11,
    BOLT12,
    LNURL,
    TAPROOT,
}

#[derive(Debug)]
enum NetworkTypes {
    TESTNET,
    MAINNET,
    REGNET,
}

struct AddressPrefixes;
impl AddressPrefixes {
    const BOLT11: &str = "lnbc";
    const LNURL: &str = "lnurl";
    const BOLT12: &str = "lno";
    const BECH32ONCHAIN: &str = "bc1";
    const P2WPKHONCHAIN: &str = "bc1q";
    const TAPROOTP2TR: &str = "bc1p";
    const BIP21: &str = "bitcoin:";
}

fn generate_address(input: &str) -> Address {
    let mut address =  Address { address_type: AddressTypes::BOLT11, description: "".to_string(), network: NetworkTypes::TESTNET};
    let network = determine_network_type(input);
    address.set_network_type(network);

    if input.contains(&AddressPrefixes::BOLT11) {
        address.set_address_type(AddressTypes::BOLT11);
    } else if input.contains(&AddressPrefixes::BOLT12) {
        address.set_address_type(AddressTypes::BOLT12);
    } else if input.contains(&AddressPrefixes::LNURL) {
        address.set_address_type(AddressTypes::LNURL);
    }

    address
}

fn determine_network_type(input: &str) -> NetworkTypes {
    // TODO
    NetworkTypes::MAINNET
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let example: &str = "lnbc20m1pvjluezsp5zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zygshp58yjmdan79s6qqdhdzgynm4zwqd5d7xmw5fk98klysy043l2ahrqspp5qqqsyqcyq5rqwzqfqqqsyqcyq5rqwzqfqqqsyqcyq5rqwzqfqypqfp4qrp33g0q5c5txsp9arysrx4k6zdkfs4nce4xj0gdcccefvpysxf3q9qrsgq9vlvyj8cqvq6ggvpwd53jncp9nwc47xlrsnenq2zp70fq83qlgesn4u3uyf4tesfkkwwfg3qs54qe426hp3tz7z6sweqdjg05axsrjqp9yrrwc";
        let address = generate_address(example);
        println!("{:#?}", address);
    }
}