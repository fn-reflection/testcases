mod unittest;
pub mod board {
    include!(concat!(env!("OUT_DIR"), "/board.v1.rs"));
}
pub mod test_proto {
    include!(concat!(env!("OUT_DIR"), "/test_proto.v1.rs"));
}
