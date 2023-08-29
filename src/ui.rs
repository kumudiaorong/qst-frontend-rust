use std::fmt::format;

use crate::comm::{self, qst_comm};
use iced::widget::text_input::focus;
use iced::widget::{self, column, row, Column, Row};
use iced::{
    alignment, executor, theme, window, Application, Command, Element, Length, Size, Subscription,
    Theme,
};
use iced_futures::core::widget::operation::focusable::{focus_next, focus_previous};
use iced_futures::futures::channel::mpsc;
use iced_futures::subscription;
use xlog_rs::log;
enum State {
    Starting,
    Ready(comm::Comm),
}
#[derive(Debug, Clone)]
pub struct Error {
    msg: String,
}
#[derive(Debug, Clone)]
pub enum AppMessage {
    Ready(mpsc::Sender<comm::Event>),
    Connected,
    Error(String),
    Input(String),
    List(qst_comm::DisplayList),
    Push(usize),
    RunSuccess,
    Empty,
    UserEvent(iced::Event),
    Submit,
}
pub struct Flags {
    addr: String,
}
impl Flags {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }
}
pub struct App {
    input: String,
    list: qst_comm::DisplayList,
    tx: Option<mpsc::Sender<comm::Event>>,
    is_connected: bool,
    addr: String,
    choosed_index: usize,
    scroll_area: (usize, usize),
    win_size: iced::Size<u32>,
}
impl App {
    // fn send<T: Message>(&mut self, msg: T, err: &str) -> bool {
    //     match self.sender.send(msg.encode_to_vec()) {
    //         Ok(_) => true,
    //         Err(_) => {
    //             logger::warn(err);
    //             false
    //         }
    //     }
    // }
    // fn send_header(&mut self, tp: MsgType, err: &str) -> bool {
    //     self.send(MsgHeader::new(tp), err)
    // }
    // fn call(&mut self) {
    //     if let Some(path) = &self.choosed {
    //         self.send(msg::Port::new(path.clone()), "send path request failed")
    //             .then(|| {
    //                 self.send_header(MsgType::Right, "send right request failed")
    //                     .then(|| self.send_header(MsgType::Query, "send query request failed"))
    //             });
    //     }
    // }
    // fn open(&mut self) {
    //     if self.is_open {
    //         self.send_header(MsgType::Close, "send close request failed")
    //             .then(|| self.is_open = false);
    //     } else if self.choosed.is_some() {
    //         let _ = self
    //             .send_header(MsgType::Open, "send open request failed")
    //             .then(|| {
    //                 self.is_open = true;
    //                 self.call()
    //             });
    //     }
    // }
    // fn proc_ticks(&mut self) {
    //     self.available_ports = config::available_ports();
    //     if let Ok(rcev) = self.receiver.try_recv() {
    //         if let Ok(h) = MsgHeader::decode(rcev.as_slice()) {
    //             match h.tp() {
    //                 MsgType::Query => {
    //                     if let Ok(rcev) = self.receiver.recv() {
    //                         if let Ok(rl) = msg::RateList::decode(rcev.as_slice()) {
    //                             self.ratelist = rl;
    //                             self.ratelist.rates.sort_by(|l, r| l.addr.cmp(&r.addr));
    //                         }
    //                     }
    //                 }
    //                 _ => (),
    //             }
    //         }
    //     }
    // }
}
const WIN_INIT_SIZE: iced::Size<u32> = Size {
    width: 300,
    height: 245,
};

impl App {
    fn try_send(&mut self, event: comm::Event) -> Result<(), mpsc::TrySendError<comm::Event>> {
        self.tx.as_mut().unwrap().try_send(event)
    }
    fn run_app(&mut self, eh: qst_comm::ExecHint) {
        if self.is_connected {
            if let Err(e) = self.try_send(comm::Event::RunApp(eh)) {
                log::warn(format!("run app failed: {:?}", e).as_str());
            }
        }
    }
}

impl Application for App {
    type Executor = executor::Default;
    type Message = AppMessage;
    type Theme = Theme;
    type Flags = Flags;

    fn new(flags: Self::Flags) -> (Self, Command<AppMessage>) {
        (
            Self {
                tx: None,
                is_connected: false,
                input: String::new(),
                list: qst_comm::DisplayList::default(),
                addr: flags.addr,
                choosed_index: 0,
                scroll_area: (0, 0),
                win_size: WIN_INIT_SIZE,
            },
            window::resize(WIN_INIT_SIZE),
        )
    }

