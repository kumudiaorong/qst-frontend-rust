pub mod comm;
pub mod ui;

use iced::Application;
use iced_futures::futures::channel::mpsc as iced_mpsc;
use xlog_rs::log;
enum WorkState {
    Normal,
    TrySend(ui::AppMessage),
}
enum State {
    Starting,
    Working(comm::Comm, WorkState),
}
fn subs() -> iced_futures::Subscription<ui::AppMessage> {
    struct SomeSub;
    iced_futures::subscription::channel(
        std::any::TypeId::of::<SomeSub>(),
        1000,
        |mut output: iced_mpsc::Sender<ui::AppMessage>| async move {
            let mut state = State::Starting;
            loop {
                use iced_futures::futures::sink::SinkExt;
                match &mut state {
                    State::Starting => {
                        let (sender, receiver) = iced_mpsc::channel(1000);
                        if let Err(e) = output.send(ui::AppMessage::RpcStart(sender)).await {
                            log::warn(format!("send ready failed: {:?}", e).as_str());
                        } else {
                            log::info("subscribe ready");
                            state = State::Working(comm::Comm::new(receiver), WorkState::Normal);
                        }
                    }
                    State::Working(comm, wstate) => match wstate {
                        WorkState::Normal => {
                            *wstate = WorkState::TrySend(ui::AppMessage::FromRpc(
                                comm.next()
                                    .await
                                    .map(|resp| match resp {
                                        comm::Response::ConnectResult => {
                                            ui::RpcMessage::ConnectResult
                                        }
                                        comm::Response::SearchResult(mut displays) => {
                                            ui::RpcMessage::SearchResult(
                                                displays
                                                    .drain(..)
                                                    .map(|d| ui::AppInfo {
                                                        id: d.id,
                                                        name: d.name,
                                                        arg_hint: d.hint,
                                                        icon: None,
                                                    })
                                                    .collect(),
                                            )
                                        }
                                        comm::Response::SubmitResult => {
                                            ui::RpcMessage::SubmitResult
                                        }
                                        comm::Response::FillResult(fill) => {
                                            ui::RpcMessage::FillResult(fill)
                                        }
                                    })
                                    .map_err(|e| {
                                        log::warn(format!("rpc error: {:?}", e).as_str());
                                        ui::Error::from(e)
                                    }),
                            ));
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
}
pub fn run() -> iced::Result {
    let settings = iced::Settings::with_flags(ui::Flags::new(std::env::args().collect(), subs));
    ui::App::run(settings)
}
