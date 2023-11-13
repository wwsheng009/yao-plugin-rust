fn main() {

    let proto_files = &["proto/model.proto","proto/grpc_controller.proto"];
    let out_dir ="./src/yao";
    let include_dirs = &[""];

    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .out_dir(out_dir)  // you can change the generated code's location
        .compile(proto_files,include_dirs, // specify the root location to search proto dependencies
        ).unwrap_or_else(|e| panic!("protobuf compilation failed: {}", e));
    
    // recompile protobufs only if any of the proto files changes.
    for file in proto_files {
        println!("cargo:rerun-if-changed={}", file);
    }
}