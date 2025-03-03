// Use this in build.rs
fn main() -> Result<(), Box<dyn std::error::Error>> {
    protobuf_codegen::Codegen::new()
        // Use `protoc` parser, optional.
        .protoc()
        // All inputs and imports from the inputs must reside in `includes` directories.
        .includes(&["./proto"])
        // Inputs must reside in some of include paths.
        .input("./proto/DENKbuffer.proto")
        .input("./proto/modelfile-v2.proto")
        .input("./proto/denkcache.proto")
        .input("./proto/inference_graph.proto")
        // Specify output directory relative to Cargo output directory.
        .cargo_out_dir("protos")
        .run_from_script();

    tonic_build::compile_protos("proto/denkcache.proto")?;
    Ok(())
}
