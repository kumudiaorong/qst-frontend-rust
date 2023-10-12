use crate::rpc::{
    defs::Status,
    service::{Error, IntoResult},
};
impl IntoResult<std::collections::HashMap<String, String>> for super::SetUpResult {
    fn into_result(self) -> Result<std::collections::HashMap<String, String>, Error> {
        match Status::from_i32(self.status).unwrap() {
            Status::Ok => Ok(self.running),
            Status::Error => return Err("search executed but failed".into()),
        }
    }
}
impl IntoResult<String> for super::ExtAddrResult {
    fn into_result(self) -> Result<String, Error> {
        match Status::from_i32(self.status).unwrap() {
            Status::Ok => Ok(self.addr),
            Status::Error => return Err("submit executed but failed".into()),
        }
    }
}
