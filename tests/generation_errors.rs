extern crate rusty_secrets;

use rusty_secrets::custom_error::{pie2io};
use rusty_secrets::generate_shares;
use std::error::Error;

#[test]
#[should_panic(expected = "Threshold K can not be larger than N")]
fn test_generate_invalid_k() {
    let share1 = "2-1-1YAYwmOHqZ69jA".to_string().into_bytes();

    generate_shares(10, 5, &share1).unwrap();
}

#[test]
fn test_parse_errors() {
    let nan = "2a".to_string();
    match nan.parse::<u8>().map_err(pie2io) {
        Ok(_) => assert!(false),
        Err(x) => assert_eq!("Integer parsing error", x.description()),
    }
}
