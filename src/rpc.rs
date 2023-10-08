pub mod defs {
    tonic::include_proto!("defs");
}

pub mod daemon;
pub mod error;
pub mod ext;
mod utils;
pub use ext::DisplayList;
use iced_futures::futures::channel::mpsc as iced_mpsc;
use xlog_rs::log;
#[derive(Debug, Clone)]
pub enum Request {
    Connect(tonic::transport::Endpoint),
    Search {
        prompt: String,
        input: String,
    },
    Submit {
        prompt: String,
        obj_id: u32,
        hint: Option<String>,
    },
    Fill {
        prompt: String,
        obj_id: u32,
    },
}

#[derive(Debug, Clone)]
pub enum Response {
    ConnectResult,
    SearchResult(Vec<ext::Display>),
    SubmitResult,
    FillResult(String),
}

pub const MAX_TRY_CONNECT: usize = 3;

// pub struct Comm {
//     dae: Option<daemon::DaemonService>,
//     ext: std::collections::HashMap<String, ext::ExtService>,
//     rx: iced_mpsc::Receiver<Request>,
// }
// impl Comm {
//     pub fn new(rx: iced_mpsc::Receiver<Request>) -> Self {
//         Self {
//             dae: None,
//             ext: std::collections::HashMap::new(),
//             rx,
//         }
//     }
//     pub async fn next(&mut self) -> Result<Response, error::Error> {
//         use iced_futures::futures::StreamExt;
//         match self.rx.select_next_some().await {
//             Request::Connect(endpoint) => {
//                 log::debug(format!("connect to {:?}", endpoint).as_str());
//                 daemon::DaemonService::connect(MAX_TRY_CONNECT as u32, endpoint.clone())
//                     .await
//                     .map(|dae| {
//                         self.dae = Some(dae);
//                         Response::ConnectResult
//                     })
//             }
//             Request::Search { prompt, input } => {
//                 log::debug(format!("search {} with {}", prompt, input).as_str());
//                 match self.ext.get_mut(&prompt) {
//                     Some(e) => e,
//                     None => {
//                         self.ext.insert(
//                             prompt.clone(),
//                             ext::ExtService::with_port(
//                                 MAX_TRY_CONNECT as u32,
//                                 self.dae.as_mut().unwrap().get_ext_port(&prompt).await?,
//                             )
//                             .await?,
//                         );
//                         self.ext.get_mut(&prompt).unwrap()
//                     }
//                 }
//                 .search(input.as_str())
//                 .await
//                 .map(|displays| Response::SearchResult(displays))
//             }
//             Request::Submit {
//                 prompt,
//                 obj_id,
//                 hint,
//             } => self
//                 .ext
//                 .get_mut(&prompt)
//                 .unwrap()
//                 .submit(obj_id, hint)
//                 .await
//                 .map(|_| Response::SubmitResult),
//             Request::Fill { prompt, obj_id } => self
//                 .ext
//                 .get_mut(&prompt)
//                 .unwrap()
//                 .fill(obj_id)
//                 .await
//                 .map(|content| Response::FillResult(content)),
//         }
//     }
//     pub async fn request(&mut self, req: Request) -> Result<Response, error::Error> {
//         match req {
//             Request::Connect(endpoint) => {
//                 log::debug(format!("connect to {:?}", endpoint).as_str());
//                 daemon::DaemonService::connect(MAX_TRY_CONNECT as u32, endpoint.clone())
//                     .await
//                     .map(|dae| {
//                         self.dae = Some(dae);
//                         Response::ConnectResult
//                     })
//             }
//             Request::Search { prompt, input } => {
//                 log::debug(format!("search {} with {}", prompt, input).as_str());
//                 match self.ext.get_mut(&prompt) {
//                     Some(e) => e,
//                     None => {
//                         self.ext.insert(
//                             prompt.clone(),
//                             ext::ExtService::with_port(
//                                 MAX_TRY_CONNECT as u32,
//                                 self.dae.as_mut().unwrap().get_ext_port(&prompt).await?,
//                             )
//                             .await?,
//                         );
//                         self.ext.get_mut(&prompt).unwrap()
//                     }
//                 }
//                 .search(input.as_str())
//                 .await
//                 .map(|displays| Response::SearchResult(displays))
//             }
//             Request::Submit {
//                 prompt,
//                 obj_id,
//                 hint,
//             } => self
//                 .ext
//                 .get_mut(&prompt)
//                 .unwrap()
//                 .submit(obj_id, hint)
//                 .await
//                 .map(|_| Response::SubmitResult),
//             Request::Fill { prompt, obj_id } => self
//                 .ext
//                 .get_mut(&prompt)
//                 .unwrap()
//                 .fill(obj_id)
//                 .await
//                 .map(|content| Response::FillResult(content)),
//         }
//     }
// }
pub struct Service {
    dae: Option<daemon::DaemonService>,
    ext: std::collections::HashMap<String, ext::ExtService>,
}
impl Service {
    pub fn new() -> Self {
        Self {
            dae: None,
            ext: std::collections::HashMap::new(),
        }
    }
    pub async fn request(&mut self, req: Request) -> Result<Response, error::Error> {
        match req {
            Request::Connect(endpoint) => {
                log::debug(format!("connect to {:?}", endpoint).as_str());
                daemon::DaemonService::connect(MAX_TRY_CONNECT as u32, endpoint.clone())
                    .await
                    .map(|dae| {
                        self.dae = Some(dae);
                        Response::ConnectResult
                    })
            }
            Request::Search { prompt, input } => {
                log::debug(format!("search {} with {}", prompt, input).as_str());
                match self.ext.get_mut(&prompt) {
                    Some(e) => e,
                    None => {
                        self.ext.insert(
                            prompt.clone(),
                            ext::ExtService::with_port(
                                MAX_TRY_CONNECT as u32,
                                self.dae.as_mut().unwrap().get_ext_port(&prompt).await?,
                            )
                            .await?,
                        );
                        self.ext.get_mut(&prompt).unwrap()
                    }
                }
                .search(input.as_str())
                .await
                .map(|displays| Response::SearchResult(displays))
            }
            Request::Submit {
                prompt,
                obj_id,
                hint,
            } => self
                .ext
                .get_mut(&prompt)
                .unwrap()
                .submit(obj_id, hint)
                .await
                .map(|_| Response::SubmitResult),
            Request::Fill { prompt, obj_id } => self
                .ext
                .get_mut(&prompt)
                .unwrap()
                .fill(obj_id)
                .await
                .map(|content| Response::FillResult(content)),
        }
    }
}
