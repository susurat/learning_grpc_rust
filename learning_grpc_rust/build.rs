fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        // This creates a descriptor you can embed for reflection.
        .file_descriptor_set_path("helloworld_descriptor.bin")
        .compile_protos(&["proto/helloworld.proto"], &["proto"])?;
    Ok(())
}