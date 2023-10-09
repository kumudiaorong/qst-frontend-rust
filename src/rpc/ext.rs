tonic::include_proto!("ext");
use super::utils;
pub struct ExtService {
    inner: ext_interact_client::ExtInteractClient<tonic::transport::Channel>,
}
impl ExtService {
    pub async fn connect(
        max_try: u32,
        endpoint: tonic::transport::Endpoint,
    ) -> Result<Self, super::error::Error> {
        Ok(Self {
            inner: ext_interact_client::ExtInteractClient::new(
                utils::try_connect(max_try, endpoint).await?,
            ),
        })
    }
    pub async fn with_port(max_try: u32, addr: String) -> Result<Self, super::error::Error> {
        Self::connect(
            max_try,
            tonic::transport::Endpoint::from_shared(format!("http://{}", addr))
                .map_err(|e| super::error::Error::new(format!("can't create endpoint {:?}", e)))?,
        )
        .await
    }
    pub async fn search(&mut self, input: Input) -> Result<Vec<Display>, super::error::Error> {
        utils::match_grpc_result(
            "search",
            self.inner.search(input).await,
            |resp| resp.status,
            |resp| resp.display_list.unwrap().list,
        )
    }
    pub async fn submit(&mut self, hint: SubmitHint) -> Result<(), super::error::Error> {
        utils::match_grpc_result(
            "submit",
            self.inner.submit(hint).await,
            |resp| resp.status,
            |_| (),
        )
    }
    // pub async fn fill(&mut self, obj_id: u32) -> Result<String, super::error::Error> {
    //     utils::match_grpc_result(
    //         "fill",
    //         self.inner.fill(FillHint { obj_id }).await,
    //         |resp| resp.status,
    //         |resp| resp.content,
    //     )
    // }
}
