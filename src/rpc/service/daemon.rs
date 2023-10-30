use super::super::def;
use super::{utils::BoxFuture, Client as AClient, Error, IntoResult, Request, Service as TService};
use def::common::Empty;
use def::daemon::main_client;
use def::daemon::{ExtAddr, ExtId, Prompt, Prompt2Addr};
type Client = main_client::MainClient<tonic::transport::Channel>;
pub type Service = TService<Client>;
impl AClient for Client {
    fn new(cli: tonic::transport::Channel) -> Self {
        Self::new(cli)
    }
}

type SetupRequest = Empty;

impl Request<Client, Prompt2Addr> for SetupRequest {
    fn action(&self) -> &'static str {
        "Search"
    }
    fn request(self, cli: &mut Client) -> BoxFuture<'_, Prompt2Addr> {
        Box::pin(cli.set_up(self))
    }
}
impl Request<Client, ExtAddr> for ExtId {
    fn action(&self) -> &'static str {
        "Submit"
    }
    fn request(self, cli: &mut Client) -> BoxFuture<'_, ExtAddr> {
        Box::pin(cli.get_ext_addr(self))
    }
}
