BUILD_VARS=OPENSSL_INCLUDE_DIR=/usr/include/openssl-1.0 OPENSSL_LIB_DIR=/usr/lib/openssl-1.0

#LOG=RUST_LOG=DEBUG
LOG=

all: run

build:
	$(LOG) $(BUILD_VARS) cargo build

update:
	$(LOG) $(BUILD_VARS) cargo update

run: build
	$(LOG) $(BUILD_VARS) cargo run

json: build
	$(LOG) $(BUILD_VARS) cargo run json
