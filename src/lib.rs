use bitcoin::network::constants::Network;
use bitcoin::util::address::Address as InternalAddress;
use bitcoin::util::address::AddressType;

const ALL_PARSERS: &'static [&'static dyn CanParse] = &[&Address::PARSER];

pub trait DoParse<T> {
    fn parse(&self, string: &String) -> Result<T, String>;
}

pub trait CanParse {
    fn name(&self) -> String;

    fn can_parse(&self, string: &String) -> bool;
}

pub trait Parser<T>: DoParse<T> + CanParse {}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
struct Address {
    address_type: AddressType,
    network: Network,
    raw: String,
}

impl Address {
    const PARSER: AddressParser = AddressParser {};

    fn wrap(address: InternalAddress) -> Address {
        Address {
            address_type: address.address_type().unwrap(),
            network: address.network,
            raw: address.to_string(),
        }
    }
}

#[derive(Copy, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
struct AddressParser {}

impl DoParse<Address> for AddressParser {
    fn parse(&self, string: &String) -> Result<Address, String> {
        match string.parse() {
            Ok(address) => Ok(Address::wrap(address)),
            Err(error) => Err(error.to_string())
        }
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

impl Parser<Address> for AddressParser {}

pub fn parse<T>(data: &String, parser: &dyn Parser<T>) -> Result<T, String> {
    if parser.can_parse(data) {
        return parser.parse(data);
    }

    for other_parser in ALL_PARSERS {
        if other_parser.can_parse(data) {
            return Err(format!("expected '{0}' but got '{1}'", parser.name(), other_parser.name())); // todo: translations? data structure?
        }
    }
    Err(format!("expected '{0}' but got unknown data", parser.name())) // todo
}


struct FluentParser {
    data: String,
    error: Option<String>,
    done: bool,
}

type DataHandler<T> = dyn FnMut(T);

impl FluentParser {
    pub fn parse(data: String) -> Self {
        FluentParser {
            data,
            error: None,
            done: false,
        }
    }

    pub fn into_on_chain(self, handler: Box<DataHandler<Address>>) -> Self {
        self.into(&Address::PARSER, handler)
    }

    pub fn into<T>(mut self, parser: &dyn Parser<T>, mut handler: Box<DataHandler<T>>) -> Self {
        if self.done {
            return self;
        }
        if parser.can_parse(&self.data) {
            match parser.parse(&self.data) {
                Ok(parsed) => handler(parsed),
                Err(error) => self.error = Some(error)
            }
            self.done = true;
        }
        self
    }

    pub fn ignore_errors(self) {}

    pub fn or_fail(self) -> Result<(), String> {
        if self.done {
            return match self.error {
                Some(error) => Err(error),
                _ => Ok(())
            }
        }
        for other_parser in ALL_PARSERS {
            if other_parser.can_parse(&self.data) {
                return Err(format!("'{0}' is not supported.", other_parser.name())); // todo: translations? data structure?
            }
        }
        Err("got unknown data".to_string()) // todo
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::*;

    #[test]
    fn address_parser() {
        let address = "3P14159f73E4gFr7JterCCQh9QjiTjiZrG";

        let parsed = Address::PARSER.parse(&address.to_string()).unwrap();

        assert_eq!(parsed.raw, address);
        assert_eq!(parsed.address_type, AddressType::P2sh);
        assert_eq!(parsed.network, Network::Bitcoin);
    }

    #[test]
    fn parser() {
        let address = "3P14159f73E4gFr7JterCCQh9QjiTjiZrG";

        let parsed = parse(&address.to_string(), &Address::PARSER).unwrap();

        assert_eq!(parsed.raw, address);
        assert_eq!(parsed.address_type, AddressType::P2sh);
        assert_eq!(parsed.network, Network::Bitcoin);
    }

    #[test]
    fn fluent_parser() {
        let address = "3P14159f73E4gFr7JterCCQh9QjiTjiZrG";

        let parsed: Rc<RefCell<Option<Address>>> = Rc::new(RefCell::new(None));
        let captured_parsed = parsed.clone();

        FluentParser::parse(address.to_string())
            .into_on_chain(Box::new(move |address: Address| {
                *captured_parsed.borrow_mut() = Some(address.clone());
            }))
            .ignore_errors();

        let actual = parsed.take().unwrap();
        assert_eq!(actual.raw, address);
        assert_eq!(actual.address_type, AddressType::P2sh);
        assert_eq!(actual.network, Network::Bitcoin);
    }

    #[test]
    fn fluent_parser_error() {
        let address = "3P14159f73E4gFr7JterCCQh9QjiTjiZr";

        let parsed: Rc<RefCell<Option<Address>>> = Rc::new(RefCell::new(None));
        let captured_parsed = parsed.clone();

        let error = FluentParser::parse(address.to_string())
            .into_on_chain(Box::new(move |address: Address| {
                *captured_parsed.borrow_mut() = Some(address.clone());
            }))
            .or_fail();

        assert_eq!(error.err(), Some("got unknown data".to_string()));
    }
}

