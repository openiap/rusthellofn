.PHONY: all build clean
all: build
build:
	cargo build --release --target x86_64-unknown-linux-musl
	cp target/x86_64-unknown-linux-musl/release/rusthellofn ./rusthellofn
	chmod +x rusthellofn
clean:
	cargo clean
	rm -f rusthellofn