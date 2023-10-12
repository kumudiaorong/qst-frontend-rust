use super::Client;
use crate::rpc::service::request::{BoxFuture, Request};

impl Request<Client, super::SetUpResult> for crate::rpc::defs::Empty {
    fn action(&self) -> &'static str {
        "Search"
    }
    fn request(self, cli: &mut Client) -> BoxFuture<'_, super::SetUpResult> {
        Box::pin(cli.set_up(self))
    }
}
impl Request<Client, super::ExtAddrResult> for super::Prompt {
    fn action(&self) -> &'static str {
        "Submit"
    }
    fn request(self, cli: &mut Client) -> BoxFuture<'_, super::ExtAddrResult> {
        Box::pin(cli.get_ext_addr(self))
    }
}
