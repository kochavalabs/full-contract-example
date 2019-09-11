# Full Contract Example

The repo is meant to be a quick tutorial that will familiarize users with
contract development. It's meant to give an overview of how to write, deploy and
update a contract. We will also cover the Mazzaroth CLI tools, XDR code
generation, writing unit tests and interacting with a mazzaroth node from the
browser.

## Example Contract

An example contract for this tutorial is located in the [contract/](https://github.com/kochavalabs/full-contract-example/tree/develop/contract)
directory.

## Generating XDR Objects

We provide the ability to use standard Rust structs inside the contract trait
method signatures. The method that does this in the example contract is the
"complex" method. It is not necessary to use rust structs, but if you want to,
they should be generated from XDR language files.

For this contract we've defined our XDR under the xdr/ directory and they can
be used to generate rust code with the following commands:

```bash
# We've included a git submodule for the code generation tool, xdr-codegen
git submodule update --init

# To generate the xdr files and run rustfmt
cargo run --manifest-path=xdr-codegen/Cargo.toml ./xdr/*.x --language rust | rustfmt > contract/src/xdr.rs

# We'll have to remove the [macro_use] section from the generated rust code.
# You can run this command or make the changes manually.
sed -i '/macro_use/d' contract/src/xdr.rs
```

## Writing Unit Tests

With a few modifications you can write unit tests for your contract using the
standard rust testing library. The one thing that slightly complicates things
is our use of host functions within the contract. These are mocked out in
mazzaroth-rs behind the rust host-mock feature. You can simply add the following
feature to your Cargo.toml and execute test as follows.

```bash
# Add these feature to your Cargo.toml
# [features]
# host-mock = []

cargo test --features "host-mock"

# If using host functions in your contract it is best to limit the tests to
# one thread:
cargo test --features "host-mock" -- --test-threads=1
```


## Building the Contract

For a contract to be built you can run the following commands:

```bash
# You must install the latest rust wasm tools
rustup toolchain install nightly
rustup update
rustup target add wasm32-unknown-unknown --toolchain nightly

# Then build the contract.
cargo +nightly build --release --target wasm32-unknown-unknown
```

This will output a couple of files that are worth noting. First is located at
contract/target/json/ExampleContract.json, and is a json file that contains the
contract ABI (a description of the functions in the contract). This will be used
when we interact with some of the CLI tools later in the tutorial. The second
file that is important is the contract wasm output located at
contract/target/wasm32-unknown-unknown/release/contract.wasm. This is what will
be uploaded to our Mazzaroth node to be executed against.

## Install The Mazzaroth CLI

TODO

## Start a Mazzaroth Standalone Node

TODO

## Deploy a Contract to the Channel

TODO

## Run The Browser Example

TODO

## Update The Contract

TODO
