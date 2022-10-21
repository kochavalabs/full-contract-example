extern crate mazzaroth_rs;
extern crate mazzaroth_rs_derive;

#[macro_use]
extern crate xdr_rs_serialize_derive;

pub mod xdr;

use xdr_rs_serialize::ser::XDROut;
 
use mazzaroth_rs::external::{sql, transaction};
use mazzaroth_rs::ContractInterface;
use mazzaroth_rs_derive::mazzaroth_abi;
use xdr::*;

#[no_mangle]
pub fn entry() {
    std::panic::set_hook(Box::new(mazzaroth_rs::external::errors::hook));

    // Creates a new instance of the ABI generated around the Contract
    let mut contract = ExampleCT::new(Example {});

    let args = transaction::arguments();

    // Execute calls one of the functions defined in our contract
    // Input for the function to call and it's params comes from the Runtime
    let response = contract.execute(&args).unwrap();

    transaction::ret(response);
}

#[mazzaroth_abi(ExampleCT)]
pub trait ExampleContract {
    fn setup(&mut self) -> bool;

    #[readonly]
    fn simple(&mut self) -> String;

    fn args(&mut self, one: String, two: String, three: String) -> u32;

    fn complex(&mut self, foo_arg: Foo, bar_arg: Bar) -> String;

    fn insert_foo(&mut self, foo: Foo) -> Vec<i32>;

    fn query_foo(&mut self, where_clause: String) -> Vec<Foo>;
}

pub struct Example {}

impl ExampleContract for Example {
    fn setup(&mut self) -> bool {
        let tables = vec!["foo", "bar"];
        for table in &tables {
            match sql::exec(format!("CREATE TABLE {};", table)) {
                Some(_) => panic!("Error creating table {}", table),
                None => {}
            };
        }
        return true;
    }

    fn simple(&mut self) -> String {
        "Hello World!".to_string()
    }

    fn args(&mut self, one: String, two: String, three: String) -> u32 {
        format!("{}{}{}", one, two, three).len() as u32
    }

    fn complex(&mut self, foo_arg: Foo, bar_arg: Bar) -> String {
        match foo_arg.status {
            FooStatus::Zero => format!("Zero: {:?}", bar_arg.id.t[0]),
            FooStatus::One => format!("One: {:?}", bar_arg.id.t[0]),
            FooStatus::Two => format!("Two: {:?}", bar_arg.id.t[0]),
            FooStatus::Three => format!("Three: {:?}", bar_arg.id.t[0]),
        }
    }

    fn insert_foo(&mut self, foo: Foo) -> Vec<i32> {
        let mut json_out: Vec<u8> = Vec::new();
        foo.write_json(&mut json_out).unwrap();
        let query = std::str::from_utf8(&json_out).unwrap().to_string();
        sql::insert("foo".to_string(), query).unwrap();
        return vec![];
    }

    fn query_foo(&mut self, where_clause: String) -> Vec<Foo> {
        let mut query = format!("SELECT * FROM foo");
        if where_clause != "".to_string() {
            query = format!("{} WHERE {}", query, where_clause);
        }
        match sql::exec(query) {
            Some(result) => xdr_rs_serialize::de::read_json_string(
                std::str::from_utf8(&result).unwrap().to_string(),
            )
            .unwrap(),
            None => vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_function() {
        let mut contract = Example {};
        assert_eq!("Hello World!".to_string(), contract.simple());
    }

    #[test]
    fn args_function() {
        let mut contract = Example {};
        assert_eq!(
            11,
            contract.args("One".to_string(), "Two".to_string(), "Three".to_string())
        );
    }

    #[test]
    fn complex_function() {
        let mut contract = Example {};
        let mut foo = Foo::default();
        let mut bar = Bar::default();
        foo.status = FooStatus::Three;
        bar.id.t = vec![1; 32];
        assert_eq!("Three: 1".to_string(), contract.complex(foo, bar));
    }
}
