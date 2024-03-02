mod daemon;
mod def;
mod error;
mod extension;
mod request;
mod utils;

pub use daemon::{RequestExtAddr, RequestSetup, Service as DaemonService};
pub use def::extension::DisplayList;
pub use error::Error;
pub use extension::{RequestSearch, RequestSubmit, Service as ExtService};
use request::Request;
use tonic::{transport::Endpoint, Code};
use xlog::debug;

pub trait Client {
    fn new(cli: tonic::transport::Channel) -> Self;
}
#[derive(Debug, Clone)]
pub struct Service<C: Client> {
    inner: C,
}
impl<C: Client> Service<C> {
    pub async fn new(endpoint: &Endpoint) -> Result<Self, Error> {
        let channel = utils::try_connect(endpoint.clone())
            .await
            .map_err(|e| Error::new(format!("endpoint can't connect {:#?}", e)))?;
        Ok(Self {
            inner: C::new(channel),
        })
    }
    pub async fn with_addr(addr: &str) -> Result<Self, Error> {
        debug!("start connect to {}", addr);
        let ep = Endpoint::from_shared(addr.to_string()).map_err(|e| {
            xlog::warn!("can't create endpoint with addr: {}, Err: {}", addr, e);
            Error::new(format!("can't create endpoint {:#?}", e))
        })?;
        Self::new(&ep).await
    }
    pub async fn request<T>(&mut self, req: impl Request<C, T> + Clone) -> Result<T, Error> {
        loop {
            let ret = req
                .clone()
                .request(&mut self.inner)
                .await
                .map(|v| v.into_inner());
            match ret {
                Ok(v) => break Ok(v),
                Err(status) => {
                    if status.code() == Code::FailedPrecondition {
                        match status.message() {
                            "retry" => continue,
                            "wait" => {
                                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                                continue;
                            }
                            _ => {}
                        };
                    }
                    break Err(Error::new(format!("request failed: {}", status)));
                }
            }
        }
    }
}
