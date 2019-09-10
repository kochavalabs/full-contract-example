extern crate mazzaroth_rs;
extern crate mazzaroth_rs_derive;

use mazzaroth_rs::external::transaction;
use mazzaroth_rs::ContractInterface;
use mazzaroth_rs_derive::mazzaroth_abi;

#[no_mangle]
pub fn main() {
    std::panic::set_hook(Box::new(mazzaroth_rs::external::errors::hook));

    // Creates a new instance of the ABI generated around the Contract
    let mut contract = HelloWorld::new(Hello {});

    let args = transaction::arguments();

    // Execute calls one of the functions defined in our contract
    // Input for the function to call and it's params comes from the Runtime
    let response = contract.execute(&args).unwrap();

    transaction::ret(response);
}

#[mazzaroth_abi(HelloWorld)]
pub trait HelloWorldContract {
    fn constructor(&mut self);

    // Simply say hello :)
    fn hello(&mut self) -> u32;
}

pub struct Hello {}

impl HelloWorldContract for Hello {
    fn constructor(&mut self) {
    }

    fn hello(&mut self) -> u32 {
        14
    }
}
