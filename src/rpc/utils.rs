use super::defs;
use super::error::Error;
use xlog_rs::log;
pub async fn try_connect(
    max_try: u32,
    endpoint: tonic::transport::Endpoint,
) -> Result<tonic::transport::Channel, Error> {
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
    endpoint
        .connect()
        .await
        .map_err(|e| Error::new(format!("connect server failed: {}", e)))
}
pub fn match_grpc_result<T, U>(
    action: &str,
    result: Result<tonic::Response<T>, tonic::Status>,
    get_status: impl FnOnce(&T) -> i32,
    handle_content: impl FnOnce(T) -> U,
) -> Result<U, Error> {
    match result {
        Ok(res) => {
            let inner = res.into_inner();
            match defs::Status::from_i32(get_status(&inner)).unwrap() {
                defs::Status::Ok => Ok(handle_content(inner)),
                defs::Status::Error => Err(Error::new(format!("{} executed but failed", action))),
            }
        }
        Err(e) => {
            log::error(format!("{} failed: {}", action, e).as_str());
            Err(Error::new(format!("{} failed", action)))
        }
    }
}
