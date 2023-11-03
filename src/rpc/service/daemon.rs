use super::super::def;
use super::{utils::BoxFuture, Client as AClient, Request, Service as TService};
use def::common::Empty;
use def::daemon::main_client;
pub use def::daemon::FastConfig;
use def::daemon::{ExtAddr, ExtId};
type Client = main_client::MainClient<tonic::transport::Channel>;
pub type Service = TService<Client>;
impl AClient for Client {
    fn new(cli: tonic::transport::Channel) -> Self {
        Self::new(cli)
    }
}

pub type RequestSetup = Empty;

impl Request<Client, FastConfig> for RequestSetup {
    fn action(&self) -> &'static str {
        "Search"
    }
    fn request(self, cli: &mut Client) -> BoxFuture<'_, FastConfig> {
        Box::pin(cli.set_up(self))
    }
}
pub type RequestExtAddr = ExtId;
impl Request<Client, ExtAddr> for RequestExtAddr {
    fn action(&self) -> &'static str {
        "Submit"
    }
    fn request(self, cli: &mut Client) -> BoxFuture<'_, ExtAddr> {
        Box::pin(cli.get_ext_addr(self))
    }
}
