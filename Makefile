all: default_compile

default_compile:
	cargo build --release

only_unpacking:
	cargo build --release --features only_unpacking

clean:
	cargo clean

