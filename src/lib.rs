mod connect;
mod flag;
mod rpc;
mod ui;
mod utils;

use iced::Application;

use tonic::transport::Endpoint;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    use clap::Parser;
    let args = flag::Args::parse();
    args.uri.parse::<Endpoint>().unwrap();
    // let c = Cell::new(receiver);
    let settings = iced::Settings::with_flags(ui::Flags::new(Box::new(move || {
        connect::connect(args.uri.parse::<Endpoint>().unwrap())
    })));
    ui::App::run(settings).map_err(|e| e.into())
}
