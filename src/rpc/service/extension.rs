tonic::include_proto!("extension");
use super::{
    utils::BoxFuture, Client as AClient, Empty, Error, IntoResult, Request, Service as TService,
};

type Client = main_client::MainClient<tonic::transport::Channel>;
pub type Service = TService<Client>;

impl AClient for Client {
    fn new(cli: tonic::transport::Channel) -> Self {
        Self::new(cli)
    }
}

impl Request<Client, DisplayList> for Input {
    fn action(&self) -> &'static str {
        "Search"
    }
    fn request(self, cli: &mut Client) -> BoxFuture<'_, DisplayList> {
        Box::pin(cli.search(self))
    }
}
impl Request<Client, Empty> for SubmitHint {
    fn action(&self) -> &'static str {
        "Submit"
    }
    fn request(self, cli: &mut Client) -> BoxFuture<'_, Empty> {
        Box::pin(cli.submit(self))
    }
}

impl IntoResult<Vec<DisplayItem>> for SearchResult {
    fn into_result(self) -> Result<Vec<DisplayItem>, Error> {
        use search_result::{MOk, Mresult};
        match self.mresult.unwrap() {
            Mresult::Ok(MOk { display_list }) => Ok(display_list.unwrap().list),
            Mresult::Status(status) => return Err("search executed but failed".into()),
        }
    }
}
