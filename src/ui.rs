use crate::comm::qst_comm::DisplayList;
use crate::comm::{self, qst_comm};
use crate::select;
use iced::widget::text_input;
use iced::widget::{self, column};
use iced::{executor, window, Application, Command, Element, Size, Subscription, Theme};
use iced_futures::futures::channel::mpsc;
use iced_futures::subscription;
use xlog_rs::log;

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
    S2uReady(mpsc::Sender<comm::Event>),
    U2cTrySendConnect,
    C2uConnectFailed(Error),
    C2uConnected,
    C2uDisconnected,
}

#[derive(Debug, Clone)]
pub enum AppMessage {
    OnConnect(ConnectMessage),
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
    pub fn new(args: Vec<String>) -> Self {
        for (i, arg) in args.iter().enumerate() {
            if arg == "--addr" {
                if i + 1 < args.len() {
                    return Self {
                        addr: "http://".to_string() + args[i + 1].as_str(),
                    };
                }
            }
        }
        Self {
            addr: "".to_string(),
        }
    }
}
pub struct App {
    input: String,
    list: qst_comm::DisplayList,
    tx: Option<mpsc::Sender<comm::Event>>,
    is_connected: bool,
    addr: String,
    select: select::Select,
    win_size: iced::Size<u32>,
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
        log::trace(format!("addr: {}", flags.addr).as_str());
        (
            Self {
                tx: None,
                is_connected: false,
                input: String::new(),
                list: qst_comm::DisplayList::default(),
                addr: flags.addr,
                select: select::Select::with_height(WIN_INIT_SIZE.height - 80),
                win_size: WIN_INIT_SIZE,
            },
            window::resize(WIN_INIT_SIZE),
        )
    }

    fn title(&self) -> String {
        String::from("Qst")
    }

    fn update(&mut self, message: AppMessage) -> Command<AppMessage> {
        match message {
            AppMessage::OnConnect(cmsg) => match cmsg {
                ConnectMessage::S2uReady(tx) => {
                    self.tx = Some(tx);
                    iced::Command::perform(
                        async move {
                            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
                        },
                        |_| AppMessage::OnConnect(ConnectMessage::U2cTrySendConnect),
                    )
                }
                ConnectMessage::U2cTrySendConnect => {
                    if let Err(e) = self.try_send(comm::Event::Connect(self.addr.clone())) {
                        log::warn(format!("send connect failed: {:?}", e).as_str());
                        iced::Command::perform(
                            async move {
                                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                            },
                            |_| AppMessage::OnConnect(ConnectMessage::U2cTrySendConnect),
                        )
                    } else {
                        widget::text_input::focus(text_input::Id::new("i0"))
                    }
                }
                ConnectMessage::C2uConnected => {
                    self.is_connected = true;
                    Command::none()
                }
                ConnectMessage::C2uConnectFailed(e) => {
                    log::warn(format!("connect failed: {:?}", e).as_str());
                    iced::Command::perform(
                        async move {
                            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                        },
                        |_| AppMessage::OnConnect(ConnectMessage::U2cTrySendConnect),
                    )
                }
                ConnectMessage::C2uDisconnected => {
                    self.is_connected = false;
                    log::trace("disconnected");
                    window::close()
                }
            },
            AppMessage::Error(msg) => {
                println!("error: {}", msg);
                Command::none()
            }
            AppMessage::Empty => Command::none(),
            AppMessage::Input(input) => {
                self.input = input;
                if self.input.is_empty() {
                    iced::Command::perform(async move {}, |_| {
                        AppMessage::List(DisplayList::default())
                    })
                } else if self.is_connected {
                    if let Err(e) = self.try_send(comm::Event::InputChanged(qst_comm::Input {
                        str: self.input.clone(),
                    })) {
                        log::warn(format!("input failed: {:?}", e).as_str());
                    }
                    Command::none()
                } else {
                    Command::none()
                }
            }
            AppMessage::List(list) => {
                println!("list: {:?}", list);
                self.select.update(list.list)
            }
            AppMessage::Push(idx) => {
                self.run_app(qst_comm::ExecHint {
                    name: self.list.list[idx].name.clone(),
                    file: None,
                    url: None,
                });
                Command::none()
            }
            AppMessage::RunSuccess => window::close(),
            AppMessage::UserEvent(e) => match e {
                iced::Event::Keyboard(iced::keyboard::Event::KeyPressed { key_code, .. }) => {
                    match key_code {
                        iced::keyboard::KeyCode::Down => self.select.down(),
                        iced::keyboard::KeyCode::Up => self.select.up(),
                        _ => Command::none(),
                    }
                }
                // iced::Event::Window(iced::window::Event::CloseRequested) => {
                //     log::trace("close requested");
                //     if let Err(e) = self.try_send(comm::Event::Over) {
                //         log::warn(format!("input failed: {:?}", e).as_str());
                //     }
                //     Command::none()
                // }
                // iced::Event::Window(iced::window::Event::Resized { width, height }
                _ => Command::none(),
            },
            AppMessage::Submit => {
                if let Some(d) = self.select.selected() {
                    self.run_app(qst_comm::ExecHint {
                        name: d.name.clone(),
                        file: None,
                        url: None,
                    });
                }
                Command::none()
            }
        }
    }
    fn subscription(&self) -> Subscription<AppMessage> {
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
                .map(AppMessage::UserEvent)
                .into(),
            subscription::channel(
                std::any::TypeId::of::<SomeSub>(),
                1000,
                |mut output| async move {
                    let mut state = State::Starting;
                    loop {
                        use iced_futures::futures::sink::SinkExt;
                        match &mut state {
                            State::Starting => {
                                // Create channel
                                let (sender, receiver) = mpsc::channel(1000);

                                // Send the sender back to the application
                                // let _ = output.send(AppMessage::Ready(sender)).await;
                                if let Err(e) = output
                                    .send(
                                        // AppMessage::Ready(sender)
                                        AppMessage::OnConnect(ConnectMessage::S2uReady(sender)),
                                    )
                                    .await
                                {
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
                                        *wstate = WorkState::TrySend(msg);
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
    fn view(&self) -> Element<AppMessage> {
        let input = widget::text_input("", self.input.as_str())
            .line_height(widget::text::LineHeight::Absolute(iced::Pixels(30.0)))
            .on_input(AppMessage::Input)
            .on_submit(AppMessage::Submit)
            .id(text_input::Id::new("i0"));
        // .direction(
        //     widget::scrollable::Direction::Vertical(),
        //     widget::scrollable::Properties::new().
        // );
        column!(input, self.select.view())
            .spacing(5)
            .padding(5)
            .into()
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
