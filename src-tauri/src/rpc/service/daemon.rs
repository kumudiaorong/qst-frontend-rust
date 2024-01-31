use super::def;
use super::{utils::BoxFuture, Client, Request, Service as TService};
pub use def::daemon::FastConfig;
use def::{
    common::Empty,
    daemon::{main_client, ExtAddr, ExtId},
};
type DaemonClient = main_client::MainClient<tonic::transport::Channel>;

pub type Service = TService<DaemonClient>;

impl Client for DaemonClient {
    fn new(cli: tonic::transport::Channel) -> Self {
        Self::new(cli)
    }
}

pub type RequestSetup = Empty;

impl Request<DaemonClient, FastConfig> for RequestSetup {
    fn action(&self) -> &'static str {
        "Search"
    }
    fn request(self, cli: &mut DaemonClient) -> BoxFuture<'_, FastConfig> {
        Box::pin(cli.set_up(self))
    }
}
pub type RequestExtAddr = ExtId;

impl Request<DaemonClient, ExtAddr> for RequestExtAddr {
    fn action(&self) -> &'static str {
        "Submit"
    }
    fn request(self, cli: &mut DaemonClient) -> BoxFuture<'_, ExtAddr> {
        Box::pin(cli.get_ext_addr(self))
    }
}
