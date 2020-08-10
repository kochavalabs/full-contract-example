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

We'll have to remove the [macro_use] section from the generated rust code.
Since the xdr.rs file is pulled into the contract as a module it cannot
contain the macro use line which is already present for the `xdr_rs_serialize_derive`
extern crate in main.rs.
You can run this command or make the changes manually.

```Bash
sed -i '' -e '/macro_use/d' contract/src/xdr.rs
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

## Writing Integration Tests

Although some of a contract's logic can be tested using standard rust unit tests,
integration tests are necessary for testing host functions and other higher level
logic. [Mazzaroth-it](https://github.com/kochavalabs/mazzaroth-it) is a relatively
straight forward node script that helps by automating some of the repetitive
tasks related with running integration tests.

Example integration tests have been added to the `/tests` directory and can be run
with Mazzaroth-it.

Run the following commands to install and run the integration tests:

```Bash
npm install -g mazzaroth-it


mazzaroth-it test --config tests
```

Visit the [Mazzaroth-it](https://github.com/kochavalabs/mazzaroth-it) repository
for more details on how to setup test config files.

## Install The Mazzaroth CLI

Deploying your built contract to a Mazzaroth node will require the mazzaroth
cli. You can install the Mazzaroth-CLI with npm.

```bash
npm install -g mazzaroth-cli
```

## Start a Mazzaroth Standalone Node

Mazzaroth is provided as an image on [Docker Hub](https://hub.docker.com/r/kochavalabs/mazzaroth).

To run a standalone node with port 8081 exposed for http access
use the following command:

```Bash
docker run -p 8081:8081 kochavalabs/mazzaroth start standalone
```

## Deploy a Contract to the Channel

There are a few steps required to deploy a contract to a new
Mazzaroth node.  To help with these steps we have included a
deploy command that takes a deploy.json config.

An example deploy.json is included in this repository.  Simply
run the following command to automatically deploy the contract
to a node running on localhost with port 8081.

```Bash
mazzaroth-cli deploy deploy.json
```

### Manual Transaction Execution

Once you have a new Mazzaroth standalone node running, you will need to deploy
the contract to the node before you can execute any functions on it.

```bash
# To see more info about mazzaroth-cli run.
mazzaroth-cli help

# We'll be running this example with the default private key of all 0s, the
# account key in the nonce-lookup below is its corresponding public key.
#
# You can look up the current nonce for the account with the following
# command. (Update host appropriately to be your node's ip address)
mazzaroth-cli nonce-lookup \
  3b6a27bcceb6a42d62a3a8d02a6f0d73653215771de243a63ac048a18b59da29 \
  --host='http://localhost:8081'

# For a readonly call that returns a JSON result you can call
# the 'simple' function on the contract.
# Note that no nonce is required for the readonly call.
mazzaroth-cli readonly-call simple --host='http://localhost:8081'

# For a non-readonly Transaction Execution make sure to set the nonce from the previous lookup
mazzaroth-cli transaction-call args -a "one" -a "two" -a "three" --nonce 9

# Provide the output transaction id from the previous command to a
# receipt lookup to get the result of the transaction execution
mazzaroth-cli receipt-lookup d82feb998c035949384ec058399d8d5697bcce82d7d2eeb8dd49be6f1b6f0c9b
```

This covers the basics for executing transactions against your contract.
You can read more about the Mazzaroth CLI in its
[repo](https://github.com/kochavalabs/mazzaroth-cli). For
the remainder of the tutorial we'll be using the contract-cli.

## Contract CLI

Operations like those above are relatively low level. Many of the results need
to be interpreted from base64 strings or require multiple calls to complete. For
example to complete a 'transaction-call', you would need to look up an account
nonce, make the call, and finally lookup the results after execution. An example
of this being done (using node.js) can be seen in the
[mazzaroth-js](https://github.com/kochavalabs/mazzaroth-js) repo.

This is cumbersome, so we've provided a further abstraction called the contract
client. This wraps the low level operations and gives the user access to their
contract through an rpc-like interface. We'll walk through how to drop into the
contract clients interactive CLI for our example contract.

```bash
# The contract client requires the ABI json produced from our contract to run
# properly. Which will drop you into an interactive CLI.
mazzaroth-cli contract-cli contract/target/json/ExampleContract.json -x xdrTypes.js
Mazz>

# You can see the currently available functions by typing abi
Mazz> abi

Functions:
  setup() -> bool
  args(string, string, string) -> uint32
  complex(Foo, Bar) -> string
  insert_foo(Foo) -> int32[]
  query_foo(string) -> Foo[]

ReadOnly Functions:
  simple() -> string

# And call them
Mazz> simple()
Hello World!
Mazz> args("one", "two", "three")
11
```

You will notice that some functions take custom objects. If you did not provide
the `-x xdrTypes.js` argument in the contract-cli command the
contract CLI will not know how to interpret our custom XDR types, Foo and Bar.
If you did provide the javascript XDR file then you should be able to call those
functions by passing in a JSON version of the objects.

```bash
Mazz> complex('{"status": 1, "one": "one__", "two": "two__", "three": "three__"}', '{ "id": "9000000000000000000000000000000000000000000000000000000000000000" }')
One: 144

# Alternatively, you can give files as arguments.
Mazz> complex(f:"foo.json", f:"bar.json")
One: 144
```

## Run The Browser Example

We've put together an example calling a Mazzaroth node from the browser. It is
found in the [browser/](https://github.com/kochavalabs/full-contract-example/tree/master/browser)
directory. This example simply constructs a node-client and contract-client then
makes a call to the Mazzaroth node to call the 'simple' function on our
contract. This is done in the
[HelloWorld.vue](https://github.com/kochavalabs/full-contract-example/tree/master/browser/src/components/HelloWorld.vue)
file. The results are then displayed.

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
