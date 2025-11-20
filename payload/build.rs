const OUT_DIR: &str = "generated";
const PROTO_FILE: &str = "ipc_transport.proto";

fn main() {
    // Create the folder if it doesn't exist
    if !std::path::Path::new(OUT_DIR).exists() {
        std::fs::create_dir_all(OUT_DIR).unwrap();
    }

    // Check if the protobuf file exists
    if !std::path::Path::new(PROTO_FILE).exists() {
        panic!("Protobuf file not found: {}", PROTO_FILE);
    }

    // Generate the protobuf code
    protobuf_codegen_pure::Codegen::new()
        .out_dir(OUT_DIR)
        .inputs(&[PROTO_FILE])
        .include(".")
        .run()
        .expect("Codegen failed");
}