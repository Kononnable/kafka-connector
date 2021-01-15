use std::fs::create_dir;

use parser::parser::parse_api_call;


const REQ: &str = r#"Fetch Response (Version: 3) => throttle_time_ms [responses] 
throttle_time_ms => INT32
responses => topic [partition_responses] 
  topic => STRING
  partition_responses => partition error_code high_watermark record_set 
    partition => INT32
    error_code => INT16
    high_watermark => INT64
    record_set => RECORDS
"#;

fn main() {
   let result =  parse_api_call(REQ).unwrap();
   println!("{:#?}",result);
}
