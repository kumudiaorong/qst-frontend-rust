pub mod defs {
    tonic::include_proto!("defs");
}

pub mod daemon;
pub mod error;
pub mod ext;
mod utils;

pub use ext::DisplayList;
pub use ext::Input;
pub use ext::SubmitHint;

use xlog_rs::log;

#[derive(Debug, Clone)]
pub enum Request {
    Connect(tonic::transport::Endpoint),
    Search {
        prompt: String,
        input: ext::Input,
    },
    Submit {
        prompt: String,
        hint: ext::SubmitHint,
    },
    // Fill {
    //     prompt: String,
    //     obj_id: u32,
    // },
}

#[derive(Debug, Clone)]
pub enum Response {
    Connected,
    Search(Vec<ext::Display>),
    Submit,
    // FillResult(String),
}

pub const MAX_TRY_CONNECT: usize = 3;

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
                        Response::Connected
                    })
            }
            Request::Search { prompt, input } => {
                log::debug(format!("search {} with {:#?}", prompt, input).as_str());
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
                .search(input)
                .await
                .map(|displays| Response::Search(displays))
            }
            Request::Submit { prompt, hint } => self
                .ext
                .get_mut(&prompt)
                .unwrap()
                .submit(hint)
                .await
                .map(|_| Response::Submit),
            // Request::Fill { prompt, obj_id } => self
            //     .ext
            //     .get_mut(&prompt)
            //     .unwrap()
            //     .fill(obj_id)
            //     .await
            //     .map(|content| Response::FillResult(content)),
        }
    }
}
