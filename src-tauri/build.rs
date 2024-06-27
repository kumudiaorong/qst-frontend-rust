extern crate tonic_build;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(false)
        .message_attribute("DisplayList", "#[derive(serde::Serialize)]")
        .message_attribute("DisplayItem", "#[derive(serde::Serialize)]")
        // .out_dir("src/comm")
        .compile(
            &["common.proto", "daemon.proto", "extension.proto"],
            &["qst-grpc/src"],
        )?;
    tauri_build::build();
    Ok(())
}
