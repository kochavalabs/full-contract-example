# Full Contract Example

The repo is meant to be a quick tutorial that will familiarize users with
contract development. It's meant to give an overview of how to write, test and
deploy a contract. We will also cover the Mazzaroth CLI tools, XDR code
generation and interacting with a Mazzaroth node from the browser.

## Example Contract

An example contract for this tutorial is located in the [contract/](https://github.com/kochavalabs/full-contract-example/tree/master/contract)
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
cargo run --manifest-path=xdr-codegen/Cargo.toml ./xdr/*.x --language rust \
  | rustfmt > contract/src/xdr.rs

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

Deploying your built contract to a Mazzaroth node will require the mazzaroth
cli. You can install the Mazzaroth-CLI with npm.

```bash
npm install -g mazzaroth-cli
```

## Start a Mazzaroth Standalone Node

TODO

## Deploy a Contract to the Channel

Once you have a new Mazzaroth standalone node running, you will need to deploy
the contract to the node before you can execute any functions on it.

```bash
# To see more info about mazzaroth-cli run.
mazzaroth-cli help

# We'll be running this example with the default private key of all 0s, the
# account key in the nonce-lookup below is its corresponding public key.
#
# You can look up the current nonce for the account with the following
# command. (Update host appopriately to be your node's ip address)
mazzaroth-cli nonce-lookup \
  e0b1fe74117e1b95b608a4f221df314774b20ea66842350d515371c7c6966c6e \
  --host='http://localhost:8081'

# Then deploy the contract.
mazzaroth-cli contract-update \
  contract/target/wasm32-unknown-unknown/release/contract.wasm \
  --nonce="0" \
  --host='http://localhost:8081'

# For a readonly call that returns an uniterpreted base64 result you can call
# the 'simple' function on the contract.
# Note that no nonce is required for the readonly call.
mazzaroth-cli readonly-call simple  --host='http://localhost:8081'
```

This covers the basics for deploying your contract. You can read more about the
Mazzaroth CLI in its [repo](https://github.com/kochavalabs/mazzaroth-cli), for
the remainder of the tutorial we'll be using the contract-cli.

## Contract CLI

Operations like those above are relatively low level. Many of the results need
to be interpreted from base64 strings or require multiple calls to complete. For
example to complete a 'transaction-call', you would need to look up an account
nonce, make the call, and finally lookup the results after execution. An example
of this being done (using  node.js) can be seen in the
[mazzaroth-js](https://github.com/kochavalabs/mazzaroth-js) repo.

This is cumbersome, so we've provided a further abstraction called the contract
client. This wraps the low level operations and gives the user access to their
contract through an rpc-like interface. We'll walk through how to drop into the
contract clients interactive CLI for our example contract.

```bash
# The contract client requires the ABI json produced from our contract to run
# properly. Which will drop you into an interactive CLI.
mazzaroth-cli contract-cli contract/target/json/ExampleContract.json
Mazz>

# You can see the currently available functions by typing abi
Mazz> abi

Functions:
  args(string, string, string) -> uint32
  complex(Foo, Bar) -> string

ReadOnly Functions:
  simple() -> string

# And call them
Mazz> simple()
Hello World!
Mazz> args("one", "two", "three")
11
Mazz> complex('', '')
Error: Type not identified: Foo
    at nodeClient.nonceLookup.then.result
    at process._tickCallback
Mazz>
```

You will notice that we had an error when trying to call our 'complex' function
from the Mazzaroth interactive CLI. This is because the contract CLI does not
know how to interpret our custom XDR types, Foo and Bar. We can give it the
correct information by generating the correct javascript XDR file for it to
interpret our custom types.

```bash
# We can generate javascript XDR code with the following command. This is output
# as ES6 compatible javascript, but must be translated using babel to work
# correctly with node. We've checked in an already translated xdrTypes.js file
# to make this step easier. The command for generating JS would be:
# cargo run --manifest-path=xdr-codegen/Cargo.toml ./xdr/*.x --language js \
#   --output 'xdrTypes.js'

Mazz> complex('{"status": 1, "one": "one__", "two": "two__", "three": "three__"}', '{ "id": "9000000000000000000000000000000000000000000000000000000000000000" }')
One: 144

# Alternatively, you can give files as arguments.
Mazz> complex(f:"foo.json", f:"bar.json")
One: 144
```

## Run The Browser Example

TODO