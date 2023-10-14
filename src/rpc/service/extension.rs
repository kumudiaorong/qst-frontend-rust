tonic::include_proto!("ext");
use super::{
    defs::{status, MResult},
    utils::BoxFuture,
    Client as AClient, Error, IntoResult, Request, Service as TService,
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
impl Request<Client, MResult> for SubmitHint {
    fn action(&self) -> &'static str {
        "Submit"
    }
    fn request(self, cli: &mut Client) -> BoxFuture<'_, MResult> {
        Box::pin(cli.submit(self))
    }
}

impl IntoResult<Vec<Display>> for SearchResult {
    fn into_result(self) -> Result<Vec<Display>, Error> {
        use search_result::{MOk, Mresult};
        match self.mresult.unwrap() {
            Mresult::Ok(MOk { display_list }) => Ok(display_list.unwrap().list),
            Mresult::Status(status) => return Err("search executed but failed".into()),
        }
    }
}
