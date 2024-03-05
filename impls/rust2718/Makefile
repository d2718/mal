
step0_repl: Cargo.toml

step1_repl: Cargo.toml src/libs.rs src/printer.rs src/reader.rs src/types.rs

%: src/bin/%.rs
	cargo build --bin $*
	cp target/debug/$* $@

	
clean:
	cargo clean
	rm step[0-9]_*
