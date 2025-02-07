fn main() {
    capnpc::CompilerCommand::new()
        .src_prefix("crates/cereal")
        .file("crates/cereal/mood.capnp")
        .output_path("crates/cereal/src")
        .run()
        .expect("cereal compiler command");
}
