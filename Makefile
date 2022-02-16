.PHONY: all generate-xdr build test integration testall deploy

all: generate-xdr build testall

generate-xdr:
	xdr-codegen ./xdr/*.x --language rust | rustfmt > contract/src/xdr.rs
	xdr-codegen ./xdr/*.x --language commonjs --output 'xdrTypes.js'

build:
	cargo build --manifest-path=contract/Cargo.toml --release --target wasm32-unknown-unknown

test:
	cargo test --manifest-path=contract/Cargo.toml --features mazzaroth-rs/host-mock -- --test-threads=1

integration:
	m8 channel exec test --test-manifest test.yaml

testall: test integration

deploy:
	m8 channel exec deployment --deployment-manifest deploy.yaml
