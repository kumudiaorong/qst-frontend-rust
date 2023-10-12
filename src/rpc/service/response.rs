pub trait IntoResult<T> {
    fn into_result(self) -> Result<T, super::error::Error>;
}
pub fn convert<T, U>(
    result: Result<tonic::Response<T>, tonic::Status>,
) -> Result<U, super::error::Error>
where
    T: IntoResult<U>,
{
    // Wrapper(result).into_result()
    match result {
        Ok(resp) => resp.into_inner().into_result(),
        Err(status) => Err(status.into()),
    }
}

// impl IntoResult for Result<tonic::Response<super::SearchResult>, tonic::Status> {
//     fn into_result(self) -> Result<Response, super::error::Error> {
//         match self {
//             Ok(resp) => {
//                 let inner = resp.into_inner();
//                 // super::utils::match_grpc_result("search", Ok(self), |resp| resp.status, |resp| resp)?;
//                 match crate::rpc::defs::Status::from_i32(inner.status).unwrap() {
//                     crate::rpc::defs::Status::Ok => {
//                         Ok(Response::Search(inner.display_list.unwrap().list))
//                     }
//                     crate::rpc::defs::Status::Error => {
//                         return Err("search executed but failed".into())
//                     }
//                 }
//             }
//             Err(status) => Err(status.into()),
//         }
//     }
// }
// impl IntoResult for tonic::Response<super::SubmitResult> {
//     fn into_result(self) -> Result<Response, super::error::Error> {
//         let inner = self.into_inner();
//         // super::utils::match_grpc_result("submit", Ok(self), |resp| resp.status, |resp| resp)?;
//         match crate::rpc::defs::Status::from_i32(inner.status).unwrap() {
//             crate::rpc::defs::Status::Ok => Ok(Response::Submit),
//             crate::rpc::defs::Status::Error => return Err("submit executed but failed".into()),
//         }
//     }
// }
