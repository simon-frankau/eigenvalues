all: out/dist.png

out/dist.png: src/main.rs
	mkdir -p out/
	cargo +nightly run --release eigenvalues

clean:
	rm -r out/
	cargo clean
