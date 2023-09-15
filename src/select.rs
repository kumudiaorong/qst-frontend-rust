use iced::widget::{scrollable, Column};
use iced::{theme, widget, Length};
use xlog_rs::log;
const SELECT_ID: &str = "s0";
pub const SPACING: u16 = 5;
pub const TEXT_WIDTH: u16 = 35;
pub struct AppInfoFlags(u32);
impl AppInfoFlags {
    const HAS_ARG_FILE: Self = Self(0b0000_0001);
    const HAS_ARG_FILES: Self = Self(0b0000_0010);
    const HAS_ARG_URL: Self = Self(0b0000_0100);
    const HAS_ARG_URLS: Self = Self(0b0000_1000);
}
impl From<u32> for AppInfoFlags {
    fn from(v: u32) -> Self {
        Self(v)
    }
}
impl std::ops::BitOr for AppInfoFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}
impl std::ops::BitOrAssign for AppInfoFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}
impl std::ops::BitAnd for AppInfoFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}
impl std::ops::BitAndAssign for AppInfoFlags {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}
impl std::ops::Not for AppInfoFlags {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}
impl std::ops::BitXor for AppInfoFlags {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

pub struct AppInfo {
    pub name: String,
    pub flags: AppInfoFlags,
    pub icon: Option<String>,
}
pub enum Message {
    Height(u16),
    Up,
    Down,
    AppInfo(Vec<AppInfo>),
}
pub struct Select<Message> {
    pub id: scrollable::Id,
    pub apps: Vec<AppInfo>,
    selected_index: usize,
    height: u16,
    scroll_start: u16,
    on_push: Option<Box<dyn Fn(String) -> Message>>,
}
impl<Message> Select<Message> {
    pub fn new() -> Self {
        Self {
            id: scrollable::Id::new(SELECT_ID),
            apps: vec![],
            selected_index: 0,
            height: 0,
            scroll_start: 0,
            on_push: None,
        }
    }
    pub fn with_height(height: u16) -> Self {
        Self {
            id: scrollable::Id::new(SELECT_ID),
            apps: vec![],
            selected_index: 0,
            height,
            scroll_start: 0,
            on_push: None,
        }
    }
    pub fn on_push<F>(mut self, f: F) -> Self
    where
        F: Fn(String) -> Message + 'static,
    {
        self.on_push = Some(Box::new(f));
        self
    }
    // pub fn app(&self, index: usize) -> Option<&AppInfo> {
    //     self.apps.get(index)
    // }
    // pub fn has_selected(&self) -> bool {
    //     self.selected_index != 0
    // }

    pub fn selected(&self) -> Option<&AppInfo> {
        if self.selected_index == 0 {
            None
        } else {
            self.apps.get((self.selected_index - 1) as usize)
        }
    }
    pub fn view(&self) -> iced::Element<Message>
    where
        Message: Clone,
    {
        let list = self
            .apps
            .iter()
            .enumerate()
            .map(|(i, r)| {
                // log::warn(format!("button: {}", r.name).as_str());
                let btn = widget::button(widget::text(r.name.as_str()).width(Length::Fill))
                    .height(TEXT_WIDTH)
                    .width(Length::Fill)
                    .style(if i + 1 == self.selected_index {
                        theme::Button::Primary
                    } else {
                        theme::Button::Secondary
                    });
                // self.on_push(r.name.clone())
                if let Some(ref f) = self.on_push {
                    btn.on_press(f(r.name.clone())).into()
                } else {
                    btn.into()
                }
            })
            .collect::<Vec<_>>();
        widget::scrollable(Column::with_children(list).spacing(SPACING))
            .id(self.id.clone())
            .into()
    }
    // pub fn selected_index(&self) -> usize {
    //     self.selected_index
    // }
}
impl<Message: 'static> Select<Message> {
    fn check_scroll(&mut self) -> iced::Command<Message> {
        let mut check_need = || {
            log::trace(format!("scrollstart: {}", self.scroll_start).as_str());
            let mut minscroll = self.selected_index as u16 * (TEXT_WIDTH + SPACING) - SPACING;
            log::trace(format!("minscrollend: {}", minscroll).as_str());
            if minscroll > self.scroll_start + self.height {
                self.scroll_start = minscroll - self.height;
                return true;
            }
            minscroll = (self.selected_index as u16 - 1) * (TEXT_WIDTH + SPACING);
            log::trace(format!("minscrollbegin: {}", minscroll).as_str());
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
    fn down(&mut self) -> iced::Command<Message> {
        log::trace("Pressed down");
        if self.selected_index < self.apps.len() {
            self.selected_index += 1;
            self.check_scroll()
        } else {
            iced::Command::none()
        }
    }
    fn up(&mut self) -> iced::Command<Message> {
        log::trace("Pressed up");
        if self.selected_index > 1 {
            self.selected_index -= 1;
            self.check_scroll()
        } else {
            iced::Command::none()
        }
    }

    pub fn update(&mut self, msg: super::select::Message) -> iced::Command<Message> {
        match msg {
            super::select::Message::Height(h) => {
                self.height = h;
                self.check_scroll()
            }
            super::select::Message::Up => self.up(),
            super::select::Message::Down => self.down(),
            super::select::Message::AppInfo(apps) => {
                self.apps = apps;
                self.selected_index = 0;
                scrollable::snap_to(self.id.clone(), scrollable::RelativeOffset::START)
            }
        }
    }
}
