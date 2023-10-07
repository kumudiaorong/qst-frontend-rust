#[derive(Debug, Clone)]
pub struct Error {
    pub msg: String,
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.msg.as_str())
    }
}
impl Error {
    pub fn new(msg: String) -> Self {
        Self { msg }
    }
}
