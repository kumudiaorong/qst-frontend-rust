mod flags;
pub use flags::Flags;

mod select;
pub use select::Item;

mod utils;

mod error;
pub use error::Error;

use iced::{
    widget::{self, column, text_input},
    window, Command, Size, Subscription,
};

use iced_futures::futures::channel::mpsc as iced_mpsc;

use xlog_rs::log;

pub const SPACING: u16 = 5;
pub const PADDING: u16 = 5;
pub const TEXT_WIDTH: u16 = 35;

const WIN_INIT_SIZE: Size<u32> = Size {
    width: 300,
    height: 245,
};

fn convert_select_msg(msg: select::Message) -> Message {
    Message::FromUi(FromUi::Select(msg))
}

#[derive(Debug, Clone)]
pub enum FromUi {
    InputChanged(String),
    Select(select::Message),
    Submit,
}
#[derive(Debug, Clone)]
pub enum FromServer {
    Connected,
    Search(Vec<select::Item>),
    Submit,
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

pub struct App {
    flags: flags::Flags,
    tx: Option<iced_mpsc::Sender<ToServer>>,
    select: select::Select,
    win_size: Size<u32>,
    placeholder: String,
    input: String,
    prompt: String,
    runstate: Runstate,
}

impl App {
    fn try_send(&mut self, req: ToServer) -> Result<(), iced_mpsc::TrySendError<ToServer>> {
        self.tx.as_mut().unwrap().try_send(req)
    }
    fn select(&mut self) {
        if let Some(item) = self.select.selected() {
            self.runstate = Runstate::AddArgs {
                placeholder: std::mem::replace(&mut self.placeholder, String::new()),
                input: std::mem::replace(&mut self.input, String::new()),
                obj_id: item.obj_id,
            };
            self.placeholder = item.arg_hint.clone().unwrap_or("none args".to_string());
        }
    }
    fn submit(&mut self, obj_id: u32) -> Result<(), iced_mpsc::TrySendError<ToServer>> {
        self.try_send(ToServer::Submit {
            prompt: self.prompt.clone(),
            obj_id,
            hint: Some(self.input.clone()),
        })
    }
    fn try_reload(&mut self) {
        if let Runstate::AddArgs {
            placeholder, input, ..
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
        let (select, cmd) = select::Select::new(
            WIN_INIT_SIZE.height as u16 - (TEXT_WIDTH + SPACING * 2) - (PADDING * 2) - SPACING,
        );
        (
            Self {
                tx: None,
                input: String::new(),
                select,
                win_size: WIN_INIT_SIZE,
                placeholder: "[prompt]content".to_string(),
                runstate: Runstate::Select,
                prompt: String::new(),
                flags,
            },
            Command::batch([window::resize(WIN_INIT_SIZE), cmd.map(convert_select_msg)]),
        )
    }

    fn title(&self) -> String {
        String::from("Qst")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Self::Message::Start(tx) => {
                self.tx = Some(tx);
                let ed = self.flags.endpoint.clone();
                Command::perform(async {}, move |_| {
                    Self::Message::ToServer(ToServer::Connect(ed))
                })
            }
            Self::Message::ToServer(req) => match self.try_send(req.clone()) {
                Err(e) => {
                    log::warn(format!("input failed: {:?}", e).as_str());
                    Command::perform(
                        async {
                            // use tokio::time::{sleep as async_sleep, Duration};
                            // async_sleep(Duration::from_millis(200)).await;
                            tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
                        },
                        move |_| Self::Message::ToServer(req),
                    )
                }
                _ => Command::none(),
            },
            Self::Message::FromServer(result) => match result {
                Ok(msg) => match msg {
                    FromServer::Connected => text_input::focus(text_input::Id::new("i0")),
                    FromServer::Search(list) => self
                        .select
                        .update(select::Message::Refresh(list))
                        .map(convert_select_msg),
                    // FromServer::FillResult(content) => {
                    //     self.input = content;
                    //     Command::none()
                    // }
                    FromServer::Submit => {
                        std::process::exit(0);
                    }
                },
                Err(_) => Command::none(),
            },
            Self::Message::FromUi(umsg) => match umsg {
                FromUi::InputChanged(input) => {
                    if input.is_empty() {
                        self.input.clear();
                        Command::perform(async {}, move |_| {
                            Self::Message::FromServer(Ok(FromServer::Search(vec![])))
                        })
                    } else {
                        self.input = input;
                        match utils::extract_prompt(self.input.as_str()) {
                            Some((prompt, content))
                                if self.runstate == Runstate::Select || prompt != self.prompt =>
                            {
                                self.prompt = prompt.clone();
                                if let Err(e) = self.try_send(ToServer::Search { prompt, content })
                                {
                                    log::warn(format!("search send failed: {:?}", e).as_str());
                                }
                            }
                            _ => {}
                        }
                        Command::none()
                    }
                }
                FromUi::Select(smsg) => {
                    let cmd = self.select.update(smsg.clone()).map(convert_select_msg);
                    match smsg {
                        select::Message::Push { obj_id: sid, .. } => match &self.runstate {
                            Runstate::Select => {
                                self.select();
                            }
                            Runstate::AddArgs { obj_id, .. } => {
                                // let item = self.select.selected().unwrap();
                                if sid == *obj_id {
                                    if let Err(e) = self.submit(sid) {
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
                        Runstate::AddArgs { obj_id, .. } => {
                            if let Err(e) = self.submit(obj_id) {
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
                    self.select.update(select::Message::Height(
                        (height as u16)
                            .checked_sub((TEXT_WIDTH + SPACING * 2) + (PADDING * 2) + SPACING)
                            .unwrap_or(0),
                    ))
                }
                iced::Event::Keyboard(iced::keyboard::Event::KeyPressed { key_code, .. }) => {
                    match key_code {
                        iced::keyboard::KeyCode::Up => {
                            self.try_reload();
                            self.select.update(select::Message::Up)
                        }
                        iced::keyboard::KeyCode::Down => {
                            self.try_reload();
                            self.select.update(select::Message::Down)
                        }
                        _ => Command::none(),
                    }
                }

                _ => Command::none(),
            }
            .map(convert_select_msg),
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
        let input = text_input(&self.placeholder, self.input.as_str())
            .line_height(widget::text::LineHeight::Absolute(iced::Pixels(
                TEXT_WIDTH as f32,
            )))
            .padding(PADDING)
            .on_input(|input| Self::Message::FromUi(FromUi::InputChanged(input)))
            .width(iced::Length::Fill)
            .on_submit(Self::Message::FromUi(FromUi::Submit))
            .id(text_input::Id::new("i0"));
        column!(input, self.select.view().map(convert_select_msg))
            .spacing(SPACING)
            .padding(PADDING)
            .into()
    }
}
