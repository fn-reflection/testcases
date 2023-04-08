mod unittest;
pub mod protobuf {
    pub mod v1 {
        include!(concat!(env!("OUT_DIR"), "/protobuf.v1.rs"));
    }
    pub mod v2 {
        include!(concat!(env!("OUT_DIR"), "/protobuf.v2.rs"));
    }
}
