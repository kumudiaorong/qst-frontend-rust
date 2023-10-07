extern crate tonic_build;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(false)
        // .out_dir("src/comm")
        .compile(
            &["defs.proto", "daemon.proto", "ext.proto"],
            &["qst-proto/src"],
        )?;
    Ok(())
}
