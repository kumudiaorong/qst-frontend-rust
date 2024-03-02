use std::pin::Pin;
use tonic::transport;
use xlog::debug;
pub type BoxFuture<'a, T> = Pin<
    Box<dyn std::future::Future<Output = Result<tonic::Response<T>, tonic::Status>> + Send + 'a>,
>;
const MAX_TRY_CONNECT: usize = 3;
const MAX_CONNECT_TIMEOUT: u64 = 100;

pub async fn try_connect(
    mut endpoint: transport::Endpoint,
) -> Result<transport::Channel, transport::Error> {
    debug!("try connect to {:#?}", endpoint.uri());
    let mut cnt = 0;
    endpoint = endpoint.connect_timeout(std::time::Duration::from_millis(MAX_CONNECT_TIMEOUT));
    while cnt < MAX_TRY_CONNECT - 1 {
        match endpoint.connect().await {
            Ok(c) => {
                return Ok(c);
            }
            Err(_) => {
                cnt += 1;
                tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
            }
        }
    }
    endpoint.connect().await
}
