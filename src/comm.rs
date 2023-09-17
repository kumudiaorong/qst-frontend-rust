pub mod qst_comm {
    tonic::include_proto!("qst_comm");
}
use iced_futures::futures::channel::mpsc as iced_mpsc;
pub use qst_comm::*;
#[derive(Debug, Clone)]
pub enum Request {
    Connect(tonic::transport::Endpoint),
    Search(String),
    RunApp(ExecHint),
}

#[derive(Debug, Clone)]
pub enum Response {
    Connected,
    ConnectFailed(String),
    SearchResult(DisplayList),
    RunSuccess,
}

const MAX_TRY_CONNECT: usize = 3;

pub struct Comm {
    cli: Option<interact_client::InteractClient<tonic::transport::Channel>>,
    rx: iced_mpsc::Receiver<Request>,
    connect_try: usize,
}
impl Comm {
    pub fn new(rx: iced_mpsc::Receiver<Request>) -> Self {
        Self {
            cli: None,
            rx,
            connect_try: 0,
        }
    }
    pub async fn connect(
        &mut self,
        endpoint: tonic::transport::Endpoint,
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        match interact_client::InteractClient::connect(endpoint).await {
            Ok(c) => {
                self.cli = Some(c);
                Ok(())
            }
            Err(e) => Err(Box::new(e)),
        }
    }
    pub async fn next(&mut self) -> Option<Response> {
        use iced_futures::futures::StreamExt;
        match self.rx.select_next_some().await {
            Request::Connect(endpoint) => {
                while self.connect_try < MAX_TRY_CONNECT - 1 {
                    if let Err(_) = self.connect(endpoint.clone()).await {
                        self.connect_try += 1;
                    } else {
                        self.connect_try = 0;
                        break;
                    }
                }
                if self.connect_try == MAX_TRY_CONNECT - 1 {
                    if let Err(e) = self.connect(endpoint).await {
                        self.connect_try = 0;
                        return Some(Response::ConnectFailed(e.to_string()));
                    }
                }
                self.connect_try = 0;
                return Some(Response::Connected);
            }
            Request::Search(input) => {
                if let Some(ref mut cli) = self.cli {
                    if let Ok(res) = cli.list_app(Input { str: input.clone() }).await {
                        return Some(Response::SearchResult(res.into_inner()));
                    }
                }
            }
            Request::RunApp(eh) => {
                if let Some(ref mut cli) = self.cli {
                    if let Ok(_) = cli.run_app(eh).await {
                        return Some(Response::RunSuccess);
                    }
                }
            }
        }
        None
    }
}
