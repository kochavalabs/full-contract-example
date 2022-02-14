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
be used to generate rust code with the [xdr-codegen](https://crates.io/crates/xdr-codegen)
tool.

First install or update xdr-codegen

```Bash
cargo install xdr-codegen
```

Then run the xdr-codegen command to generate the xdr rust file:

```Bash
xdr-codegen ./xdr/*.x --language rust | rustfmt > contract/src/xdr.rs
```

If you want to use the Mazzaroth-CLI or JavaScript to interact with
the contract you will also need the JavaScript generated file.  This
can be generated with the command below.

```Bash
xdr-codegen ./xdr/*.x --language commonjs --output 'xdrTypes.js'
```

## Writing Unit Tests

With a few modifications you can write unit tests for your contract using the
standard rust testing library. The one thing that slightly complicates things
is our use of host functions within the contract. These are mocked out in
mazzaroth-rs behind the rust host-mock feature. You can specify using the
host-mock feature for the mazzaroth-rs library through the command line.

```bash
# Unit testing requires enabling the host-mock feature for the mazzaroth-rs
# library.
cargo test --features mazzaroth-rs/host-mock

# If using host functions in your contract it is best to limit the tests to
# one thread:
cargo test --features mazzaroth-rs/host-mock -- --test-threads=1
```

## Building the Contract

For a contract to be built you can run the following commands:

```bash
# You must install the latest rust wasm tools
rustup toolchain install
rustup update
rustup target add wasm32-unknown-unknown

# Then build the contract.
cargo build --release --target wasm32-unknown-unknown
```

This will output a couple of files that are worth noting. First is located at
contract/target/json/ExampleContract.json, and is a json file that contains the
contract ABI (a description of the functions in the contract). This will be used
when we interact with some of the CLI tools later in the tutorial. The second
file that is important is the contract wasm output located at
contract/target/wasm32-unknown-unknown/release/contract.wasm. This is what will
be uploaded to our Mazzaroth node to be executed against.

## Install m8

Deploying your built contract to a Mazzaroth node will require the mazzaroth
cli tool `m8`. You can install m8 with from the github repo:

```bash
go install github.com/kochavalabs/m8@latest
```

## Start a Mazzaroth Standalone Node

Mazzaroth is provided as an image on [Docker Hub](https://hub.docker.com/r/kochavalabs/mazzaroth).

To run a standalone node with port 8081 exposed for http access
use the following command:

```Bash
docker run -p 6299:6299 kochavalabs/mazzaroth start standalone
```

## Writing Integration Tests

Although some of a contract's logic can be tested using standard rust unit tests,
integration tests are necessary for testing host functions and other higher level
logic. [m8](https://github.com/kochavalabs/m8) also has the ability to execute an
integration test config against a running Mazzaroth node.

Example integration tests have been added to the `/tests` directory and can be run
with m8.

Run the following commands to run the integration tests:

```Bash
m8 channel test --test-manifest test.yaml
```

Visit the [m8](https://github.com/kochavalabs/m8) repository
for more details on how to setup test config files.

## Deploy a Contract to the Channel

There are a few steps required to deploy a contract to a new
Mazzaroth node.  To help with these steps we have included a
deploy command that takes a deploy.yaml config.

An example deploy.yaml is included in this repository.  Simply
run the following command to automatically deploy the contract
to a node running on localhost with port 6299.

```Bash
m8 channel deploy --deployment-manifest deploy.yaml
```

### Manual Transaction Execution

Once you have a new Mazzaroth standalone node running, you will need to deploy
the contract to the node before you can execute any functions on it.

```bash
# To see more info about mazzaroth-cli run.
mazzaroth-cli help

# For a readonly call that returns a JSON result you can call
# the 'simple' function on the contract.
mazzaroth-cli transaction-call simple

# For a non-readonly Transaction Execution you will need to save the returned
# transaction id to perform a lookup of the receipt
mazzaroth-cli transaction-call args -a "one" -a "two" -a "three"

# Provide the output transaction id from the previous command to a
# receipt lookup to get the result of the transaction execution
mazzaroth-cli receipt-lookup 57bf4efde900eba94b525a7ec997558da426893244705007e77d36f1d65d4aec
```

This covers the basics for executing transactions against your contract.
You can read more about the Mazzaroth CLI in its
[repo](https://github.com/kochavalabs/mazzaroth-cli). For
the remainder of the tutorial we'll be using the contract-cli.

## Run The Browser Example

We've put together an example calling a Mazzaroth node from the browser. It is
found in the [browser/](https://github.com/kochavalabs/full-contract-example/tree/master/browser)
directory. This example simply constructs a node-client and contract-client then
makes a call to the Mazzaroth node to call the 'simple' function on our
contract. This is done in the
[HelloWorld.vue](https://github.com/kochavalabs/full-contract-example/tree/master/browser/src/components/HelloWorld.vue)
file. The results are then displayed.

The Browser example targets a Mazzaroth Node running on localhost:8081 so for this
example to work make sure you have performed the [Start](#Start-a-Mazzaroth-Standalone-Node)
and [Deploy](#Deploy-a-Contract-to-the-Channel) steps and that the
node is not stopped.

To run:

```bash
# From the repo root directory
cd browser

# To install the node dependencies during development we ran:
# npm install --save mazzaroth-js
npm install

# As long as a node is running, the result of the call to simple should display
# on the page -> 'Hello World!'
npm run serve
```
