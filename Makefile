RUST_DIR := $(shell readlink -m $(shell dirname $(firstword $(MAKEFILE_LIST))))
PWD = $(shell pwd)
export TARGET_CC=${PWD}/vendor/x86_64-linux-musl-native/bin/x86_64-linux-musl-gcc
export TARGET_CMAKE_TOOLCHAIN_FILE=${PWD}/nitro-revm.cmake
export CC=${PWD}/vendor/x86_64-linux-musl-native/bin/x86_64-linux-musl-gcc
export CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER = ${PWD}/vendor/x86_64-linux-musl-native/bin/ld
build:
	curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh -s -- -y
	# yeah - this next line + sudo in make is dumb lmao, will revisit this
	source "${HOME}/.cargo/env"
	rustup target install x86_64-unknown-linux-musl
	sudo yum install -y openssl-devel protobuf-compiler cmake clang
	cargo build --manifest-path=${RUST_DIR}/Cargo.toml --target=x86_64-unknown-linux-musl --release --verbose
	cp ${RUST_DIR}/target/x86_64-unknown-linux-musl/release/nitro_revm ${RUST_DIR}/
	cp ${RUST_DIR}/target/x86_64-unknown-linux-musl/release/revm_driver ${RUST_DIR}/

server: build
	docker build -t nitro-revm-server -f Dockerfile.server .
	nitro-cli build-enclave --docker-uri nitro-revm-server --output-file nitro-revm-server.eif

.PHONY: clean
clean:
	rm -rf ${RUST_DIR}/target ${RUST_DIR}/vsock_sample_*.eif ${RUST_DIR}/vsock-sample
