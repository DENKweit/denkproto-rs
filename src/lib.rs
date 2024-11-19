include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));

pub mod denkcache_proto {
    tonic::include_proto!("denkcache"); // The string specified here must match the proto package name
}
