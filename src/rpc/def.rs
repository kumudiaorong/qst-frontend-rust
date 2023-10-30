pub mod daemon {
    tonic::include_proto!("daemon");
}
pub mod common {
    tonic::include_proto!("common");
}
pub mod extension {
    tonic::include_proto!("extension");
}
pub enum Request {
    SetUp,
    Search,
    Submit,
}
