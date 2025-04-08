// include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));

pub mod modelfile {
    pub mod v2 {
        include!(concat!(env!("OUT_DIR"), "/modelfile.v2.rs"));
    }
}

pub mod denkcache {
    include!(concat!(env!("OUT_DIR"), "/denkcache.rs"));
}

pub mod denkcache_proto {
    include!(concat!(env!("OUT_DIR"), "/inference_graph.rs"));
}
