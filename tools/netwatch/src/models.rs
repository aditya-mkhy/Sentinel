use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Connection {
    pub pid: u32,
    pub process: String,
    pub local_addr: String,
    pub remote_addr: String,
    pub domain: String,
}
