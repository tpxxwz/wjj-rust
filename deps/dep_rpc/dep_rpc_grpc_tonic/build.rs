fn main() -> Result<(), Box<dyn std::error::Error>> {
    // .build_server(true)
    // .build_client(true)
    tonic_prost_build::configure()
        .compile_protos(
            &["proto/demo.proto"],
            &["proto", "proto/third_party/google/protobuf"], // 搜索 proto 依赖的路径
        )?;
    Ok(())
}
