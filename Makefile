init:
	@echo "Initializing submodules"
	cargo install trunk
	rustup toolchain install nightly
	rustup override set nightly
	rustup target add wasm32-unknown-unknown
	npm i -g tailwindcss
	# tailwindcss -o ./tailwind.css

run:
	@echo "Running the app"
	cargo clippy --fix & cargo fmt
	trunk serve --port 3000 --open

build:
	@echo "Building the app"
	NODE_ENV=production tailwindcss -c ./tailwind.config.js -o ./tailwind.css --minify
	trunk build --release
