fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("api/protos/auth.proto").expect("no proto");
    tonic_build::compile_protos("../api/protos/user.proto").expect("no proto");
    Ok(())
}
