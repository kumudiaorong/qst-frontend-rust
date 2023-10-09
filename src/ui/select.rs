use iced::widget::{self, scrollable};
// use xlog_rs::log;

const SELECT_ID: &str = "s0";
pub const SPACING: u16 = 5;
pub const TEXT_WIDTH: u16 = 35;

#[derive(std::fmt::Debug, Clone)]
pub struct Item {
    pub obj_id: u32,
    pub name: String,
    pub arg_hint: Option<String>,
    pub icon: Option<String>,
}
#[derive(std::fmt::Debug, Clone)]
pub enum Message {
    Height(u16),
    Refresh(Vec<Item>),
    Up,
    Down,
    Push { idx: usize, obj_id: u32 },
    Scroll(f32),
}
pub type Flags = u16;

pub struct Select {
    id: scrollable::Id,
    apps: Vec<Item>,
    selected_index: usize,
    height: u16,
    scroll_start: u16,
}
impl Select {
    pub fn selected(&self) -> Option<&Item> {
        self.apps.get(self.selected_index)
    }
    fn check_scroll(&mut self) -> iced::Command<Message> {
        let mut check_need = || {
            // log::trace(format!("scrollstart: {}", self.scroll_start).as_str());
            let mut minscroll = self.selected_index as u16 * (TEXT_WIDTH + SPACING) + TEXT_WIDTH;
            // log::trace(format!("minscrollend: {}", minscroll).as_str());
            if minscroll > self.scroll_start + self.height {
                self.scroll_start = minscroll - self.height;
                return true;
            }
            minscroll = (self.selected_index as u16) * (TEXT_WIDTH + SPACING);
            // log::trace(format!("minscrollbegin: {}", minscroll).as_str());
            if minscroll < self.scroll_start {
                self.scroll_start = minscroll;
                return true;
            }
            false
        };
        if check_need() {
            let all = ((self.apps.len() * (TEXT_WIDTH + SPACING) as usize)
                - (SPACING + self.height) as usize) as f32;
            scrollable::snap_to(
                self.id.clone(),
                scrollable::RelativeOffset {
                    x: 0.0,
                    y: self.scroll_start as f32 / all,
                },
            )
        } else {
            iced::Command::none()
        }
    }
    pub fn new(flags: Flags) -> (Self, iced::Command<Message>) {
        (
            Self {
                id: scrollable::Id::new(SELECT_ID),
                apps: vec![],
                selected_index: 0,
                height: flags,
                scroll_start: 0,
            },
            iced::Command::none(),
        )
    }
    pub fn update(&mut self, msg: Message) -> iced::Command<Message> {
        match msg {
            Message::Height(h) => {
                self.height = h;
                self.check_scroll()
            }
            Message::Refresh(apps) => {
                self.apps = apps;
                self.selected_index = 0;
                scrollable::snap_to(self.id.clone(), scrollable::RelativeOffset::START)
            }
            Message::Up => match self.selected_index {
                0 => iced::Command::none(),
                _ => {
                    self.selected_index -= 1;
                    self.check_scroll()
                }
            },
            Message::Down => {
                if self.selected_index == self.apps.len() - 1 {
                    iced::Command::none()
                } else {
                    self.selected_index += 1;
                    self.check_scroll()
                }
            }
            Message::Push { idx, .. } => {
                self.selected_index = idx;
                self.check_scroll()
            }
            Message::Scroll(y) => {
                self.scroll_start = y as u16;
                iced::Command::none()
            }
        }
    }
    pub fn view(&self) -> iced::Element<'_, Message> {
        // let list = ;
        widget::scrollable(
            widget::Column::with_children(
                self.apps
                    .iter()
                    .enumerate()
                    .map(|(idx, item)| {
                        // log::warn(format!("button: {}", r.name).as_str());
                        widget::button(widget::text(item.name.as_str()).width(iced::Length::Fill))
                            .height(TEXT_WIDTH)
                            .width(iced::Length::Fill)
                            .style(if idx == self.selected_index {
                                iced::theme::Button::Primary
                            } else {
                                iced::theme::Button::Secondary
                            })
                            .on_press(Message::Push {
                                idx,
                                obj_id: item.obj_id,
                            })
                            .into()
                    })
                    .collect::<Vec<_>>(),
            )
            .spacing(SPACING),
        )
        .id(self.id.clone())
        .on_scroll(|v| {
            // log::debug(format!("scroll: {:#?}", v).as_str());
            Message::Scroll(v.absolute_offset().y)
        })
        .into()
    }
}
