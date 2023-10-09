fn main() -> iced::Result {
    xlog_rs::log::init(std::io::stdout(), xlog_rs::log::Level::Trace);
    qstf::run()
}
