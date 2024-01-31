use std::pin::Pin;
use tonic::transport;
pub type BoxFuture<'a, T> = Pin<
    Box<dyn std::future::Future<Output = Result<tonic::Response<T>, tonic::Status>> + Send + 'a>,
>;
pub async fn try_connect(
    max_try: usize,
    endpoint: transport::Endpoint,
) -> Result<transport::Channel, transport::Error> {
    let mut cnt = 0;
    while cnt < max_try - 1 {
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
