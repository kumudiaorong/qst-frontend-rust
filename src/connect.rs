use super::{
    rpc::{RequestSearch, RequestSubmit, Server},
    ui::{Error, FromServer, Item, Message, ToServer},
};
use iced_futures::futures::channel::mpsc as iced_mpsc;
use tonic::transport::Endpoint;
use xlog_rs::log;

const MAX_TRY_SEND: usize = 3;
async fn convert(ui: ToServer, ser: &mut Server) -> Result<FromServer, Error> {
    match ui {
        ToServer::Search { prompt, content } => {
            if let Some(ext) = ser.get_ext(&prompt).await {
                let mut r = ext
                    .request(RequestSearch { content })
                    .await
                    .map_err(|e| Error::from(e))?;
                Ok(FromServer::Search(
                    r.list
                        .drain(..)
                        .map(|d| Item {
                            obj_id: d.obj_id,
                            name: d.name,
                            arg_hint: d.hint,
                            icon: None,
                        })
                        .collect(),
                ))
            } else {
                Err(Error::from("no such prompt"))
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
                    .map_err(|e| Error::from(e))?;
                Ok(FromServer::Submit)
            } else {
                Err(Error::from("no such prompt"))
            }
        }
    }
}
struct SomeSub;
enum State {
    TryRecv,
    TrySend {
        msg: Message,
        action: fn() -> State,
        cnt: usize,
    },
}
impl State {
    pub fn new(msg: Message, action: fn() -> State, cnt: usize) -> Self {
        Self::TrySend { msg, action, cnt }
    }
    pub fn with_msg(msg: Message) -> Self {
        Self::TrySend {
            msg,
            action: || Self::TryRecv,
            cnt: 0,
        }
    }
}
pub fn connect(ep: Endpoint) -> iced_futures::Subscription<Message> {
    iced_futures::subscription::channel(
        std::any::TypeId::of::<SomeSub>(),
        1000,
        |mut toui: iced_mpsc::Sender<Message>| async move {
            let (tx, mut rx) = iced_mpsc::channel(1000);

            let mut msgqueue = std::collections::VecDeque::from(vec![State::with_msg(
                Message::FromServer(Ok(FromServer::Setup(tx))),
            )]);
            let mut server = Server::connet(ep.clone())
                .await
                .expect("connect to server failed");
            while let Some(state) = msgqueue.pop_front() {
                match state {
                    State::TryRecv => {
                        use iced_futures::futures::StreamExt;
                        msgqueue.push_back(State::with_msg(Message::FromServer(
                            convert(rx.select_next_some().await, &mut server).await,
                        )));
                    }
                    State::TrySend { msg, action, cnt } => {
                        use iced_futures::futures::SinkExt;
                        if cnt >= MAX_TRY_SEND {
                            log::error(format!("try send failed too many times").as_str());
                            std::process::exit(1);
                        } else if let Err(e) = toui.send(msg.clone()).await {
                            log::warn(format!("send failed: {:?}", e).as_str());
                            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                            msgqueue.push_back(State::new(msg, action, cnt + 1));
                        } else {
                            msgqueue.push_back((action)());
                        }
                    }
                }
            }
            panic!("should not reach here")
        },
    )
}
