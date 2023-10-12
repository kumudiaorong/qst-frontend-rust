mod error;
pub mod request;
pub mod response;
tonic::include_proto!("daemon");
use super::utils;
use super::MAX_TRY_CONNECT;
pub use error::Error;
pub use request::BoxFuture;
pub use request::Request;
pub use response::convert;
pub use response::IntoResult;
enum Inner<C> {
    Ready(tonic::transport::Endpoint),
    Connected(C),
}
pub trait Client {
    fn new(cli: tonic::transport::Channel) -> Self;
}
pub struct Service<C: Client> {
    inner: Inner<C>,
}
impl<C: Client> Service<C> {
    pub fn new(endpoint: tonic::transport::Endpoint) -> Self {
        Self {
            inner: Inner::Ready(endpoint),
        }
    }
    pub fn with_addr(addr: String) -> Result<Self, error::Error> {
        Ok(Self::new(
            tonic::transport::Endpoint::from_shared(format!("http://{}", addr)).map_err(|e| {
                xlog_rs::log::warn(
                    format!("can't create endpoint with addr: {}, Err: {}", addr, e).as_str(),
                );
                error::Error::new(format!("can't create endpoint {:#?}", e))
            })?,
        ))
    }
    pub async fn check_connected(&mut self) -> Result<(), error::Error> {
        if let Inner::Ready(ep) = &mut self.inner {
            self.inner = Inner::Connected(C::new(
                utils::try_connect(MAX_TRY_CONNECT, ep.clone())
                    .await
                    .map_err(|e| error::Error::new(format!("can't create endpoint {:#?}", e)))?,
            ));
        }
        Ok(())
    }
    pub async fn request<T, U>(&mut self, req: impl Request<C, T>) -> Result<U, error::Error>
    where
        T: response::IntoResult<U>,
    {
        if let Inner::Connected(cli) = &mut self.inner {
            response::convert(req.request(cli).await)
        } else {
            unreachable!("check_connected should have connected and return Ok")
        }
    }
}
