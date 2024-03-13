%: src/bin/%.rs
	cargo build --bin $*
	cp target/debug/$* $@

step0_repl: Cargo.toml

step1_read_print: Cargo.toml src/*.rs src/*/*.rs src/bin/step1_read_print.rs
	cargo build --bin step1_read_print
	cp target/debug/step1_read_print ./
	
step2_eval: Cargo.toml src/* src/bin/step2_eval.rs
	cargo build --bin step2_eval
	cp target/debug/step2_eval ./

step3_env: Cargo.toml src/* src/bin/step3_env.rs
	cargo build --bin step3_env
	cp target/debug/step3_env ./
	
tidy:
	rm step[0-9]_*
	
clean:
	cargo clean
	rm step[0-9]_*
