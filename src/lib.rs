pub mod rpc;
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
    Working(rpc::Service, iced_mpsc::Receiver<ui::ToServer>, WorkState),
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
                        if let Err(e) = output.send(ui::AppMessage::Start(sender)).await {
                            log::warn(format!("send ready failed: {:?}", e).as_str());
                        } else {
                            log::info("subscribe ready");
                            state =
                                State::Working(rpc::Service::new(), receiver, WorkState::Normal);
                        }
                    }
                    State::Working(rpc, rx, wstate) => match wstate {
                        WorkState::Normal => {
                            use iced_futures::futures::StreamExt;
                            let ret = match rx.select_next_some().await {
                                ui::ToServer::Connect(endpoint) => {
                                    rpc.request(rpc::Request::Connect(endpoint.clone())).await
                                }
                                ui::ToServer::Search { prompt, input } => {
                                    rpc.request(rpc::Request::Search { prompt, input }).await
                                }
                                ui::ToServer::Submit {
                                    prompt,
                                    obj_id,
                                    hint,
                                } => {
                                    rpc.request(rpc::Request::Submit {
                                        prompt,
                                        obj_id,
                                        hint,
                                    })
                                    .await
                                }
                                ui::ToServer::Fill { prompt, obj_id } => {
                                    rpc.request(rpc::Request::Fill { prompt, obj_id }).await
                                }
                            };
                            *wstate = WorkState::TrySend(ui::AppMessage::FromServer(
                                ret.map(|resp| match resp {
                                    rpc::Response::ConnectResult => ui::FromServer::ConnectResult,
                                    rpc::Response::SearchResult(mut displays) => {
                                        ui::FromServer::SearchResult(
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
                                    rpc::Response::SubmitResult => ui::FromServer::SubmitResult,
                                    rpc::Response::FillResult(fill) => {
                                        ui::FromServer::FillResult(fill)
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