    fn title(&self) -> String {
        String::from("Rating")
    }

    fn update(&mut self, message: AppMessage) -> Command<AppMessage> {
        match message {
            AppMessage::Ready(tx) => {
                self.tx = Some(tx);
                if let Err(e) = self.try_send(comm::Event::Connect(self.addr.clone())) {
                    log::warn(format!("connect failed: {:?}", e).as_str());
                }
                Command::none()
            }
            AppMessage::Error(msg) => {
                println!("error: {}", msg);
                Command::none()
            }
            AppMessage::Connected => {
                self.is_connected = true;
                Command::none()
            }
            AppMessage::Empty => Command::none(),
            AppMessage::Input(input) => {
                self.input = input;
                if self.is_connected {
                    if let Err(e) = self.try_send(comm::Event::InputChanged(qst_comm::Input {
                        str: self.input.clone(),
                    })) {
                        log::warn(format!("input failed: {:?}", e).as_str());
                    }
                }
                Command::none()
            }
            AppMessage::List(list) => {
                self.scroll_area = (0, self.win_size.height as usize - 80);
                self.list = list;
                Command::none()
            }
            AppMessage::Push(idx) => {
                self.run_app(qst_comm::ExecHint {
                    name: self.list.list[idx].name.clone(),
                    file: None,
                    url: None,
                });
                Command::none()
            } // AppMessage::Tick => self.proc_ticks(),
            // AppMessage::OpenSerial => self.open(),
            // AppMessage::ReSet => {
            //     self.send_header(MsgType::Reset, "send reset request failed");
            // }
            // AppMessage::ReCheck => {
            //     self.send_header(MsgType::Right, "send right request failed");
            // }
            // AppMessage::ReQuery => {
            //     self.ratelist.clear();
            //     self.send_header(MsgType::Next, "send query request failed");
            //     self.send_header(MsgType::Query, "send query request failed");
            // }
            // AppMessage::Save => self.config.save(),
            // AppMessage::Apply => {
            //     self.send_header(MsgType::Reload, "send load request failed")
            //         .then(|| self.call());
            // }
            // AppMessage::PortSelected(path) => self.choosed = Some(path),
            // AppMessage::CfgBaudRate(rate) => self.config.baud_rate = rate,
            // AppMessage::CfgTimeout(timeout) => {
            //     self.config.timeout = timeout.parse().unwrap_or(self.config.timeout)
            // }
            // AppMessage::CfgMaxDev(max_dev) => {
            //     self.config.max_dev = max_dev.parse().unwrap_or(self.config.max_dev)
            // }
            // AppMessage::CfgTryCnt(try_cnt) => {
            //     self.config.try_cnt = try_cnt.parse().unwrap_or(self.config.try_cnt)
            // }
            AppMessage::RunSuccess => Command::none(),
            AppMessage::UserEvent(e) => match e {
                iced::Event::Keyboard(iced::keyboard::Event::KeyPressed { key_code, .. }) => {
                    match key_code {
                        iced::keyboard::KeyCode::Down => {
                            log::trace("Pressed down");
                            if self.choosed_index < self.list.list.len() {
                                self.choosed_index += 1;
                            }
                            let minscrollend = self.choosed_index * 35 - 5;
                            log::trace(format!("minscrollend: {}", minscrollend).as_str());
                            if minscrollend > self.scroll_area.1 as usize {
                                let scrolloff = minscrollend - self.scroll_area.1;
                                log::trace(
                                    format!("Scroll to down with offset {}", scrolloff).as_str(),
                                );
                                self.scroll_area = (self.scroll_area.0 + scrolloff, minscrollend);
                                let all = ((self.list.list.len() * 35)
                                    - 5
                                    - (self.scroll_area.1 - self.scroll_area.0))
                                    as f32;
                                return widget::scrollable::snap_to(
                                    widget::scrollable::Id::new("s0"),
                                    widget::scrollable::RelativeOffset {
                                        x: 0.0,
                                        y: self.scroll_area.0 as f32 / all,
                                    },
                                );
                            }
                            Command::none()
                        }
                        iced::keyboard::KeyCode::Up => {
                            log::trace("Pressed up");
                            if self.choosed_index > 1 {
                                self.choosed_index -= 1;
                            }
                            let minscrollbegin = (self.choosed_index - 1) * 35;
                            log::trace(format!("minscrollbegin: {}", minscrollbegin).as_str());
                            if minscrollbegin < self.scroll_area.0 {
                                let scrolloff = self.scroll_area.0 - minscrollbegin;
                                log::trace(
                                    format!("Scroll to up with offset {}", scrolloff).as_str(),
                                );
                                self.scroll_area = (minscrollbegin, self.scroll_area.1 - scrolloff);
                                let all = ((self.list.list.len() * 35) - 5) as f32;
                                return widget::scrollable::snap_to(
                                    widget::scrollable::Id::new("s0"),
                                    widget::scrollable::RelativeOffset {
                                        x: 0.0,
                                        y: self.scroll_area.0 as f32 / all,
                                    },
                                );
                            }

                            Command::none()
                        }
                        _ => Command::none(),
                    }
                }
                // iced::Event::Window(iced::window::Event::Resized { width, height }
                _ => Command::none(),
            },
            AppMessage::Submit => {
                self.run_app(qst_comm::ExecHint {
                    name: self.input.clone(),
                    file: None,
                    url: None,
                });
                Command::none()
            }
        }
    }
    fn subscription(&self) -> Subscription<AppMessage> {
        struct SomeSub;
        Subscription::batch([
            iced_futures::subscription::events()
                .map(AppMessage::UserEvent)
                .into(),
            subscription::channel(
                std::any::TypeId::of::<SomeSub>(),
                100,
                |mut output| async move {
                    let mut state = State::Starting;

                    loop {
                        use iced_futures::futures::sink::SinkExt;
                        match &mut state {
                            State::Starting => {
                                // Create channel
                                let (sender, receiver) = mpsc::channel(100);

                                // Send the sender back to the application
                                output.send(AppMessage::Ready(sender)).await;

                                // We are ready to receive messages
                                state = State::Ready(comm::Comm::new(receiver));
                            }
                            State::Ready(comm) => {
                                // Read next input sent from `Application`
                                if let Some(msg) = comm.next().await {
                                    output.send(msg).await;
                                }
                            }
                        }
                    }
                },
            )
            .into(),
        ])
    }
    fn view(&self) -> Element<AppMessage> {
        use iced::widget::text_input::Id;
        let input = widget::text_input("", self.input.as_str())
            .line_height(widget::text::LineHeight::Absolute(iced::Pixels(30.0)))
            .on_input(AppMessage::Input)
            .on_submit(AppMessage::Submit)
            .id(Id::new("0"));
        let list = self
            .list
            .list
            .iter()
            .enumerate()
            .map(|(i, r)| {
                widget::button(widget::text(r.name.as_str()))
                    .width(Length::Fill)
                    .height(35)
                    .on_press(AppMessage::Push(i))
                    .style(if i + 1 == self.choosed_index {
                        theme::Button::Primary
                    } else {
                        theme::Button::Secondary
                    })
                    .into()
            })
            .collect::<Vec<_>>();
        let list = widget::scrollable(Column::with_children(list).spacing(5))
            .id(widget::scrollable::Id::new("s0"));
        // .direction(
        //     widget::scrollable::Direction::Vertical(),
        //     widget::scrollable::Properties::new().
        // );
        column!(input, list).spacing(5).padding(5).into()
        // let available_list = widget::pick_list(
        //     &self.available_ports,
        //     self.choosed.clone(),
        //     AppMessage::PortSelected,
        // )
        // .placeholder("choose a port");
        // let ratesheader = add_boder(row![
        //     std_text("idx"),
        //     std_text("addr"),
        //     std_text("score"),
        //     std_text("state"),
        // ]);
        // let ratesbody = add_boder(
        //     widget::scrollable(
        //         Column::with_children(
        //             self.ratelist
        //                 .rates
        //                 .iter()
        //                 .enumerate()
        //                 .map(|(i, r)| create_row(i, r.addr, r.score, r.state()))
        //                 .collect(),
        //         )
        //         .width(Length::Fixed(320.0)),
        //     )
        //     .height(180),
        // );
        // let rates = column!(ratesheader, ratesbody)
        //     .spacing(5)
        //     .align_items(alignment::Alignment::Center);
        // let allokscores = self.ratelist.rates.iter().filter_map(|r| match r.state() {
        //     rate_list::State::Ok => Some(r.score),
        //     _ => None,
        // });
        // let allokscoreslen = allokscores.clone().count();
        // let sumscore = allokscores.clone().sum::<i32>();
        // let display: Column<'_, AppMessage> = column![
        //     add_boder(
        //         column![
        //             creat_info("sum", allokscores.clone().sum::<i32>()),
        //             creat_info("max", allokscores.clone().max().unwrap_or(-1)),
        //             creat_info("min", allokscores.clone().min().unwrap_or(-1)),
        //             creat_info("avg", sumscore / allokscoreslen.max(1) as i32),
        //         ]
        //         .spacing(5)
        //     ),
        //     row![
        //         widget::button(std_text(if self.is_open { "close" } else { "open" }))
        //             .on_press(AppMessage::OpenSerial),
        //         widget::button(std_text("reset")).on_press(AppMessage::ReSet),
        //     ]
        //     .spacing(5),
        //     row![
        //         widget::button(std_text("recheck")).on_press(AppMessage::ReCheck),
        //         widget::button(std_text("requery")).on_press(AppMessage::ReQuery),
        //     ]http://
        //     .spacing(5),
        // ]
        // .spacing(5);
        // let cfg = column![
        //     row![std_text("port"), available_list].spacing(5),
        //     row![
        //         std_text("baud rate"),
        //         widget::pick_list(
        //             &config::BAUD_RATES[..],
        //             Some(self.config.baud_rate),
        //             AppMessage::CfgBaudRate,
        //         ),
        //     ]
        //     .spacing(5),
        //     row![
        //         std_text("timeout"),
        //         widget::text_input("", self.config.timeout.to_string().as_str())
        //             .width(Length::Fixed(135.0))
        //             .on_input(AppMessage::CfgTimeout),
        //     ]
        //     .spacing(5),
        //     row![
        //         std_text("max dev"),
        //         widget::text_input("", self.config.max_dev.to_string().as_str())
        //             .width(Length::Fixed(135.0))
        //             .on_input(AppMessage::CfgMaxDev),
        //     ]
        //     .spacing(5),
        //     row![
        //         std_text("try times"),
        //         widget::text_input("", self.config.try_cnt.to_string().as_str())
        //             .width(Length::Fixed(135.0))
        //             .on_input(AppMessage::CfgTryCnt),
        //     ]
        //     .spacing(5),
        //     widget::vertical_space(15),
        //     row![
        //         widget::button(std_text("save")).on_press(AppMessage::Save),
        //         widget::button(std_text("apply")).on_press(AppMessage::Apply),
        //     ]
        //     .spacing(40),
        // ]
        // .spacing(5);
        // widget::container(row![rates, display, cfg].spacing(5))
        //     .center_x()
        //     .center_y()
        //     .width(Length::Shrink)
        //     .height(Length::Shrink)
        //     .padding([5, 5, 5, 5])
        //     .into()
    }
}

