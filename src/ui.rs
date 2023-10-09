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
            Message::ToServer(req) => match self.try_send(req.clone()) {
                Err(e) => {
                    log::warn(format!("input failed: {:?}", e).as_str());
                    Command::perform(
                        async {
                            async_sleep(Duration::from_millis(200)).await;
                        },
                        move |_| Self::Message::ToServer(req),
                    )
                }
                _ => Command::none(),
            },
            Message::FromServer(result) => match result {
                Ok(msg) => match msg {
                    FromServer::Connected => {
                        self.is_connected = true;
                        widget::text_input::focus(text_input::Id::new("i0"))
                    }
                    FromServer::Search(list) => self
                        .select
                        .update(select::Message::Refresh(list))
                        .map(convert_select_msg),
                    // FromServer::FillResult(content) => {
                    //     self.input = content;
                    //     Command::none()
                    // }
                    FromServer::Submit => Command::none(),
                },
                Err(_) => Command::none(),
            },
            Message::FromUi(umsg) => match umsg {
                FromUi::InputChanged(input) => match input.is_empty() {
                    true => {
                        self.input.clear();
                        Command::perform(async {}, move |_| {
                            Self::Message::FromServer(Ok(FromServer::Search(vec![])))
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
        column!(input, self.select.view().map(convert_select_msg))
            .spacing(SPACING)
            .padding(PADDING)
            .into()
    }
}
