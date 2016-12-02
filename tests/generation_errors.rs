extern crate rusty_secrets;

use rusty_secrets::generate_shares;

#[test]
#[should_panic(expected = "Threshold K can not be larger than N")]
fn test_generate_invalid_k() {
    let share1 = "2-1-1YAYwmOHqZ69jA".to_string().into_bytes();

    generate_shares(10, 5, share1.as_slice(), true).unwrap();
}
