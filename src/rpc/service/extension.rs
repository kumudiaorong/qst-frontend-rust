use super::def;
use super::{utils::BoxFuture, Client as AClient, Request, Service as TService};
use def::{
    common::Empty,
    extension::{main_client, DisplayList, Input, SubmitHint},
};

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
