#[derive(Debug, Clone)]
pub struct Error {
    pub msg: String,
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.msg.as_str())
    }
}
impl From<String> for Error {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}
impl From<super::service::Error> for Error {
    fn from(e: super::service::Error) -> Self {
        Self::new(e.msg)
    }
}
impl Error {
    pub fn new(msg: String) -> Self {
        Self { msg }
    }
}
