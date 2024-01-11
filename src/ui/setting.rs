use iced::widget::{self, scrollable};
use iced::Command;
use xlog_rs::log;

const SELECT_ID: &str = "s0";
pub const SPACING: u16 = 5;
pub const TEXT_WIDTH: u16 = 35;

#[derive(std::fmt::Debug, Clone)]
pub struct ExtInfo {
    pub id: u32,
    pub name: String,
    pub dir: String,
    pub exec: String,
}
use iced_futures::futures::channel::mpsc as iced_mpsc;
#[derive(std::fmt::Debug, Clone)]
pub enum Message {
    PickExt(usize),
}
pub struct Flags {
    pub tx: iced_mpsc::Sender<super::ToServer>,
    pub exts: Vec<ExtInfo>,
}

pub struct Setting {
    selected_ext: usize,
    exts: Vec<ExtInfo>,
    tx: iced_mpsc::Sender<super::ToServer>,
}
impl iced::Application for Setting {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = Flags;
    type Theme = iced::Theme;
    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let setting = Self {
            selected_ext: 0,
            exts: flags.exts,
            tx: flags.tx,
        };
        (setting, Command::none())
    }
    fn update(&mut self, msg: Message) -> Command<Message> {
        match msg {
            Message::PickExt(idx) => {
                todo!()
            }
        }
    }
    fn title(&self) -> String {
        "Setting".to_string()
    }
    fn view(&self) -> iced::Element<'_, Message> {
        #[derive(Debug, Clone, PartialEq, Eq)]
        struct PickItem {
            name: String,
            idx: usize,
        }
        impl PickItem {
            fn new(name: String, idx: usize) -> Self {
                Self { name, idx }
            }
        }
        impl ToString for PickItem {
            fn to_string(&self) -> String {
                self.name.clone()
            }
        }
        let opts = self
            .exts
            .iter()
            .enumerate()
            .map(|(idx, ext)| PickItem::new(ext.name.clone(), idx))
            .collect::<Vec<_>>();
        let selected = self
            .exts
            .get(self.selected_ext)
            .map(|ext| PickItem::new(ext.name.clone(), self.selected_ext));
        let pick =
            widget::PickList::new(opts, selected, |item: PickItem| Message::PickExt(item.idx));
        widget::Row::new()
            .push(widget::Text::new("Extensions"))
            .push(pick)
            .into()
    }
}
