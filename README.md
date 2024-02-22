# denkproto-rs

This Rust crate contains auto-generated protobuf bindings for the denkweit ecosystem. The bindings are generated from `.proto` files and should not be manually edited, as changes will be overwritten when the files are re-generated.

## Usage

To use these bindings in your Rust project, add `denkproto-rs` as a dependency in your `Cargo.toml` file.

## Generation

The protobuf bindings are generated using `protoc`, the protobuf compiler. The generation process is automated through a Makefile in the root of the repository.

## Note

Please ensure that you have the same version of the protobuf runtime as specified in the generated files to avoid compatibility issues.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