// fn add_boder<'a>(c: impl Into<Element<'a, AppMessage>>) -> Element<'a, AppMessage> {
//     widget::container(c)
//         .padding(5)
//         .style(iced::theme::Container::Custom(Box::new(Boder)))
//         .into()
// }

// fn creat_info(name: impl ToString, val: i32) -> Element<'static, AppMessage> {
//     row![std_text(name), std_text(val)].spacing(5).into()
// }

// fn std_text<'a>(t: impl ToString) -> Element<'a, AppMessage> {
//     widget::text(t)
//         .width(80)
//         .height(30)
//         .horizontal_alignment(alignment::Horizontal::Center)
//         .vertical_alignment(alignment::Vertical::Center)
//         .into()
// }
// fn create_row<'a>(
//     idx: usize,
//     addr: i32,
//     score: i32,
//     state: rate_list::State,
// ) -> Element<'a, AppMessage> {
//     row![
//         std_text(idx),
//         std_text(addr),
//         std_text(score),
//         std_text(match state {
//             rate_list::State::Ok => "Ok",
//             rate_list::State::Ready => "Ready",
//             rate_list::State::Error => "Error",
//         }),
//     ]
//     .into()
// }

// #[derive(Default)]
// struct Boder;

// impl widget::container::StyleSheet for Boder {
//     type Style = Theme;

//     fn appearance(&self, _style: &Self::Style) -> widget::container::Appearance {
//         widget::container::Appearance {
//             border_radius: 1.0,
//             border_width: 1.0,
//             border_color: iced::Color::BLACK,
//             ..Default::default()
//         }
//     }
// }
