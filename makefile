build:
	cargo build

build-release:
	cargo build --release

run:
	cargo run

check:
	cargo check

clean:
	rm -rf target/
	mkdir target
	
# TODO: add jobs for common chores like version bump, lint, formating, tests