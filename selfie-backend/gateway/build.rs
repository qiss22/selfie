use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_files = vec![
        "proto/auth.proto",
        "proto/user.proto",
        "proto/post.proto",
        "proto/media.proto",
        "proto/chat.proto",
    ];

    let out_dir = PathBuf::from(env::var("OUT_DIR")?);

    tonic_build::configure()
        .protoc_arg("--experimental_allow_proto3_optional")
        .build_client(true)
        .build_server(true)
        .file_descriptor_set_path(out_dir.join("selfie_descriptor.bin"))
        .compile(&proto_files, &["proto"])?;

    Ok(())
}