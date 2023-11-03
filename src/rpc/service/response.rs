use super::error::Error;
pub trait IntoResult<T> {
    fn into_result(self) -> Result<T, Error>;
}
