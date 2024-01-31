use super::def;
use super::{utils::BoxFuture, Client, Request, Service as TService};
use def::{
    common::Empty,
    extension::{main_client, DisplayList, Input, SubmitHint},
};

type ExtClient = main_client::MainClient<tonic::transport::Channel>;

pub type Service = TService<ExtClient>;

impl Client for ExtClient {
    fn new(cli: tonic::transport::Channel) -> Self {
        Self::new(cli)
    }
}

pub type RequestSearch = Input;

impl Request<ExtClient, DisplayList> for RequestSearch {
    fn action(&self) -> &'static str {
        "Search"
    }
    fn request(self, cli: &mut ExtClient) -> BoxFuture<'_, DisplayList> {
        Box::pin(cli.search(self))
    }
}

pub type RequestSubmit = SubmitHint;

impl Request<ExtClient, Empty> for RequestSubmit {
    fn action(&self) -> &'static str {
        "Submit"
    }
    fn request(self, cli: &mut ExtClient) -> BoxFuture<'_, Empty> {
        Box::pin(cli.submit(self))
    }
}
