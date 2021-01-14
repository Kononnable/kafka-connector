use tokio::net::TcpStream;

pub struct LibraryState {
    pub clients: Vec<Client>,
    pub state: KafkaState,
}

pub struct Client {
    pub connection: TcpStream,
}

pub struct KafkaState {}
