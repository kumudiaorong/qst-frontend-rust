mod error;
mod service;
pub use error::Error;
use service::{DaemonService, ExtService, RequestExtAddr, RequestSetup};
pub use service::{DisplayList, RequestSearch, RequestSubmit};
use std::collections::{hash_map::Entry, HashMap};
use xlog::{debug, error, info, warn};
pub struct Server {
    dae: DaemonService,
    exts: std::collections::HashMap<String, ExtService>,
    by_prompt: std::collections::HashMap<String, String>,
}
impl Server {
    pub async fn get_ext(&mut self, prompt: &str) -> Option<&mut ExtService> {
        let id = self.by_prompt.get(prompt)?;
        let s = match self.exts.entry(id.clone()) {
            Entry::Vacant(e) => {
                let addr = self
                    .dae
                    .request(RequestExtAddr { id: id.clone() })
                    .await
                    .map_or_else(
                        |e| {
                            error!("get ext port failed: {}", e);
                            None
                        },
                        |e| {
                            debug!("get ext port successful: {}", e.addr);
                            Some(e.addr)
                        },
                    )?;
                debug!("start connected to {}:{}", prompt, addr);
                let service = ExtService::with_addr(&addr).await.ok()?;
                info!("connected to {}:{}", prompt, addr);
                e.insert(service)
            }
            Entry::Occupied(e) => e.into_mut(),
        };
        Some(s)
    }

    pub async fn connect(ep: tonic::transport::Endpoint) -> Result<Self, error::Error> {
        debug!("start connect to {:#?}", ep.uri());
        //create daemon service
        let mut dae = DaemonService::new(&ep).await?;
        info!("connected to server:{}", ep.uri());
        //request for fast config
        let fcfg = dae
            .request(RequestSetup {})
            .await
            .map_err(|e| error::Error::new(format!("get ext port failed: {}", e)))?;
        //get extension servers with fast config
        let mut exts = HashMap::new();
        let mut by_prompt = HashMap::new();
        for (id, v) in fcfg.fexts.into_iter() {
            debug!(
                "Ext{{Id:{},\tPrompt:{},\tAddr:{}}}",
                id,
                v.prompt,
                v.addr.clone().unwrap_or("".to_string())
            );
            if let Some(addr) = v.addr {
                let _ = warn!(
                    res,
                    ExtService::with_addr(&addr).await.map(|service| {
                        exts.insert(id.clone(), service);
                    }),
                    "connect to ext failed: {}",
                    addr
                );
            }
            by_prompt.insert(v.prompt, id);
        }
        Ok(Self {
            dae,
            exts,
            by_prompt,
        })
    }
}
