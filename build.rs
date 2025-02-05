fn main() {
    capnpc::CompilerCommand::new()
        .src_prefix("crates/cereal")
        .file("crates/cereal/openai_request.capnp")
        .output_path("crates/cereal/src")
        .run()
        .expect("cereal compiler command");
}
