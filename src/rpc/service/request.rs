use std::pin::Pin;
pub type BoxFuture<'a, T> = Pin<
    Box<dyn std::future::Future<Output = Result<tonic::Response<T>, tonic::Status>> + Send + 'a>,
>;
pub trait Request<C, T> {
    fn action(&self) -> &'static str;
    fn request(self, cli: &mut C) -> BoxFuture<'_, T>;
}
