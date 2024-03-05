mod daemon;
mod def;
mod extension;
mod request;
mod utils;

use super::Error;
pub use daemon::{RequestExtAddr, RequestSetup, Service as DaemonService};
pub use def::extension::DisplayList;
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
            .map_err(Error::from)?;
        Ok(Self {
            inner: C::new(channel),
        })
    }
    pub async fn with_addr(addr: &str) -> Result<Self, Error> {
        debug!("start connect to {}", addr);
        let ep = Endpoint::from_shared(addr.to_string()).map_err(Error::from)?;
        Self::new(&ep).await
    }
    pub async fn request<T>(&mut self, req: impl Request<C, T> + Clone) -> Result<T, Error> {
        loop {
            let ret = req
                .clone()
                .request(&mut self.inner)
                .await
                .map(|v| v.into_inner());
            let status = match ret {
                Ok(v) => break Ok(v),
                Err(status) => status,
            };
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
            break Err(Error::unknown(status.message()));
        }
    }
}
