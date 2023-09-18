use crate::comm;
use crate::select;
use comm::Request as RpcRequest;
use comm::Response as RpcResponse;
use iced::{
    widget::{self, column, text_input},
    window, Command, Size, Subscription,
};
use iced_futures::futures::channel::mpsc as iced_mpsc;
use std::time::Duration;
use tokio::time::sleep as async_sleep;
use tonic::transport;
use xlog_rs::log;
pub const SPACING: u16 = 5;
pub const PADDING: u16 = 5;
pub const TEXT_WIDTH: u16 = 35;
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
pub enum ConnectMessage {
    S2uReady(iced_mpsc::Sender<RpcRequest>),
    U2cTrySendConnect,
    C2uConnectFailed(Error),
}

#[derive(Debug, Clone)]
pub enum FromUiMessage {
    InputChanged(String),
    Push(usize),
    Submit,
}

#[derive(Debug, Clone)]
pub enum AppMessage {
    RpcStart(iced_mpsc::Sender<RpcRequest>),
    FromRpc(RpcResponse),
    ToRpc(RpcRequest),
    FromUi(FromUiMessage),
    Error(String),
    UserEvent(iced::Event),
}
pub struct Flags {
    endpoint: transport::Endpoint,
}
fn show_help() {
    println!("Usage: qst [options]");
    println!("Options:");
    println!("  --uri <uri>    set uri");
    println!("  --help         show help");
}
impl Flags {
    pub fn new(args: Vec<String>) -> Self {
        for (i, arg) in args.iter().enumerate() {
            match arg.as_str() {
                "--help" => {
                    show_help();
                    std::process::exit(0);
                }
                "--uri" => {
                    if i + 1 < args.len() {
                        match transport::Channel::from_shared(args[i + 1].clone()) {
                            Err(e) => {
                                println!("invalid uri: {}", e);
                                show_help();
                                std::process::exit(1);
                            }
                            Ok(c) => return Self { endpoint: c },
                        }
                    }
                }
                _ => {}
            }
        }
        println!("invalid args");
        show_help();
        std::process::exit(1);
    }
}
enum Runstate {
    Select,
    AddArgs(String), //input
}
pub struct App {
    input: String,
    tx: Option<iced_mpsc::Sender<RpcRequest>>,
    is_connected: bool,
    endpoint: transport::Endpoint,
    select: select::Select<AppMessage>,
    win_size: Size<u32>,
    placeholder: String,
    runstate: Runstate,
}

const WIN_INIT_SIZE: Size<u32> = Size {
    width: 300,
    height: 245,
};

impl App {
    fn try_send(&mut self, req: RpcRequest) -> Result<(), iced_mpsc::TrySendError<RpcRequest>> {
        self.tx.as_mut().unwrap().try_send(req)
    }
    fn run_app(&mut self) {
        log::trace("run app");
        if self.is_connected {
            if let Err(e) = self.try_send(RpcRequest::RunApp(comm::ExecHint {
                idx: self.select.selected_index as u32,
                args: if !self.input.is_empty() {
                    Some(self.input.clone())
                } else {
                    None
                },
            })) {
                log::warn(format!("run app failed: {:?}", e).as_str());
            }
        }
    }
}

