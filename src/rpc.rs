mod def;
pub mod error;
pub use error::Error;
mod service;
use std::collections::HashMap;

pub use service::Service;
use service::{DaemonService, ExtService, RequestExtAddr, RequestSetup};
pub use service::{RequestSearch, RequestSubmit};
use xlog_rs::log;
pub struct Server {
    dae: DaemonService,
    exts: std::collections::HashMap<String, ExtService>,
    by_prompt: std::collections::HashMap<String, String>,
}
impl Server {
    pub async fn get_ext(&mut self, prompt: &str) -> Option<&mut ExtService> {
        let id = self.by_prompt.get(prompt)?;
        Some(
            self.exts.entry(id.clone()).or_insert(
                ExtService::with_addr(
                    self.dae
                        .request(RequestExtAddr { id: id.to_string() })
                        .await
                        .map_or_else(
                            |e| {
                                log::error(format!("get ext port failed: {}", e).as_str());
                                return None;
                            },
                            |e| Some(e.addr),
                        )?,
                )
                .map_or_else(
                    |e| {
                        log::error(format!("get ext port failed: {}", e).as_str());
                        return None;
                    },
                    |e| Some(e),
                )?,
            ),
        )
    }

    pub async fn connet(ep: tonic::transport::Endpoint) -> Result<Self, error::Error> {
        log::debug(format!("connect to {:#?}", ep.uri()).as_str());
        let mut dae = DaemonService::new(ep.clone());
        dae.check_connected()
            .await
            .map_err(|e| error::Error::new(format!("connect to daemon failed: {}", e)))?;
        let fcfg = dae
            .request(RequestSetup {})
            .await
            .map_err(|e| error::Error::new(format!("get ext port failed: {}", e)))?;
        let (exts, by_prompt): (Vec<Option<_>>, HashMap<_, _>) = fcfg
            .fexts
            .into_iter()
            .map(|(k, v)| {
                (
                    v.addr.and_then(|addr| {
                        ExtService::with_addr(addr).map_or(None, |e| Some((k.clone(), e)))
                    }),
                    (v.prompt, k),
                )
            })
            .unzip();
        // self.exts.extend(exts.into_iter().filter_map(|e| e));
        // self.by_prompt.extend(by_prompt);
        // Ok(())
        Ok(Self {
            dae: dae,
            exts: exts.into_iter().filter_map(|e| e).collect(),
            by_prompt: by_prompt,
        })
    }
}
