fn main() {
    tonic_build::configure()
        .build_server(true)
        .file_descriptor_set_path("proto/artie_engine.bin") 
        .compile(&["proto/artie_distances.proto"], &["proto"])
        .unwrap();
}
