tonic::include_proto!("daemon");
mod request;
mod response;
use crate::rpc::service;
type Client = main_interact_client::MainInteractClient<tonic::transport::Channel>;
pub type Service = service::Service<Client>;
impl service::Client for Client {
    fn new(cli: tonic::transport::Channel) -> Self {
        Self::new(cli)
    }
}
