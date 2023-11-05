mod flag;
mod rpc;
mod ui;
mod utils;

use std::cell::Cell;

use iced::Application;

use iced_futures::{futures::channel::mpsc as iced_mpsc, subscription::Recipe};
use tonic::transport::Endpoint;
use xlog_rs::log;

const MAX_TRY_SEND: usize = 3;

struct Converter {
    rx: iced_mpsc::Receiver<ui::ToServer>,
    server: rpc::Server,
}
impl Converter {
    pub async fn new(
        rx: iced_mpsc::Receiver<ui::ToServer>,
        ep: Endpoint,
    ) -> Result<Self, rpc::Error> {
        Ok(Self {
            rx,
            server: rpc::Server::connet(ep).await?,
        })
    }
}

fn connect(ep: Endpoint) -> iced_futures::Subscription<ui::Message> {
    struct SomeSub;
    enum State {
        TryRecv,
        TrySend {
            msg: ui::Message,
            action: fn() -> State,
            cnt: usize,
        },
    }
    iced_futures::subscription::channel(
        std::any::TypeId::of::<SomeSub>(),
        1000,
        |mut toui: iced_mpsc::Sender<ui::Message>| async move {
            let (tx, mut rx) = iced_mpsc::channel(1000);

            let mut msgqueue = std::collections::VecDeque::from(vec![State::TrySend {
                msg: ui::Message::FromServer(Ok(ui::FromServer::Setup(tx))),
                action: || State::TryRecv,
                cnt: 0,
            }]);
            let mut server = rpc::Server::connet(ep.clone())
                .await
                .expect("connect to server failed");
            while let Some(state) = msgqueue.pop_front() {
                use iced_futures::futures::sink::SinkExt;
                let mut states = vec![];
                match state {
                    State::TryRecv => {
                        use iced_futures::futures::StreamExt;
                        states.push(State::TrySend {
                            msg: ui::Message::FromServer(
                                utils::convert(rx.select_next_some().await, &mut server).await,
                            ),
                            action: || State::TryRecv,
                            cnt: 0,
                        });
                    }
                    State::TrySend { msg, action, cnt } => {
                        if cnt >= MAX_TRY_SEND {
                            log::error(format!("try send failed too many times").as_str());
                            std::process::exit(1);
                        } else if let Err(e) = toui.send(msg.clone()).await {
                            log::warn(format!("send failed: {:?}", e).as_str());
                            // use tokio::time::{sleep as async_sleep, Duration};
                            // async_sleep(Duration::from_millis(100)).await;
                            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                            states.push(State::TrySend {
                                msg,
                                action,
                                cnt: cnt + 1,
                            });
                        } else {
                            states.push((action)());
                        }
                    }
                }
                msgqueue.extend(states);
            }
            panic!("should not reach here")
        },
    )
}
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    use clap::Parser;
    let args = flag::Args::parse();
    args.uri.parse::<Endpoint>().unwrap();
    // let c = Cell::new(receiver);
    let settings = iced::Settings::with_flags(ui::Flags::new(Box::new(move || {
        connect(args.uri.parse::<Endpoint>().unwrap())
    })));
    ui::App::run(settings).map_err(|e| e.into())
}
