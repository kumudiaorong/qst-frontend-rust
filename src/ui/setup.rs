use iced;
struct SetUp {}
#[derive(Debug)]
enum Message {}
impl iced::Sandbox for SetUp {
    type Message = Message;
    fn new() -> Self {
        Self {}
    }
    fn title(&self) -> String {
        "Qst - Setup".to_string()
    }
    fn update(&mut self, message: Message) {}
    fn view(&self) -> iced::Element<'_, Self::Message> {
        use iced::advanced::widget::*;
        let ele: iced::Element<'_, Message> = iced::widget::text("Setup").into();
        ele
    }
}
