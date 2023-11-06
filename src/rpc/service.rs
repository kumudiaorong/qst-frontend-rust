mod daemon;
mod def;
mod error;
mod extension;
mod request;
mod utils;
const MAX_TRY_CONNECT: usize = 3;

pub use daemon::{FastConfig, RequestExtAddr, RequestSetup, Service as DaemonService};
pub use error::Error;
pub use extension::{RequestSearch, RequestSubmit, Service as ExtService};
use request::Request;
use tonic::{transport::Endpoint, Code};
use xlog_rs::log;

enum Inner<C> {
    Ready(Endpoint),
    Connected(C),
}
pub trait Client {
    fn new(cli: tonic::transport::Channel) -> Self;
}
pub struct Service<C: Client> {
    inner: Inner<C>,
}
impl<C: Client> Service<C> {
    pub fn new(endpoint: Endpoint) -> Self {
        Self {
            inner: Inner::Ready(endpoint),
        }
    }
    pub fn with_addr(addr: String) -> Result<Self, Error> {
        Ok(Self::new(
            Endpoint::from_shared(format!("http://{}", addr)).map_err(|e| {
                xlog_rs::log::warn(
                    format!("can't create endpoint with addr: {}, Err: {}", addr, e).as_str(),
                );
                Error::new(format!("can't create endpoint {:#?}", e))
            })?,
        ))
    }
    pub fn with_ep(ep: Endpoint) -> Self {
        Self::new(ep)
    }
    async fn check_connected(&mut self) -> Result<(), Error> {
        if let Inner::Ready(ep) = &mut self.inner {
            let channel = utils::try_connect(MAX_TRY_CONNECT, ep.clone())
                .await
                .map_err(|e| Error::new(format!("can't create endpoint {:#?}", e)))?;
            log::info(format!("connected to {:#?}", ep.uri()).as_str());
            self.inner = Inner::Connected(C::new(channel));
        }
        Ok(())
    }
    pub async fn request<T>(&mut self, req: impl Request<C, T> + Clone) -> Result<T, Error> {
        self.check_connected().await?;
        let cli = match &mut self.inner {
            Inner::Connected(cli) => cli,
            _ => unreachable!("check_connected should have connected and return Ok"),
        };
        loop {
            let ret = req.clone().request(cli).await.map(|v| v.into_inner());
            match ret {
                Ok(v) => break Ok(v),
                Err(status) => {
                    if status.code() != Code::Unavailable {
                        break Err(Error::new(format!("request failed: {}", status)));
                    }
                }
            }
        }
    }
}
