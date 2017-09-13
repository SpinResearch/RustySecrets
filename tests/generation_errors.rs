extern crate rusty_secrets;

use rusty_secrets::sss;

#[test]
#[should_panic(expected = "ThresholdTooBig")]
fn test_generate_invalid_k() {
    let share1 = "2-1-1YAYwmOHqZ69jA".to_string().into_bytes();

    sss::generate_shares(10, 5, share1.as_slice(), true).unwrap();
}
