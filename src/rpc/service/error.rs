#[derive(Debug, Clone)]
pub struct Error {
    pub msg: String,
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.msg.as_str())
    }
}
impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Self::new(s.to_string())
    }
}
impl From<String> for Error {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}
impl From<tonic::Status> for Error {
    fn from(s: tonic::Status) -> Self {
        Self::new(s.to_string())
    }
}
impl Error {
    pub fn new(msg: String) -> Self {
        Self { msg }
    }
}
