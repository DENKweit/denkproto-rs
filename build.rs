// Use this in build.rs
fn main() {
    protobuf_codegen::Codegen::new()
        // Use `protoc` parser, optional.
        .protoc()
        // All inputs and imports from the inputs must reside in `includes` directories.
        .includes(&["../proto"])
        // Inputs must reside in some of include paths.
        .input("../proto/DENKbuffer.proto")
        .input("../proto/modelfile-v2.proto")
        // Specify output directory relative to Cargo output directory.
        .out_dir("src/gen")
        .run_from_script();
}
