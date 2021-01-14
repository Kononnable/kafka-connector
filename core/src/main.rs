use std::{collections::binary_heap::Iter, fmt};

use bytes::{BufMut, Bytes, BytesMut};
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use kafka_connector_derive::KafkaSerialize;
use kafka_connector_derive::KafkaDeserialize;

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    println!("Hello, world!");

    let mut stream = TcpStream::connect("127.0.0.1:9092").await?;
    println!("After connect");

    let topics: Vec<String> = Vec::new();
    let req = MetadataRequest::new(1, "my-client".to_string(), topics);

    // ~ buffer to receive data to be sent
    let mut buffer = BytesMut::with_capacity(4096);
    // ~ reserve bytes for the actual request size (we'll fill in that later)
    // buffer.extend_from_slice(&[0, 0, 0, 0]);
    // ~ encode the request data
    // req.encode(&mut buffer);
    req.serialize(&mut buffer);
    // try!(request.encode(&mut buffer));
    // ~ put the size of the request data into the reseved area
    // let size = buffer.len() as i32 - 4;
    // size.encode(&mut &mut buffer[..])?;

    // trace!("__send_request: Sending bytes: {:?}", &buffer);

    // ~ send the prepared buffer
    println!("Before write");
    let len = buffer.len() as i32;
    println!("Writing len {:?}", len.to_be_bytes());

    stream.write_all(&len.to_be_bytes()).await?;
    println!("Writing buff {:?}", buffer);
    stream.write_all(&buffer).await?;
    println!("Before read");
    // match __send_request(conn, req) {
    //     Ok(_) => return __get_response::<protocol::MetadataResponse>(conn),
    //     Err(e) => debug!(
    //         "fetch_metadata: failed to request metadata from {}: {}",
    //         host, e
    //     ),
    // }
    let mut size: [u8; 4] = [0, 0, 0, 0];
    let read = stream.read_exact(&mut size).await?;
    println!("Read {} bytes: {:?}", read, size);

    let cap = i32::from_be_bytes(size);
    println!("Message size: {}", cap);
    // let mut buf2 = BytesMut::with_capacity(cap as usize);
    let mut buf2 = [0; 31];
    let read = stream.read_exact(&mut buf2).await?;
    // let buf2 = Bytes::copy_from_slice(&buf2);
    // println!("Read {} bytes: {:?}", read, buf2);
    println!("A");
    let mut x = buf2.iter().copied();
    println!("B");
    let metadata =KafkaResponseS::<MetadataResponse>::deserialize(&mut x);
    println!("Metadata: {:#?}",metadata);

    Ok(())
}





pub trait KafkaRequest {
    fn serialize(&self,buf:&mut BytesMut);
}
impl KafkaRequest for str {
    fn serialize(&self, buf: &mut BytesMut) {
        buf.put_i16(self.len() as i16);
        buf.put_slice(&self.as_bytes());
    }
}
impl KafkaRequest for String {
    fn serialize(&self, buf: &mut BytesMut) {
        buf.put_i16(self.len() as i16);
        buf.put_slice(&self.as_bytes());
    }
}
impl KafkaRequest for i16 {
    fn serialize(&self, buf: &mut BytesMut) {
        buf.put_i16(self.clone());
    }
}
impl KafkaRequest for i32 {
    fn serialize(&self, buf: &mut BytesMut) {
        buf.put_i32(self.clone());
    }
}
impl<T> KafkaRequest for Vec<T> {
    fn serialize(&self, buf: &mut BytesMut) {
        buf.put_i32(self.len() as i32);
    }
}



#[derive(Debug,KafkaSerialize)]
pub struct HeaderRequest {
    pub api_key: i16,
    pub api_version: i16,
    pub correlation_id: i32,
    pub client_id: String,
}

impl HeaderRequest {
    fn new(
        api_key: i16,
        api_version: i16,
        correlation_id: i32,
        client_id: String,
    ) -> HeaderRequest {
        HeaderRequest {
            api_key: api_key,
            api_version: api_version,
            correlation_id: correlation_id,
            client_id: client_id,
        }
    }
}
impl HeaderRequest {
    fn encode(&self, buf: &mut BufMut) {
        buf.put_i16(self.api_key);
        buf.put_i16(self.api_version);
        buf.put_i32(self.correlation_id);
        buf.put_i16(self.client_id.len() as i16);
        buf.put_slice(&self.client_id.as_bytes());
    }
}
const API_KEY_METADATA: i16 = 3;
const API_VERSION: i16 = 0;

