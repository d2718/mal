%: src/bin/%.rs
	cargo build --bin $*
	cp target/debug/$* $@

step0_repl: Cargo.toml

step1_read_print: Cargo.toml src/lib.rs src/printer.rs src/reader.rs src/types.rs src/bin/step1_read_print.rs
	cargo build --bin step1_read_print
	cp target/debug/step1_read_print ./
	
tidy:
	rm step[0-9]_*
	
clean:
	cargo clean
	rm step[0-9]_*
