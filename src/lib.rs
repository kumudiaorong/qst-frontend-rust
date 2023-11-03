mod flag;
mod rpc;
mod ui;
mod utils;

use iced::Application;

use xlog_rs::log;

const MAX_TRY_SEND: usize = 3;

fn connect() -> iced_futures::Subscription<ui::Message> {
    struct SomeSub;
    enum State {
        Starting,
        TryRecv,
        TrySend {
            msg: ui::Message,
            action: fn() -> State,
            cnt: usize,
        },
    }
    use iced_futures::futures::channel::mpsc as async_mpsc;
    iced_futures::subscription::channel(
        std::any::TypeId::of::<SomeSub>(),
        1000,
        |mut toui: async_mpsc::Sender<ui::Message>| async move {
            let mut msgqueue = std::collections::VecDeque::from(vec![State::Starting]);
            let mut server = rpc::Server::new();
            let mut rx: Option<async_mpsc::Receiver<ui::ToServer>> = None;
            while let Some(state) = msgqueue.pop_front() {
                use iced_futures::futures::sink::SinkExt;
                let mut states = vec![];
                match state {
                    State::Starting => {
                        let (sender, receiver) = async_mpsc::channel(1000);
                        rx = Some(receiver);
                        states.push(State::TrySend {
                            msg: ui::Message::Start(sender),
                            action: || State::TryRecv,
                            cnt: 0,
                        });
                    }
                    State::TryRecv => {
                        use iced_futures::futures::StreamExt;
                        states.push(State::TrySend {
                            msg: ui::Message::FromServer(
                                utils::convert(
                                    rx.as_mut().unwrap().select_next_some().await,
                                    &mut server,
                                )
                                .await,
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
pub fn run() -> iced::Result {
    let settings = iced::Settings::with_flags(ui::Flags::new(std::env::args().collect(), connect));
    ui::App::run(settings)
}
