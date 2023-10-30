mod daemon;
mod error;
mod extension;
mod request;
mod response;
mod utils;
const MAX_TRY_CONNECT: usize = 3;

pub use daemon::{Prompt, Service as DaemonService};
pub use defs::Empty;
pub use error::Error;
pub use extension::{Input, Service as ExtService, SubmitHint};
use request::Request;
use response::IntoResult;
use tonic::transport::Endpoint;
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
    pub async fn check_connected(&mut self) -> Result<(), Error> {
        if let Inner::Ready(ep) = &mut self.inner {
            self.inner = Inner::Connected(C::new(
                utils::try_connect(MAX_TRY_CONNECT, ep.clone())
                    .await
                    .map_err(|e| Error::new(format!("can't create endpoint {:#?}", e)))?,
            ));
        }
        Ok(())
    }
    pub async fn request<T, U>(&mut self, req: impl Request<C, T>) -> Result<U, Error>
    where
        T: response::IntoResult<U>,
    {
        self.check_connected().await?;
        let cli = match &mut self.inner {
            Inner::Connected(cli) => cli,
            _ => unreachable!("check_connected should have connected and return Ok"),
        };
        response::convert(req.request(cli).await)
    }
}
