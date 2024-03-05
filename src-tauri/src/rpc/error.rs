#[derive(Debug, Clone)]
pub enum ErrorKind{
    Unknown,
    Abort,
}
#[derive(Debug, Clone)]
pub struct Error {
    pub kind: ErrorKind,
    pub msg: String,
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.msg.as_str())
    }
}
impl From<tonic::transport::Error> for Error {
    fn from(e: tonic::transport::Error) -> Self {
        Self {
            kind: ErrorKind::Unknown,
            msg: e.to_string(),
        }
    }
}
impl Error {
    pub fn new(kind: ErrorKind, msg: &str) -> Self {
        Self {
            kind,
            msg: msg.to_string(),
        }
    }
    pub fn unknown(msg: &str) -> Self {
        Self {
            kind: ErrorKind::Unknown,
            msg: msg.to_string(),
        }
    }
    pub fn abort(msg: &str) -> Self {
        Self {
            kind: ErrorKind::Abort,
            msg: msg.to_string(),
        }
    }
}
