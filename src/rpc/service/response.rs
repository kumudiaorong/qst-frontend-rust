use super::error::Error;
pub trait IntoResult<T> {
    fn into_result(self) -> Result<T, Error>;
}

impl<T> IntoResult<T> for Result<tonic::Response<T>, tonic::Status> {
    fn into_result(self) -> Result<T, Error> {
        match self {
            Ok(resp) => resp.into_inner().into_result(),
            Err(status) => Err(status.into()),
        }
    }
}

pub fn convert<T, U>(result: Result<tonic::Response<T>, tonic::Status>) -> Result<U, Error>
where
    T: IntoResult<U>,
{
    // Wrapper(result).into_result()
    match result {
        Ok(resp) => resp.into_inner().into_result(),
        Err(status) => Err(status.into()),
    }
}
