init:
	@echo "Initializing submodules"
	cargo install trunk
	rustup toolchain install nightly
	rustup override set nightly
	rustup target add wasm32-unknown-unknown

run:
	@echo "Running the app"
	cargo clippy --fix & cargo fmt
	trunk serve --port 3000 --open
