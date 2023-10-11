tonic::include_proto!("daemon");
use super::error::Error;
use super::utils;
pub struct DaemonService {
    inner: main_interact_client::MainInteractClient<tonic::transport::Channel>,
}
impl DaemonService {
    pub async fn connect(
        max_try: u32,
        endpoint: tonic::transport::Endpoint,
    ) -> Result<Self, super::error::Error> {
        Ok(Self {
            inner: main_interact_client::MainInteractClient::new(
                utils::try_connect(max_try, endpoint).await?,
            ),
        })
    }
    pub async fn set_up(&mut self) -> Result<std::collections::HashMap<String, String>, Error> {
        utils::match_grpc_result(
            "set up",
            self.inner.set_up(super::defs::Empty {}).await,
            |resp| resp.status,
            |resp| resp.running,
        )
    }
    pub async fn get_ext_port(&mut self, prompt: &str) -> Result<String, Error> {
        utils::match_grpc_result(
            "get ext port",
            self.inner
                .get_ext_addr(Prompt {
                    content: prompt.to_string(),
                })
                .await,
            |resp| resp.status,
            |resp| resp.addr,
        )
    }
}
