#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

#[allow(unused_imports)]
use std::io::Write;
#[allow(unused_imports)]
use xdr_rs_serialize::de::{
    read_fixed_array, read_fixed_array_json, read_fixed_opaque, read_fixed_opaque_json,
    read_var_array, read_var_array_json, read_var_opaque, read_var_opaque_json, read_var_string,
    read_var_string_json, XDRIn,
};
use xdr_rs_serialize::error::Error;
#[allow(unused_imports)]
use xdr_rs_serialize::ser::{
    write_fixed_array, write_fixed_array_json, write_fixed_opaque, write_fixed_opaque_json,
    write_var_array, write_var_array_json, write_var_opaque, write_var_opaque_json,
    write_var_string, write_var_string_json, XDROut,
};

extern crate json;

// Namespace start example

// Start typedef section

#[derive(PartialEq, Clone, Default, Debug, XDROut, XDRIn)]
pub struct ID {
    #[array(fixed = 32)]
    pub t: Vec<u8>,
}
// End typedef section

// Start struct section

#[derive(PartialEq, Clone, Default, Debug, XDROut, XDRIn)]
pub struct Bar {
    pub id: ID,
}
// End struct section

// Start union section

// End union section

// Namespace end example
// Namespace start example

// Start typedef section

// End typedef section

// Start struct section

#[derive(PartialEq, Clone, Default, Debug, XDROut, XDRIn)]
pub struct Foo {
    pub status: FooStatus,
    #[array(var = 256)]
    pub one: String,
    #[array(var = 256)]
    pub two: String,
    #[array(var = 256)]
    pub three: String,
}
// End struct section

#[derive(PartialEq, Clone, Debug, XDROut, XDRIn)]
pub enum FooStatus {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
}

impl Default for FooStatus {
    fn default() -> Self {
        FooStatus::Zero
    }
}
// Start union section

// End union section

// Namespace end example
