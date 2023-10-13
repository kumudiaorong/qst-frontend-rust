use iced;
struct Setting {}
#[derive(Debug)]
enum Message {}
impl Setting {
    fn new() -> Self {
        Self {}
    }
    fn update(&mut self, message: Message) {}
    fn view(&self) -> iced::Element<'_, Message> {
        use iced::advanced::widget::*;
        let ele: iced::Element<'_, Message> = iced::widget::text("Setup").into();
        ele
    }
}
