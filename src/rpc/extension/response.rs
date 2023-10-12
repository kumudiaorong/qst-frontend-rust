use crate::rpc::{
    defs::Status,
    service::{Error, IntoResult},
};
impl IntoResult<Vec<super::Display>> for super::SearchResult {
    fn into_result(self) -> Result<Vec<super::Display>, Error> {
        match Status::from_i32(self.status).unwrap() {
            Status::Ok => Ok(self.display_list.unwrap().list),
            Status::Error => return Err("search executed but failed".into()),
        }
    }
}
impl IntoResult<()> for super::SubmitResult {
    fn into_result(self) -> Result<(), Error> {
        match Status::from_i32(self.status).unwrap() {
            Status::Ok => Ok(()),
            Status::Error => return Err("submit executed but failed".into()),
        }
    }
}
