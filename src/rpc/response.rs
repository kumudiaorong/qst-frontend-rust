use super::extension;
// #[derive(Debug, Clone)]
// pub enum Response {
//     Connected,
//     Search(Vec<extension::Display>),
//     Submit,
//     // FillResult(String),
// }
pub trait Response {
    fn ok(&self) -> bool;
}

impl Response for extension::SearchResult {
    fn ok(&self) -> bool {
        crate::rpc::defs::Status::from_i32(self.status).unwrap() == crate::rpc::defs::Status::Ok
    }
}
impl Response for extension::SubmitResult {
    fn ok(&self) -> bool {
        crate::rpc::defs::Status::from_i32(self.status).unwrap() == crate::rpc::defs::Status::Ok
    }
}
