#![allow(dead_code)]

pub mod cluster;
pub mod test_topic;

pub const KAFKA_TEST_BROKER_ADDR_1_HOST: &str = "localhost";
pub const KAFKA_TEST_BROKER_ADDR_1_PORT: u16 = 19092;
pub const KAFKA_TEST_BROKER_1_RACK: Option<&str> = Some("RACK_A");

pub const KAFKA_TEST_BROKER_ADDR_2_HOST: &str = "localhost";
pub const KAFKA_TEST_BROKER_ADDR_2_PORT: u16 = 29092;
pub const KAFKA_TEST_BROKER_2_RACK: Option<&str> = Some("RACK_B");

pub const KAFKA_TEST_BROKER_ADDR_3_HOST: &str = "localhost";
pub const KAFKA_TEST_BROKER_ADDR_3_PORT: u16 = 39092;
pub const KAFKA_TEST_BROKER_3_RACK: Option<&str> = None;
