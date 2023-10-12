tonic::include_proto!("ext");
mod request;
mod response;
use crate::rpc::service;
type Client = ext_interact_client::ExtInteractClient<tonic::transport::Channel>;
pub type Service = service::Service<Client>;
impl service::Client for Client {
    fn new(cli: tonic::transport::Channel) -> Self {
        Self::new(cli)
    }
}
