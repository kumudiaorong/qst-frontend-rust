mod rpc;
mod ui;
mod utils;

use iced::Application;
use iced_futures::futures::channel::mpsc as iced_mpsc;

use tokio::time::{sleep as async_sleep, Duration};
use xlog_rs::log;

const MAX_TRY_SEND: usize = 3;

fn subs() -> iced_futures::Subscription<ui::Message> {
    struct SomeSub;
    enum WorkState {
        TryRecv,
        TrySend {
            msg: ui::Message,
            action: fn(rpc::Service, iced_mpsc::Receiver<ui::ToServer>) -> State,
            cnt: usize,
        },
    }
    enum State {
        Starting,
        Working(rpc::Service, iced_mpsc::Receiver<ui::ToServer>, WorkState),
    }
    iced_futures::subscription::channel(
        std::any::TypeId::of::<SomeSub>(),
        1000,
        |mut toui: iced_mpsc::Sender<ui::Message>| async move {
            let mut msgqueue = std::collections::VecDeque::from(vec![State::Starting]);
            while let Some(state) = msgqueue.pop_front() {
                use iced_futures::futures::sink::SinkExt;
                match state {
                    State::Starting => {
                        let (sender, receiver) = iced_mpsc::channel(1000);
                        msgqueue.push_back(State::Working(
                            rpc::Service::new(),
                            receiver,
                            WorkState::TrySend {
                                msg: ui::Message::Start(sender),
                                action: |rpc, rx| State::Working(rpc, rx, WorkState::TryRecv),
                                cnt: 0,
                            },
                        ));
                    }
                    State::Working(mut rpc, mut rx, wstate) => match wstate {
                        WorkState::TryRecv => {
                            use iced_futures::futures::StreamExt;
                            let ret = rpc
                                .request(utils::convert_ui_to_server(rx.select_next_some().await))
                                .await;
                            msgqueue.push_back(State::Working(
                                rpc,
                                rx,
                                WorkState::TrySend {
                                    msg: ui::Message::FromServer(
                                        ret.map(utils::convert_server_to_ui).map_err(|e| {
                                            log::warn(format!("rpc error: {:?}", e).as_str());
                                            ui::Error::from(e)
                                        }),
                                    ),
                                    action: |rpc, rx| State::Working(rpc, rx, WorkState::TryRecv),
                                    cnt: 0,
                                },
                            ));
                        }
                        WorkState::TrySend { msg, action, cnt } => {
                            if cnt >= MAX_TRY_SEND {
                                log::error(format!("try send failed too many times").as_str());
                                std::process::exit(1);
                            } else if let Err(e) = toui.send(msg.clone()).await {
                                log::warn(format!("send failed: {:?}", e).as_str());
                                async_sleep(Duration::from_millis(100)).await;
                            } else {
                                msgqueue.push_back((action)(rpc, rx));
                            }
                        }
                    },
                }
            }
            panic!("should not reach here")
            // let mut state = State::Starting;
            // loop {
            //     use iced_futures::futures::sink::SinkExt;
            //     match &mut state {
            //         State::Starting => {
            //             let (sender, receiver) = iced_mpsc::channel(1000);
            //             if let Err(e) = toui.send(ui::Message::Start(sender)).await {
            //                 log::warn(format!("send ready failed: {:?}", e).as_str());
            //                 async_sleep(Duration::from_millis(100)).await;
            //             } else {
            //                 log::info("subscribe ready");
            //                 state =
            //                     State::Working(rpc::Service::new(), receiver, WorkState::TryRecv);
            //             }
            //         }
            //         State::Working(rpc, rx, wstate) => match wstate {
            //             WorkState::TryRecv => {
            //                 use iced_futures::futures::StreamExt;
            //                 let ret = match rx.select_next_some().await {
            //                     ui::ToServer::Connect(endpoint) => {
            //                         rpc.request(rpc::Request::Connect(endpoint.clone())).await
            //                     }
            //                     ui::ToServer::Search { prompt, content } => {
            //                         rpc.request(rpc::Request::Search {
            //                             prompt,
            //                             input: rpc::Input { content },
            //                         })
            //                         .await
            //                     }
            //                     ui::ToServer::Submit {
            //                         prompt,
            //                         obj_id,
            //                         hint,
            //                     } => {
            //                         rpc.request(rpc::Request::Submit {
            //                             prompt,
            //                             hint: rpc::SubmitHint { obj_id, hint },
            //                         })
            //                         .await
            //                     } // ui::ToServer::Fill { prompt, obj_id } => {
            //                       //     rpc.request(rpc::Request::Fill { prompt, obj_id }).await
            //                       // }
            //                 };
            //                 *wstate = WorkState::TrySend {
            //                     msg: ui::Message::FromServer(
            //                         ret.map(|resp| match resp {
            //                             rpc::Response::Connected => ui::FromServer::ConnectResult,
            //                             rpc::Response::Search(mut displays) => {
            //                                 ui::FromServer::SearchResult(
            //                                     displays
            //                                         .drain(..)
            //                                         .map(|d| ui::AppInfo {
            //                                             id: d.id,
            //                                             name: d.name,
            //                                             arg_hint: d.hint,
            //                                             icon: None,
            //                                         })
            //                                         .collect(),
            //                                 )
            //                             }
            //                             rpc::Response::Submit => ui::FromServer::SubmitResult,
            //                             // rpc::Response::FillResult(fill) => {
            //                             //     ui::FromServer::FillResult(fill)
            //                             // }
            //                         })
            //                         .map_err(|e| {
            //                             log::warn(format!("rpc error: {:?}", e).as_str());
            //                             ui::Error::from(e)
            //                         }),
            //                     ),
            //                     action: |_, wstate| *wstate = WorkState::TryRecv,
            //                     cnt: 0,
            //                 };
            //             }
            //             WorkState::TrySend { msg, action, cnt } => {
            //                 if *cnt >= MAX_TRY_SEND {
            //                     log::error(format!("try send failed too many times").as_str());
            //                     std::process::exit(1);
            //                 } else if let Err(e) = toui.send(msg.clone()).await {
            //                     log::warn(format!("send failed: {:?}", e).as_str());
            //                     async_sleep(Duration::from_millis(100)).await;
            //                 } else {
            //                     (action)(&mut state, wstate);
            //                 }
            //             }
            //         },
            //     }
            // }
        },
    )
}
pub fn run() -> iced::Result {
    let settings = iced::Settings::with_flags(ui::Flags::new(std::env::args().collect(), subs));
    ui::App::run(settings)
}
