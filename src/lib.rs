use std::ops::Add;
use bitcoin::Script;
use std::str::FromStr;
use bitcoin::util::address::Address as InternalAddress;
use bitcoin::util::address::AddressType;
use bitcoin::network::constants::Network as InternalNetwork;

const ALL_PARSERS: &'static [&'static dyn CanParse] = &[&Address::PARSER];

trait DoParse<T> {

    fn parse(&self, string: &String) -> Result<T, String>;

}

trait CanParse {

    fn name(&self) -> String;

    fn can_parse(&self, string: &String) -> bool;
}

trait Parser<T> : DoParse<T> + CanParse {}

struct Address {
    address_type: AddressType,
    network: Network,
    raw: String
}

impl Address {

    const PARSER: AddressParser = AddressParser{};

    fn wrap(address: InternalAddress) -> Address{
        Address {
            address_type: address.address_type().unwrap(),
            network: Network::wrap(address.network),
            raw: address.to_string()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Network {
    Mainnet,
    Testnet,
    // todo
    Unknown,
}

impl Network {
    fn wrap(network: InternalNetwork) -> Network{
        match network {
            InternalNetwork::Bitcoin => Network::Mainnet,
            InternalNetwork::Testnet => Network::Testnet,
            _ => Network::Unknown
        }
    }
}

struct AddressParser {}

impl DoParse<Address> for AddressParser {

    fn parse(&self, string: &String) -> Result<Address, String> {
        let address: InternalAddress = string.parse().unwrap();
        Ok(Address::wrap(address))
    }
}

impl CanParse for AddressParser {

    fn name(&self) -> String {
        "on-chain-address".to_string()
    }
    
    fn can_parse(&self, string: &String) -> bool {
        match self.parse(&string) {
            Ok(_) => true,
            _ => false
        }
    }
}

impl Parser<Address> for AddressParser {

}

pub fn parse<T>(data: &String, parser: &dyn Parser<T>) -> Result<T, String> {
    if parser.can_parse(data) {
        return parser.parse(data);
    }

    for otherParser in ALL_PARSERS {
        if otherParser.can_parse(data) {
            return Err(format!("expected '{0}' but got '{1}'", parser.name(), otherParser.name())); // todo: translations? data structure?
        }
    }
    Err(format!("expected '{0}' but got unknown data", parser.name())) // todo
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn address_parser() {
        let address = "3P14159f73E4gFr7JterCCQh9QjiTjiZrG";

        let parsed = Address::PARSER.parse(&address.to_string()).unwrap();

        assert_eq!(parsed.raw, address);
        assert_eq!(parsed.address_type, AddressType::P2sh);
        assert_eq!(parsed.network, Network::Mainnet);
    }

    #[test]
    fn parser() {
        let address = "3P14159f73E4gFr7JterCCQh9QjiTjiZrG";

        let parsed = parse(&address.to_string(), &Address::PARSER).unwrap();

        assert_eq!(parsed.raw, address);
        assert_eq!(parsed.address_type, AddressType::P2sh);
        assert_eq!(parsed.network, Network::Mainnet);
    }
}