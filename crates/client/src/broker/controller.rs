#[derive(Copy, Clone, Debug)]
pub enum BrokerStatus {
    Initializing,
}
pub struct BrokerController {
    address: String,
    status: BrokerStatus,
}
impl BrokerController {
    pub fn new(host: &str, port: i32) -> BrokerController {
        BrokerController {
            address: format!("{host}:{port}"),
            status: BrokerStatus::Initializing,
        }
    }

    pub fn get_status(&self) -> BrokerStatus {
        self.status
    }
    pub fn get_address(&self) -> &str {
        &self.address
    }
}