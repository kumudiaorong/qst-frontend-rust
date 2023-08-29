use crate::comm::qst_comm;
use iced::widget::scrollable;
const id: &str = "s0";
const button_width: u16 = 30;
const spacing: u16 = 10;

pub struct Select {
    pub id: scrollable::Id,
    pub apps: Vec<qst_comm::Display>,
    chosen_index: usize,
}