#[derive(Debug,KafkaSerialize)]
pub struct MetadataRequest {
    pub header: HeaderRequest,
    pub topics: Vec<String>,
}

impl MetadataRequest {
    pub fn new(correlation_id: i32, client_id: String, topics: Vec<String>) -> MetadataRequest {
        MetadataRequest {
            header: HeaderRequest::new(API_KEY_METADATA, API_VERSION, correlation_id, client_id),
            topics: topics,
        }
    }
}

impl MetadataRequest {
    fn encode(&self, buf: &mut BufMut) {
        self.header.encode(buf);
        let l = self.topics.len() as i32;
        buf.put_i32(l);
    }
}



pub trait KafkaResponse {
    fn deserialize<T>(buf: &mut T) -> Self where T:Iterator<Item=u8> ;
}
// impl<T> KafkaResponse<T> for i32 where T: IntoIterator<Item = u8> {
//     fn deserialize(buf:&T) {
//         buf.put_i32(self.clone());
//     }
// }
impl<R> KafkaResponse for Vec<R> where R:KafkaResponse+fmt::Debug {
    fn deserialize<T>(buf: &mut T) -> Vec<R> where T:Iterator<Item=u8> {
        println!("Deserialize start Vec");
        let len:[u8;4]  = [buf.next().unwrap(),buf.next().unwrap(),buf.next().unwrap(),buf.next().unwrap()];
        let cap = i32::from_be_bytes(len);
        println!("Vec size {}",cap);
        let mut ret = Vec::with_capacity(cap as usize);
        for i in 0..cap{
        println!("Start element {}",i);
        let element = KafkaResponse::deserialize(buf);
        println!("Element deserialized: {:?}",element);
        ret.push(element);
        } 
        println!("Deserialize end Vec");
        ret
    }
}
impl KafkaResponse for i32 {
    fn deserialize<T>(buf: &mut T) -> Self  where T: Iterator<Item = u8>{
        println!("Deserialize start i32");
        let data:[u8;4]  = [buf.next().unwrap(),buf.next().unwrap(),buf.next().unwrap(),buf.next().unwrap()];
        let x = i32::from_be_bytes(data);
        println!("{}",x);
        x

    }
}
impl KafkaResponse for i16 {
    fn deserialize<T>(buf: &mut T) -> Self where T: Iterator<Item = u8> {
        println!("Deserialize start i16");
        let data:[u8;2]  = [buf.next().unwrap(),buf.next().unwrap()];
        let x = i16::from_be_bytes(data);
        println!("{}",x);
        x
    }
}


impl KafkaResponse for String {
    fn deserialize<T>(buf: &mut T) -> Self where T: Iterator<Item = u8> {
        println!("Deserialize start String");
        let len:i16 = KafkaResponse::deserialize(buf);
        println!("String length {}",len);
        let data:Vec<u8>  = buf.take(len as usize).collect();
        println!("Deserialize end String");
        let x = String::from_utf8_lossy(&data).to_string();
        println!("{}",x);
        x
    }
}
#[derive(Debug,KafkaDeserialize)]
pub struct KafkaResponseS<S> where S:KafkaResponse{
    pub header: HeaderResponse,
    pub response: S
}

#[derive(Debug,KafkaDeserialize)]
pub struct HeaderResponse {
    pub correlation: i32,
}

#[derive(Debug,KafkaDeserialize)]
pub struct MetadataResponse {
    pub brokers: Vec<Broker>,
    pub topic: Vec<Topic>,
}
#[derive(Debug,KafkaDeserialize)]
pub struct Broker {
    pub node_id: i32,
    pub host: String,
    pub port: i32,
}
#[derive(Debug,KafkaDeserialize)]
pub struct Topic {
    pub error_code: i16,
    pub name: String,
    // pub partitions: Vec<Partition>,
}
#[derive(Debug,KafkaDeserialize)]
pub struct Partition {
    // pub error_code: i16,
    // pub partition_index: i32,
    // pub leader_id: i32,
    // pub replica_nodes: Vec<i32>,
    // pub isr_nodes: Vec<i32>,
}
