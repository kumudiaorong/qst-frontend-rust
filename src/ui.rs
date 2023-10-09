mod flags;
mod select;
pub use flags::Flags;
use iced::{
    widget::{self, column, text_input},
    window, Command, Size, Subscription,
};
use iced_futures::futures::channel::mpsc as iced_mpsc;
use tokio::time::{sleep as async_sleep, Duration};
use xlog_rs::log;

pub const SPACING: u16 = 5;
pub const PADDING: u16 = 5;
pub const TEXT_WIDTH: u16 = 35;

pub use select::Item;
fn convert_select_msg(msg: select::Message) -> Message {
    Message::FromUi(FromUi::Select(msg))
}

#[derive(Debug, Clone)]
pub struct Error {
    msg: String,
}
impl Error {
    pub fn from(msg: impl ToString) -> Self {
        Self {
            msg: msg.to_string(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum FromUi {
    InputChanged(String),
    Select(select::Message),
    Submit,
}
#[derive(Debug, Clone)]
pub enum FromServer {
    ConnectResult,
    SearchResult(Vec<select::Item>),
    SubmitResult,
    // FillResult(String),
}

#[derive(Debug, Clone)]
pub enum ToServer {
    Connect(tonic::transport::Endpoint),
    Search {
        prompt: String,
        content: String,
    },
    Submit {
        prompt: String,
        obj_id: u32,
        hint: Option<String>,
    },
    // Fill {
    //     prompt: String,
    //     obj_id: u32,
    // },
}

#[derive(Debug, Clone)]
pub enum Message {
    Start(iced_mpsc::Sender<ToServer>),
    FromServer(Result<FromServer, Error>),
    ToServer(ToServer),
    FromUi(FromUi),
    UserEvent(iced::Event),
}

#[derive(Debug, Clone, PartialEq)]
enum Runstate {
    Select,
    AddArgs {
        placeholder: String,
        input: String,
        obj_id: u32,
    },
}

fn extract_prompt(s: &str) -> Option<(String, String)> {
    if s.starts_with('|') {
        let mut iter = s[1..].splitn(2, '|');
        if let Some(prompt) = iter.next() {
            if let Some(content) = iter.next() {
                return Some((prompt.to_string(), content.to_string()));
            }
        }
        return None;
    }
    return Some((String::new(), s.to_string()));
}
pub struct App {
    input: String,
    tx: Option<iced_mpsc::Sender<ToServer>>,
    is_connected: bool,
    select: select::Select,
    win_size: Size<u32>,
    placeholder: String,
    runstate: Runstate,
    prompt: String,
    flags: flags::Flags,
}

const WIN_INIT_SIZE: Size<u32> = Size {
    width: 300,
    height: 245,
};

impl App {
    fn try_send(&mut self, req: ToServer) -> Result<(), iced_mpsc::TrySendError<ToServer>> {
        self.tx.as_mut().unwrap().try_send(req)
    }
    fn select(&mut self) {
        if let Some(item) = self.select.selected() {
            self.runstate = Runstate::AddArgs {
                placeholder: "[prompt]content".to_string(),
                input: std::mem::replace(&mut self.input, String::new()),
                obj_id: item.obj_id,
            };
            self.placeholder = item.arg_hint.clone().unwrap_or("none args".to_string());
        }
    }
    fn submit(
        &mut self,
        prompt: impl ToString,
        obj_id: u32,
        hint: Option<String>,
    ) -> Result<(), iced_mpsc::TrySendError<ToServer>> {
        self.try_send(ToServer::Submit {
            prompt: prompt.to_string(),
            obj_id,
            hint,
        })
    }
    fn try_reload(&mut self) {
        if let Runstate::AddArgs {
            placeholder,
            input,
            obj_id: _,
        } = &mut self.runstate
        {
            std::mem::swap(&mut self.input, input);
            std::mem::swap(&mut self.placeholder, placeholder);
            self.runstate = Runstate::Select;
        }
    }
}

impl iced::Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = flags::Flags;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                tx: None,
                is_connected: false,
                input: String::new(),
                select: select::Select::new(
                    WIN_INIT_SIZE.height as u16
                        - (TEXT_WIDTH + SPACING * 2)
                        - (PADDING * 2)
                        - SPACING,
                )
                .0,
                win_size: WIN_INIT_SIZE,
                placeholder: "app name".to_string(),
                runstate: Runstate::Select,
                prompt: String::new(),
                flags,
            },
            window::resize(WIN_INIT_SIZE),
        )
    }

