tonic::include_proto!("ext");
use super::{
    defs::Status, utils::BoxFuture, Client as AClient, Error, IntoResult, Request,
    Service as TService,
};

type Client = ext_interact_client::ExtInteractClient<tonic::transport::Channel>;
pub type Service = TService<Client>;

impl AClient for Client {
    fn new(cli: tonic::transport::Channel) -> Self {
        Self::new(cli)
    }
}

impl Request<Client, SearchResult> for Input {
    fn action(&self) -> &'static str {
        "Search"
    }
    fn request(self, cli: &mut Client) -> BoxFuture<'_, SearchResult> {
        Box::pin(cli.search(self))
    }
}
impl Request<Client, SubmitResult> for SubmitHint {
    fn action(&self) -> &'static str {
        "Submit"
    }
    fn request(self, cli: &mut Client) -> BoxFuture<'_, SubmitResult> {
        Box::pin(cli.submit(self))
    }
}

impl IntoResult<Vec<Display>> for SearchResult {
    fn into_result(self) -> Result<Vec<Display>, Error> {
        match Status::from_i32(self.status).unwrap() {
            Status::Ok => Ok(self.display_list.unwrap().list),
            Status::Error => return Err("search executed but failed".into()),
        }
    }
}
impl IntoResult<()> for SubmitResult {
    fn into_result(self) -> Result<(), Error> {
        match Status::from_i32(self.status).unwrap() {
            Status::Ok => Ok(()),
            Status::Error => return Err("submit executed but failed".into()),
        }
    }
}
