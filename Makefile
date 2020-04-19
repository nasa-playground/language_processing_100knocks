build:
	RUSTFLAGS="${RUSTFLAGS} -A dead_code" cargo build
test:
	RUSTFLAGS="${RUSTFLAGS} -A dead_code" cargo test
