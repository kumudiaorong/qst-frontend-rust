tonic::include_proto!("defs");
use super::{Error, IntoResult};
impl IntoResult<()> for MResult {
    fn into_result(self) -> std::result::Result<(), Error> {
        use m_result::Mresult;
        match self.mresult.unwrap() {
            Mresult::Ok(_) => Ok(()),
            Mresult::Status(status) => return Err("submit executed but failed".into()),
        }
    }
}
