use qstf::ui;

use iced::Application;

fn main() -> iced::Result {
    xlog_rs::log::init(std::io::stdout());
    xlog_rs::log::set_level(xlog_rs::log::Level::Trace);
    let settings = iced::Settings::with_flags(ui::Flags::new(std::env::args().collect()));
    ui::App::run(settings)
    // let mut c = Comm::new("127.0.0.1:50051".to_string()).await;
    // loop {
    //     c.input("hello").await?;
    //     thread::sleep(std::time::Duration::from_secs(1));
    // }
    // let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    // let request = tonic::Request::new(HelloRequest {
    //     name: "Tonic".into(),
    // });

    // let response = client.say_hello(request).await?;

    // println!("RESPONSE={:?}", response);
    // Ok(())
}
