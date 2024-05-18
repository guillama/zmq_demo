use std::io::Result;

pub const PROTO_FILE: &str = "zmqproto.proto";
pub const PROTO_INCLUDE_DIR: &str = "../protobuf/";

// This program is compiled and run during the build process.
// It generate rust code from the protobuf file
fn main() -> Result<()>{
    prost_build::compile_protos(&[PROTO_FILE], &[PROTO_INCLUDE_DIR])?;
    Ok(())
}