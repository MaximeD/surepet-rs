
# Lint the files.
lint:
	cargo fmt

# Create a binary release.
release:
	cargo build --release

# Run the tests.
test:
	cargo test
