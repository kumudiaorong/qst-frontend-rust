use super::Client;
use crate::rpc::service::{BoxFuture, Request};
impl Request<Client, super::SearchResult> for super::Input {
    fn action(&self) -> &'static str {
        "Search"
    }
    fn request(self, cli: &mut Client) -> BoxFuture<'_, super::SearchResult> {
        Box::pin(cli.search(self))
    }
}
impl Request<Client, super::SubmitResult> for super::SubmitHint {
    fn action(&self) -> &'static str {
        "Submit"
    }
    fn request(self, cli: &mut Client) -> BoxFuture<'_, super::SubmitResult> {
        Box::pin(cli.submit(self))
    }
}
