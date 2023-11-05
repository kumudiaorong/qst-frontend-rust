#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    xlog_rs::log::init(std::io::stdout(), xlog_rs::log::Level::Trace);
    qstf::run().await
}
