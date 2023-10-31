use super::super::def;
use super::{utils::BoxFuture, Client as AClient, Error, IntoResult, Request, Service as TService};
use def::common::Empty;
use def::extension::main_client;
use def::extension::DisplayItem;
use def::extension::DisplayList;
use def::extension::Input;
use def::extension::SubmitHint;

type Client = main_client::MainClient<tonic::transport::Channel>;
pub type Service = TService<Client>;

impl AClient for Client {
    fn new(cli: tonic::transport::Channel) -> Self {
        Self::new(cli)
    }
}
pub type RequestSearch = Input;
impl Request<Client, DisplayList> for RequestSearch {
    fn action(&self) -> &'static str {
        "Search"
    }
    fn request(self, cli: &mut Client) -> BoxFuture<'_, DisplayList> {
        Box::pin(cli.search(self))
    }
}
pub type RequestSubmit = SubmitHint;
impl Request<Client, Empty> for RequestSubmit {
    fn action(&self) -> &'static str {
        "Submit"
    }
    fn request(self, cli: &mut Client) -> BoxFuture<'_, Empty> {
        Box::pin(cli.submit(self))
    }
}

// impl IntoResult<Vec<DisplayItem>> for SearchResult {
//     fn into_result(self) -> Result<Vec<DisplayItem>, Error> {
//         use search_result::{MOk, Mresult};
//         match self.mresult.unwrap() {
//             Mresult::Ok(MOk { display_list }) => Ok(display_list.unwrap().list),
//             Mresult::Status(status) => return Err("search executed but failed".into()),
//         }
//     }
// }
