pub trait Request<C, T> {
    fn action(&self) -> &'static str;
    fn request(self, cli: &mut C) -> super::utils::BoxFuture<'_, T>;
}
