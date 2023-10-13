tonic::include_proto!("daemon");
use super::{
    defs::Status, utils::BoxFuture, Client as AClient, Empty, Error, IntoResult, Request,
    Service as TService,
};
type Client = main_interact_client::MainInteractClient<tonic::transport::Channel>;
pub type Service = TService<Client>;
impl AClient for Client {
    fn new(cli: tonic::transport::Channel) -> Self {
        Self::new(cli)
    }
}
impl Request<Client, SetUpResult> for Empty {
    fn action(&self) -> &'static str {
        "Search"
    }
    fn request(self, cli: &mut Client) -> BoxFuture<'_, SetUpResult> {
        Box::pin(cli.set_up(self))
    }
}
impl Request<Client, ExtAddrResult> for Prompt {
    fn action(&self) -> &'static str {
        "Submit"
    }
    fn request(self, cli: &mut Client) -> BoxFuture<'_, ExtAddrResult> {
        Box::pin(cli.get_ext_addr(self))
    }
}
impl IntoResult<std::collections::HashMap<String, String>> for SetUpResult {
    fn into_result(self) -> Result<std::collections::HashMap<String, String>, Error> {
        match Status::from_i32(self.status).unwrap() {
            Status::Ok => Ok(self.running),
            Status::Error => return Err("search executed but failed".into()),
        }
    }
}
impl IntoResult<String> for ExtAddrResult {
    fn into_result(self) -> Result<String, Error> {
        match Status::from_i32(self.status).unwrap() {
            Status::Ok => Ok(self.addr),
            Status::Error => return Err("submit executed but failed".into()),
        }
    }
}
