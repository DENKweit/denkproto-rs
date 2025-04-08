// Use this in build.rs
fn main() -> Result<(), Box<dyn std::error::Error>> {
    prost_validate_build::Builder::new().compile_protos(
        &[
            "./proto/inference_graph.proto",
            "./proto/modelfile-v2.proto",
            "./proto/denkcache.proto",
        ],
        &["./proto"],
    )?;
    Ok(())
}
