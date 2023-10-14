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
#[derive(std::fmt::Debug, Clone)]
pub enum Message {
    PickExt(usize),
}
pub struct Flags {
    pub exts: Vec<ExtInfo>,
}

pub struct Setting {
    exts: Vec<ExtInfo>,
    selected_ext: usize,
}
impl Setting {
    pub fn new(flags: Flags) -> Self {
        Self {
            exts: flags.exts,
            selected_ext: 0,
        }
    }
    pub fn update(&mut self, msg: Message) -> Command<Message> {
        match msg {
            Message::PickExt(idx) => {
                todo!()
            }
        }
    }
    pub fn view(&self) -> iced::Element<'_, Message> {
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
