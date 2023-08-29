pub mod qst_comm {
    tonic::include_proto!("qst_comm");
}
use qst_comm::interact_client::InteractClient;

use crate::ui::AppMessage;
use iced_futures::futures::channel::mpsc;
pub enum Event {
    Connect(String),
    InputChanged(qst_comm::Input),
    RunApp(qst_comm::ExecHint),
}
pub enum Status {
    Proc(tonic::Status),
    Send(),
}

pub struct Comm {
    cli: Option<InteractClient<tonic::transport::Channel>>,
    rx: mpsc::Receiver<Event>,
}
impl Comm {
    pub fn new(rx: mpsc::Receiver<Event>) -> Self {
        Self { cli: None, rx }
        // let mut client = InteractClient::connect(addr).await?;
        // Self {
        //     Cli: InteractClient::connect(addr).await?,
        // }
    }
    pub async fn connect(
        &mut self,
        addr: impl Into<String>,
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        match InteractClient::connect(addr.into()).await {
            Ok(c) => {
                self.cli = Some(c);
                Ok(())
            }
            Err(e) => Err(Box::new(e)),
        }
    }
    pub async fn next(&mut self) -> Option<AppMessage> {
        use iced_futures::futures::StreamExt;
        match self.rx.select_next_some().await {
            Event::Connect(addr) => {
                if let Err(e) = self.connect(addr).await {
                    return Some(AppMessage::Error(format!("{:?}", e)));
                }
                return Some(AppMessage::Connected);
            }
            Event::InputChanged(input) => {
                if let Some(ref mut cli) = self.cli {
                    if let Ok(res) = cli.list_app(input).await {
                        return Some(AppMessage::List(res.into_inner()));
                    }
                }
            }
            Event::RunApp(eh) => {
                if let Some(ref mut cli) = self.cli {
                    if let Ok(_) = cli.run_app(eh).await {
                        return Some(AppMessage::RunSuccess);
                    }
                }
            }
        }
        None
    }
}
