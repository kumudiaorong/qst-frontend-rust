pub mod defs;

pub mod daemon;
pub mod error;
pub mod extension;
mod response;
pub use response::Response;
mod service;
mod utils;
use daemon::Service as DaemonService;
pub use extension::DisplayList;
pub use extension::Input;
use extension::Service as ExtService;
pub use extension::SubmitHint;
use service::Service;
use xlog_rs::log;

pub const MAX_TRY_CONNECT: usize = 3;

pub struct Server {
    dae: Option<DaemonService>,
    exts: std::collections::HashMap<String, ExtService>,
}
impl Server {
    pub fn new() -> Self {
        Self {
            dae: None,
            exts: std::collections::HashMap::new(),
        }
    }
    pub async fn get_ext(&mut self, prompt: &String) -> Result<&mut ExtService, error::Error> {
        let ext = self
            .exts
            .entry(prompt.clone())
            .or_insert(Service::with_addr(
                self.dae
                    .as_mut()
                    .unwrap()
                    .request(daemon::Prompt {
                        content: prompt.clone(),
                    })
                    .await
                    .map_err(|e| error::Error::new(format!("get ext port failed: {}", e)))?,
            )?);
        ext.check_connected().await?;
        Ok(ext)
    }
    pub async fn connet(&mut self, ep: tonic::transport::Endpoint) -> Result<(), error::Error> {
        log::debug(format!("connect to {:#?}", ep.uri()).as_str());
        let mut dae = DaemonService::new(ep.clone());
        dae.check_connected()
            .await
            .map_err(|e| error::Error::new(format!("connect to daemon failed: {}", e)))?;
        let c = dae
            .request(crate::rpc::defs::Empty {})
            .await
            .map_err(|e| error::Error::new(format!("get ext port failed: {}", e)))?
            .into_iter()
            .flat_map(|(k, v)| ExtService::with_addr(v).map_or(None, |e| Some((k, e))));
        self.exts.extend(c);
        self.dae = Some(dae);
        Ok(())
    }
}
