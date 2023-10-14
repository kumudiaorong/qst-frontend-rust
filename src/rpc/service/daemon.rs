tonic::include_proto!("daemon");
use super::{
    utils::BoxFuture, Client as AClient, Empty, Error, IntoResult, Request, Service as TService,
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
        use set_up_result::{MOk, Mresult};
        match self.mresult.unwrap() {
            Mresult::Ok(MOk { running }) => Ok(running),
            Mresult::Status(status) => return Err("search executed but failed".into()),
        }
    }
}
impl IntoResult<String> for ExtAddrResult {
    fn into_result(self) -> Result<String, Error> {
        use ext_addr_result::{MOk, Mresult};
        match self.mresult.unwrap() {
            Mresult::Ok(MOk { addr }) => Ok(addr),
            Mresult::Status(status) => return Err("submit executed but failed".into()),
        }
    }
}
