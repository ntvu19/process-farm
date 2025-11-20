.PHONY: help proto clean

help:
	@echo "Usage: make <target>"
	@echo "Targets:"
	@echo "  proto - Generate protobuf code using Rust"
	@echo "  clean - Clean up the generated code and binary files"

# Generate protobuf code using Rust
proto:
	@echo "Generating protobuf code using Rust..."
	cargo build --manifest-path payload/Cargo.toml

# Clean up the generated code and binary files
clean:
	@echo "Cleaning up protobuf generated code..."
	@if [ -d payload/generated ]; then rm -rf payload/generated; fi

	@echo "Cleaning up binary files..."
	@if [ -d payload/target ]; then rm -rf payload/target; fi