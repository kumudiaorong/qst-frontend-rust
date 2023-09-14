use crate::comm::qst_comm;
use crate::ui;
use crate::ui::AppMessage;
use crate::ui::FromUiMessage;
use iced::widget::{scrollable, Column};
use iced::{theme, widget, Length};
use xlog_rs::log;
const SELECT_ID: &str = "s0";

pub struct Select {
    pub id: scrollable::Id,
    pub apps: Vec<qst_comm::Display>,
    selected_index: usize,
    height: u16,
    scroll_start: u16,
}
impl Select {
    pub fn new() -> Self {
        Self {
            id: scrollable::Id::new(SELECT_ID),
            apps: vec![],
            selected_index: 0,
            height: 0,
            scroll_start: 0,
        }
    }
    pub fn with_height(height: u16) -> Self {
        Self {
            id: scrollable::Id::new(SELECT_ID),
            apps: vec![],
            selected_index: 0,
            height,
            scroll_start: 0,
        }
    }
    pub fn update(&mut self, apps: Vec<qst_comm::Display>) -> iced::Command<crate::ui::AppMessage> {
        self.apps = apps;
        self.selected_index = 0;
        scrollable::snap_to(self.id.clone(), scrollable::RelativeOffset::START)
    }
    pub fn app(&self, index: usize) -> Option<&qst_comm::Display> {
        self.apps.get(index)
    }
    pub fn has_selected(&self) -> bool {
        self.selected_index != 0
    }
    fn check_scroll(&mut self) -> iced::Command<crate::ui::AppMessage> {
        let mut check_need = || {
            log::trace(format!("scrollstart: {}", self.scroll_start).as_str());
            let mut minscroll =
                self.selected_index as u16 * (ui::TEXT_WIDTH + ui::SPACING) - ui::SPACING;
            log::trace(format!("minscrollend: {}", minscroll).as_str());
            if minscroll > self.scroll_start + self.height {
                self.scroll_start = minscroll - self.height;
                return true;
            }
            minscroll = (self.selected_index as u16 - 1) * (ui::TEXT_WIDTH + ui::SPACING);
            log::trace(format!("minscrollbegin: {}", minscroll).as_str());
            if minscroll < self.scroll_start {
                self.scroll_start = minscroll;
                return true;
            }
            false
        };
        if check_need() {
            let all = ((self.apps.len() * (ui::TEXT_WIDTH + ui::SPACING) as usize)
                - (ui::SPACING + self.height) as usize) as f32;
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
    pub fn down(&mut self) -> iced::Command<crate::ui::AppMessage> {
        log::trace("Pressed down");
        if self.selected_index < self.apps.len() {
            self.selected_index += 1;
            self.check_scroll()
        } else {
            iced::Command::none()
        }
        // self.scroll_start += 5;
        // let all = ((self.apps.len() * (ui::TEXT_WIDTH + ui::SPACING) as usize)
        //     - (ui::SPACING + self.height) as usize) as f32;
        // scrollable::snap_to(
        //     self.id.clone(),
        //     scrollable::RelativeOffset {
        //         x: 0.0,
        //         y: self.scroll_start as f32 / all,
        //     },
        // )
    }
    // iced::keyboard::KeyCode::Up => {
    //     log::trace("Pressed up");
    //     if self.choosed_index > 1 {
    //         self.choosed_index -= 1;
    //         let minscrollbegin = (self.choosed_index - 1) * 35;
    //         log::trace(format!("minscrollbegin: {}", minscrollbegin).as_str());
    //         if minscrollbegin < self.scroll_area.0 {
    //             let scrolloff = self.scroll_area.0 - minscrollbegin;
    //             log::trace(
    //                 format!("Scroll to up with offset {}", scrolloff).as_str(),
    //             );
    //             self.scroll_area =
    //                 (minscrollbegin, self.scroll_area.1 - scrolloff);
    //             let all = ((self.list.list.len() * 35)
    //                 - 5
    //                 - (self.scroll_area.1 - self.scroll_area.0))
    //                 as f32;
    //             return widget::scrollable::snap_to(
    //                 widget::scrollable::Id::new("s0"),
    //                 widget::scrollable::RelativeOffset {
    //                     x: 0.0,
    //                     y: self.scroll_area.0 as f32 / all,
    //                 },
    //             );
    //         }
    //     }
    //     Command::none()
    // }
    pub fn up(&mut self) -> iced::Command<crate::ui::AppMessage> {
        log::trace("Pressed up");
        if self.selected_index > 1 {
            self.selected_index -= 1;
            self.check_scroll()
        } else {
            iced::Command::none()
        }
        // if self.selected_index > 1 {
        //     self.selected_index -= 1;
        //     let minscrollbegin = (self.selected_index as u16 - 1) * (ui::TEXT_WIDTH + ui::SPACING);
        //     log::trace(format!("minscrollbegin: {}", minscrollbegin).as_str());
        //     if minscrollbegin < self.scroll_start {
        //         self.scroll_start = minscrollbegin;
        //         let all = ((self.apps.len() * (ui::TEXT_WIDTH + ui::SPACING) as usize)
        //             - (ui::SPACING + self.height) as usize) as f32;
        //         return scrollable::snap_to(
        //             self.id.clone(),
        //             scrollable::RelativeOffset {
        //                 x: 0.0,
        //                 y: self.scroll_start as f32 / all,
        //             },
        //         );
        //     }
        // }
        // iced::Command::none()
        // self.scroll_start -= 5;
        // let all = ((self.apps.len() * (ui::TEXT_WIDTH + ui::SPACING) as usize)
        //     - (ui::SPACING + self.height) as usize) as f32;
        // scrollable::snap_to(
        //     self.id.clone(),
        //     scrollable::RelativeOffset {
        //         x: 0.0,
        //         y: self.scroll_start as f32 / all,
        //     },
        // )
    }
    pub fn selected(&self) -> Option<&qst_comm::Display> {
        if self.selected_index == 0 {
            None
        } else {
            self.apps.get((self.selected_index - 1) as usize)
        }
    }
    pub fn selected_index(&self) -> usize {
        self.selected_index
    }
    pub fn view(&self) -> iced::Element<crate::ui::AppMessage> {
        let list = self
            .apps
            .iter()
            .enumerate()
            .map(|(i, r)| {
                // log::warn(format!("button: {}", r.name).as_str());
                widget::button(widget::text(r.name.as_str()).width(Length::Fill))
                    .height(ui::TEXT_WIDTH)
                    .width(Length::Fill)
                    .on_press(AppMessage::FromUi(FromUiMessage::Push(i)))
                    .style(if i + 1 == self.selected_index {
                        theme::Button::Primary
                    } else {
                        theme::Button::Secondary
                    })
                    .into()
            })
            .collect::<Vec<_>>();
        widget::scrollable(Column::with_children(list).spacing(ui::SPACING))
            .id(self.id.clone())
            .into()
    }
}