impl iced::Application for App {
    type Executor = iced::executor::Default;
    type Message = AppMessage;
    type Theme = iced::Theme;
    type Flags = Flags;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        log::trace(format!("uri: {:#?}", flags.endpoint).as_str());
        (
            Self {
                tx: None,
                is_connected: false,
                input: String::new(),
                endpoint: flags.endpoint,
                select: select::Select::with_height(
                    WIN_INIT_SIZE.height as u16
                        - (TEXT_WIDTH + SPACING * 2)
                        - (PADDING * 2)
                        - SPACING,
                )
                .on_push(|idx| AppMessage::FromUi(FromUiMessage::Push(idx))),
                win_size: WIN_INIT_SIZE,
                placeholder: "app name".to_string(),
                runstate: Runstate::Select,
            },
            window::resize(WIN_INIT_SIZE),
        )
    }

    fn title(&self) -> String {
        String::from("Qst")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            AppMessage::RpcStart(tx) => {
                self.tx = Some(tx);
                let ed = self.endpoint.clone();
                Command::perform(async {}, move |_| {
                    Self::Message::ToRpc(RpcRequest::Connect(ed))
                })
            }
            AppMessage::ToRpc(req) => {
                if let Err(e) = self.try_send(req.clone()) {
                    log::warn(format!("input failed: {:?}", e).as_str());
                    Command::perform(
                        async {
                            async_sleep(Duration::from_millis(500)).await;
                        },
                        move |_| Self::Message::ToRpc(req),
                    )
                } else {
                    Command::none()
                }
            }
            AppMessage::FromRpc(msg) => match msg {
                RpcResponse::Connected => {
                    self.is_connected = true;
                    widget::text_input::focus(text_input::Id::new("i0"))
                }
                RpcResponse::ConnectFailed(e) => {
                    log::warn(format!("connect failed: {:?}", e).as_str());
                    let ed = self.endpoint.clone();
                    Command::perform(
                        async {
                            async_sleep(Duration::from_millis(500)).await;
                        },
                        move |_| Self::Message::ToRpc(RpcRequest::Connect(ed)),
                    )
                }
                RpcResponse::RunSuccess => {
                    log::trace("run success");
                    window::close()
                }
                RpcResponse::SearchResult(list) => self.select.update(select::Message::AppInfo(
                    list.list
                        .into_iter()
                        .map(|d| select::AppInfo {
                            name: d.name,
                            arg_hint: d.arg_hint,
                            icon: d.icon,
                        })
                        .collect(),
                )),
            },
            AppMessage::FromUi(umsg) => match umsg {
                FromUiMessage::InputChanged(input) => {
                    self.input = input;
                    if matches!(self.runstate, Runstate::Select) {
                        if self.input.is_empty() {
                            return Command::perform(async move {}, |_| {
                                Self::Message::FromRpc(RpcResponse::SearchResult(
                                    comm::DisplayList::default(),
                                ))
                            });
                        } else if self.is_connected {
                            if let Err(e) = self.try_send(RpcRequest::Search(self.input.clone())) {
                                log::warn(format!("input failed: {:?}", e).as_str());
                            }
                        }
                    }
                    Command::none()
                }
                FromUiMessage::Push(idx) => {
                    log::trace(format!("push: {}", idx).as_str());

                    match self.runstate {
                        Runstate::Select => {
                            self.select.selected_index = idx;
                            if let Some(app) = self.select.selected() {
                                self.runstate = Runstate::AddArgs(std::mem::replace(
                                    &mut self.input,
                                    String::new(),
                                ));
                                self.placeholder =
                                    app.arg_hint.clone().unwrap_or("none args".to_string());
                            }
                        }
                        Runstate::AddArgs(_) => {
                            if self.select.selected_index == idx {
                                self.run_app();
                            } else {
                                self.select.selected_index = idx;
                                self.placeholder = self
                                    .select
                                    .selected()
                                    .unwrap()
                                    .arg_hint
                                    .clone()
                                    .unwrap_or("".to_string());
                            }
                            // self.runstate = Runstate::Select;
                            // self.placeholder = "app name".to_string();

                            // todo!("trans args")
                        }
                    }
                    text_input::focus(text_input::Id::new("i0"))
                }
                FromUiMessage::Submit => {
                    match self.runstate {
                        Runstate::Select => {
                            if let Some(app) = self.select.selected() {
                                self.runstate = Runstate::AddArgs(std::mem::replace(
                                    &mut self.input,
                                    String::new(),
                                ));
                                self.placeholder =
                                    app.arg_hint.clone().unwrap_or("none args".to_string());
                            }
                        }
                        Runstate::AddArgs(_) => {
                            self.run_app();
                        }
                    }
                    Command::none()
                }
            },
            Self::Message::Error(msg) => {
                println!("error: {}", msg);
                Command::none()
            }
            Self::Message::UserEvent(e) => {
                let cmd = self.select.on_event(&e);
                if cmd.is_some() {
                    if let Runstate::AddArgs(input) = &mut self.runstate {
                        std::mem::swap(&mut self.input, input);
                        self.runstate = Runstate::Select;
                        self.placeholder = "app name".to_string();
                    }
                }
                Command::batch([
                    cmd.unwrap_or(Command::none()),
                    match e {
                        iced::Event::Window(iced::window::Event::Resized { width, height }) => {
                            self.win_size = Size { width, height };
                            // self.select.update(select::Message::Height(h))
                            self.select.update(select::Message::Height(
                                (height as u16)
                                    .checked_sub(
                                        (TEXT_WIDTH + SPACING * 2) + (PADDING * 2) + SPACING,
                                    )
                                    .unwrap_or(0),
                            ))
                        }
                        _ => Command::none(),
                    },
                ])
            }
        }
    }
    fn subscription(&self) -> Subscription<Self::Message> {
        struct SomeSub;
        enum WorkState {
            Normal,
            TrySend(AppMessage),
        }
        enum State {
            Starting,
            Working(comm::Comm, WorkState),
        }
        Subscription::batch([
            iced_futures::subscription::events()
                .map(Self::Message::UserEvent)
                .into(),
            iced_futures::subscription::channel(
                std::any::TypeId::of::<SomeSub>(),
                1000,
                |mut output| async move {
                    let mut state = State::Starting;
                    loop {
                        use iced_futures::futures::sink::SinkExt;
                        match &mut state {
                            State::Starting => {
                                // Create channel
                                let (sender, receiver) = iced_mpsc::channel(1000);
                                // Send the sender back to the application
                                // let _ = output.send(Self::Message::Ready(sender)).await;
                                if let Err(e) = output.send(Self::Message::RpcStart(sender)).await {
                                    log::warn(format!("send ready failed: {:?}", e).as_str());
                                } else {
                                    // We are ready to receive messages
                                    state = State::Working(
                                        comm::Comm::new(receiver),
                                        WorkState::Normal,
                                    );
                                }
                            }
                            State::Working(comm, wstate) => match wstate {
                                WorkState::Normal => {
                                    // Read next input sent from `Application`
                                    if let Some(msg) = comm.next().await {
                                        *wstate = WorkState::TrySend(AppMessage::FromRpc(msg));
                                    }
                                }
                                WorkState::TrySend(msg) => {
                                    if let Err(e) = output.send(msg.clone()).await {
                                        log::warn(format!("send failed: {:?}", e).as_str());
                                    } else {
                                        *wstate = WorkState::Normal;
                                    }
                                }
                            },
                        }
                    }
                },
            )
            .into(),
        ])
    }
    fn view(&self) -> iced::Element<Self::Message> {
        let input = widget::text_input(&self.placeholder, self.input.as_str())
            .line_height(widget::text::LineHeight::Absolute(iced::Pixels(
                TEXT_WIDTH as f32,
            )))
            .padding(PADDING)
            .on_input(|input| AppMessage::FromUi(FromUiMessage::InputChanged(input)))
            .width(iced::Length::Fill)
            .on_submit(AppMessage::FromUi(FromUiMessage::Submit))
            .id(text_input::Id::new("i0"));
        // .direction(
        //     widget::scrollable::Direction::Vertical(),
        //     widget::scrollable::Properties::new().
        // );
        column!(input, self.select.view())
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
