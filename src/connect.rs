use super::{rpc, ui};
use iced_futures::futures::{channel::mpsc as iced_mpsc, sink::SinkExt, StreamExt};
use tonic::transport::Endpoint;
use xlog_rs::log;
const MAX_TRY_SEND: usize = 3;
use rpc::{RequestSearch, RequestSubmit, Server};
use ui::FromServer;
use ui::ToServer;
async fn convert(ui: ToServer, ser: &mut Server) -> Result<FromServer, ui::Error> {
    match ui {
        ToServer::Search { prompt, content } => {
            if let Some(ext) = ser.get_ext(&prompt).await {
                let mut r = ext
                    .request(RequestSearch { content })
                    .await
                    .map_err(|e| ui::Error::from(e))?;
                Ok(FromServer::Search(
                    r.list
                        .drain(..)
                        .map(|d| ui::Item {
                            obj_id: d.obj_id,
                            name: d.name,
                            arg_hint: d.hint,
                            icon: None,
                        })
                        .collect(),
                ))
            } else {
                Err(ui::Error::from("no such prompt"))
            }
        }
        ToServer::Submit {
            prompt,
            obj_id,
            hint,
        } => {
            if let Some(ext) = ser.get_ext(&prompt).await {
                ext.request(RequestSubmit { obj_id, hint })
                    .await
                    .map_err(|e| ui::Error::from(e))?;
                Ok(FromServer::Submit)
            } else {
                Err(ui::Error::from("no such prompt"))
            }
        }
    }
}
struct SomeSub;
enum State {
    TryRecv,
    TrySend {
        msg: ui::Message,
        action: fn() -> State,
        cnt: usize,
    },
}
impl State {
    pub fn new(msg: ui::Message, action: fn() -> State, cnt: usize) -> Self {
        Self::TrySend { msg, action, cnt }
    }
    pub fn with_msg(msg: ui::Message) -> Self {
        Self::TrySend {
            msg,
            action: || Self::TryRecv,
            cnt: 0,
        }
    }
}
pub fn connect(ep: Endpoint) -> iced_futures::Subscription<ui::Message> {
    iced_futures::subscription::channel(
        std::any::TypeId::of::<SomeSub>(),
        1000,
        |mut toui: iced_mpsc::Sender<ui::Message>| async move {
            let (tx, mut rx) = iced_mpsc::channel(1000);

            let mut msgqueue = std::collections::VecDeque::from(vec![State::with_msg(
                ui::Message::FromServer(Ok(ui::FromServer::Setup(tx))),
            )]);
            let mut server = rpc::Server::connet(ep.clone())
                .await
                .expect("connect to server failed");
            while let Some(state) = msgqueue.pop_front() {
                let mut states = vec![];
                match state {
                    State::TryRecv => {
                        states.push(State::with_msg(ui::Message::FromServer(
                            convert(rx.select_next_some().await, &mut server).await,
                        )));
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
                            states.push(State::new(msg, action, cnt + 1));
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