    fn title(&self) -> String {
        String::from("Qst")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Start(tx) => {
                self.tx = Some(tx);
                let ed = self.flags.endpoint.clone();
                Command::perform(async {}, move |_| {
                    Self::Message::ToServer(ToServer::Connect(ed))
                })
            }
            Message::ToServer(req) => {
                if let Err(e) = self.try_send(req.clone()) {
                    log::warn(format!("input failed: {:?}", e).as_str());
                    Command::perform(
                        async {
                            async_sleep(Duration::from_millis(200)).await;
                        },
                        move |_| Self::Message::ToServer(req),
                    )
                } else {
                    Command::none()
                }
            }
            Message::FromServer(result) => match result {
                Ok(msg) => match msg {
                    FromServer::ConnectResult => {
                        self.is_connected = true;
                        widget::text_input::focus(text_input::Id::new("i0"))
                    }
                    FromServer::SearchResult(list) => self
                        .select
                        .update(select::Message::Refresh(list))
                        .map(convert_select_msg),
                    // FromServer::FillResult(content) => {
                    //     self.input = content;
                    //     Command::none()
                    // }
                    FromServer::SubmitResult => Command::none(),
                },
                Err(_) => Command::none(),
            },
            Message::FromUi(umsg) => match umsg {
                FromUi::InputChanged(input) => match input.is_empty() {
                    true => {
                        self.input.clear();
                        Command::perform(async {}, move |_| {
                            Self::Message::FromServer(Ok(FromServer::SearchResult(vec![])))
                        })
                    }
                    false => {
                        self.input = input;
                        match extract_prompt(self.input.as_str()) {
                            Some((prompt, content))
                                if self.runstate == Runstate::Select || prompt != self.prompt =>
                            {
                                self.prompt = prompt.clone();
                                if self.is_connected {
                                    if let Err(e) =
                                        self.try_send(ToServer::Search { prompt, content })
                                    {
                                        log::warn(format!("search send failed: {:?}", e).as_str());
                                    }
                                }
                            }
                            _ => {}
                        }
                        Command::none()
                    }
                },
                FromUi::Select(smsg) => {
                    let cmd = self.select.update(smsg.clone()).map(convert_select_msg);
                    match smsg {
                        select::Message::Push { obj_id, .. } => match &self.runstate {
                            Runstate::Select => {
                                self.select();
                            }
                            Runstate::AddArgs {
                                placeholder: _,
                                input: _,
                                obj_id: sid,
                            } => {
                                // let item = self.select.selected().unwrap();
                                if obj_id == *sid {
                                    if let Err(e) = self.submit(
                                        self.prompt.clone(),
                                        obj_id,
                                        Some(self.input.clone()),
                                    ) {
                                        log::warn(format!("input failed: {:?}", e).as_str());
                                    }
                                } else {
                                    self.select();
                                }
                            }
                        },
                        _ => {}
                    }
                    Command::batch([cmd, text_input::focus(text_input::Id::new("i0"))])
                }
                FromUi::Submit => {
                    match self.runstate {
                        Runstate::Select => {
                            self.select();
                        }
                        Runstate::AddArgs {
                            placeholder: _,
                            input: _,
                            obj_id,
                        } => {
                            if let Err(e) =
                                self.submit(self.prompt.clone(), obj_id, Some(self.input.clone()))
                            {
                                log::warn(format!("input failed: {:?}", e).as_str());
                            }
                        }
                    }
                    Command::none()
                }
            },
            Self::Message::UserEvent(e) => match e {
                iced::Event::Window(iced::window::Event::Resized { width, height }) => {
                    self.win_size = Size { width, height };
                    self.select
                        .update(select::Message::Height(
                            (height as u16)
                                .checked_sub((TEXT_WIDTH + SPACING * 2) + (PADDING * 2) + SPACING)
                                .unwrap_or(0),
                        ))
                        .map(convert_select_msg)
                }
                iced::Event::Keyboard(iced::keyboard::Event::KeyPressed { key_code, .. }) => {
                    match key_code {
                        iced::keyboard::KeyCode::Up => {
                            self.try_reload();
                            self.select
                                .update(select::Message::Up)
                                .map(convert_select_msg)
                        }
                        iced::keyboard::KeyCode::Down => {
                            self.try_reload();
                            self.select
                                .update(select::Message::Down)
                                .map(convert_select_msg)
                        }
                        _ => Command::none(),
                    }
                }
                _ => Command::none(),
            },
        }
    }
    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::batch([
            iced_futures::subscription::events()
                .map(Self::Message::UserEvent)
                .into(),
            (self.flags.recipe)(),
        ])
    }
    fn view(&self) -> iced::Element<Self::Message> {
        let input = widget::text_input(&self.placeholder, self.input.as_str())
            .line_height(widget::text::LineHeight::Absolute(iced::Pixels(
                TEXT_WIDTH as f32,
            )))
            .padding(PADDING)
            .on_input(|input| Message::FromUi(FromUi::InputChanged(input)))
            .width(iced::Length::Fill)
            .on_submit(Message::FromUi(FromUi::Submit))
            .id(text_input::Id::new("i0"));
        // .direction(
        //     widget::scrollable::Direction::Vertical(),
        //     widget::scrollable::Properties::new().
        // );
        column!(input, self.select.view().map(convert_select_msg))
            .spacing(SPACING)
            .padding(PADDING)
            .into()
        // let available_list = widget::pick_list(
        //     &self.available_ports,
        //     self.choosed.clone(),
        //     Self::Message::PortSelected,
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
        // let display: Column<'_, Self::Message> = column![
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
        //             .on_press(Self::Message::OpenSerial),
        //         widget::button(std_text("reset")).on_press(Self::Message::ReSet),
        //     ]
        //     .spacing(5),
        //     row![
        //         widget::button(std_text("recheck")).on_press(Self::Message::ReCheck),
        //         widget::button(std_text("requery")).on_press(Self::Message::ReQuery),
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
        //             Self::Message::CfgBaudRate,
        //         ),
        //     ]
        //     .spacing(5),
        //     row![
        //         std_text("timeout"),
        //         widget::text_input("", self.config.timeout.to_string().as_str())
        //             .width(Length::Fixed(135.0))
        //             .on_input(Self::Message::CfgTimeout),
        //     ]
        //     .spacing(5),
        //     row![
        //         std_text("max dev"),
        //         widget::text_input("", self.config.max_dev.to_string().as_str())
        //             .width(Length::Fixed(135.0))
        //             .on_input(Self::Message::CfgMaxDev),
        //     ]
        //     .spacing(5),
        //     row![
        //         std_text("try times"),
        //         widget::text_input("", self.config.try_cnt.to_string().as_str())
        //             .width(Length::Fixed(135.0))
        //             .on_input(Self::Message::CfgTryCnt),
        //     ]
        //     .spacing(5),
        //     widget::vertical_space(15),
        //     row![
        //         widget::button(std_text("save")).on_press(Self::Message::Save),
        //         widget::button(std_text("apply")).on_press(Self::Message::Apply),
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

// fn add_boder<'a>(c: impl Into<Element<'a, Self::Message>>) -> Element<'a, Self::Message> {
//     widget::container(c)
//         .padding(5)
//         .style(iced::theme::Container::Custom(Box::new(Boder)))
//         .into()
// }

// fn creat_info(name: impl ToString, val: i32) -> Element<'static, Self::Message> {
//     row![std_text(name), std_text(val)].spacing(5).into()
// }

// fn std_text<'a>(t: impl ToString) -> Element<'a, Self::Message> {
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
// ) -> Element<'a, Self::Message> {
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
