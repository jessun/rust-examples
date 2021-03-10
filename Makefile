test:
	RUST_LOG=trace RUSTFLAGS="-Awarnings" RUST_BACKTRACE=full cargo test -j 32 -- --nocapture
