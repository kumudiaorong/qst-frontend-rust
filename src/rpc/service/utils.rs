use std::pin::Pin;
use xlog_rs::log;
pub type BoxFuture<'a, T> = Pin<
    Box<dyn std::future::Future<Output = Result<tonic::Response<T>, tonic::Status>> + Send + 'a>,
>;
pub async fn try_connect(
    max_try: usize,
    endpoint: tonic::transport::Endpoint,
) -> Result<tonic::transport::Channel, tonic::transport::Error> {
    log::info(format!("try connect server: {:#?}", endpoint.uri()).as_str());
    let mut cnt = 0;
    while cnt < max_try - 1 {
        match endpoint.connect().await {
            Ok(c) => {
                return Ok(c);
            }
            Err(e) => {
                cnt += 1;
                log::warn(format!("connect server failed: {}", e).as_str());
                tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
            }
        }
    }
    endpoint.connect().await.map_err(|e| {
        log::error(format!("connect server failed: {}", e).as_str());
        e
    })
}
