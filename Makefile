.PHONY : release
release : x86_64-apple-darwin

.PHONY : x86_64-apple-darwin
.PHONY : x86_64-unknown-linux-musl
x86_64-apple-darwin x86_64-unknown-linux-musl :
	cargo build --release --target $@
	cp ./target/$@/release/swc2dot .
	tar -czf ./release/"swc2dot-"$$(grep "version" Cargo.toml | grep -o "[[:digit:].]\+")"-"$$(git rev-parse --short HEAD)"-"$@.tar.gz ./swc2dot ./README.md ./LICENSE
	rm ./swc2dot
