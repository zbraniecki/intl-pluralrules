extern crate intl_pluralrules;

// use intl_pluralrules::*;

fn main() {
	println!("{:?}", intl_pluralrules::PluralOperands::new("5"));
	println!("{:?}", intl_pluralrules::PluralOperands::new("55.0"));
}
