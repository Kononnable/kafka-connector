use tokio::net::TcpStream;

pub struct LibraryState {
    pub clients: Vec<Client>,
    pub state: KafkaState,
}

pub struct Client {
    pub connection: TcpStream,
    pub last_correlation: i32,
}

pub struct KafkaState {}
