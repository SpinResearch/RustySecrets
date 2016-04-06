extern crate rusty_secrets;

use rusty_secrets::{recover_secret};

#[test]
#[should_panic(expected = "Not enough shares provided!")]
fn test_recover_sellibitze_missing_share() {
    let share1 = "2-1-1YAYwmOHqZ69jA".to_string();
    let shares = vec![share1];

    recover_secret(shares).unwrap();
}

#[test]
#[should_panic(expected = "Not enough shares provided!")]
fn test_recover_sellibitze_no_shares() {
    let shares = vec![];
    recover_secret(shares).unwrap();
}

#[test]
#[should_panic(expected = "Share parse error: Expected 3 parts separated by a minus sign")]
fn test_recover_2_parts_share() {
    let share1 = "2-1-1YAYwmOHqZ69jA".to_string();
    let share2 = "2-F7rAjX3UOa53KA".to_string();

    let shares = vec![share1, share2];

    recover_secret(shares).unwrap();
}

#[test]
#[should_panic(expected = "Integer parsing error")]
fn test_recover_incorrect_share_num() {
    let share1 = "2-1-1YAYwmOHqZ69jA".to_string();
    let share2 = "2-DEFINITLY_NAN-YJZQDGm22Y77Gw".to_string();

    let shares = vec![share1, share2];

    recover_secret(shares).unwrap();
}

#[test]
#[should_panic(expected = "Share parse error: Illegal K,N parameters")]
fn test_recover_0_share_num() {
    let share1 = "2-0-1YAYwmOHqZ69jA".to_string();
    let share2 = "2-1-YJZQDGm22Y77Gw".to_string();

    let shares = vec![share1, share2];

    recover_secret(shares).unwrap();
}

#[test]
#[should_panic(expected = "Share parse error: Base64 decoding of data block failed")]
fn test_recover_invalid_b64() {
    let share1 = "2-5-j0P4PHsw4lW+rg".to_string();
    let share2 = "2-1-YJZQDG((((m22Y)))77Gw".to_string();

    let shares = vec![share1, share2];

    recover_secret(shares).unwrap();
}

#[test]
#[should_panic(expected = "Incompatible shares")]
fn test_recover_invalid_b64_size() {
    let share1 = "2-5-j0P4PHsw4lW+rg".to_string();
    let share2 = "2-1-YJZQDGm22Y77GwZ69jA".to_string();

    let shares = vec![share1, share2];

    recover_secret(shares).unwrap();
}

#[test]
#[should_panic(expected = "Duplicate Share Number")]
fn test_recover_duplicate_shares_number() {
    let share1 = "2-1-1YAYwmOHqZ69jA".to_string();
    let share2 = "2-1-j0P4PHsw4lW+rg".to_string();

    let shares = vec![share1, share2];

    recover_secret(shares).unwrap();
}

#[test]
#[should_panic(expected = "Duplicate Share Data")]
fn test_recover_duplicate_shares_data() {
    let share1 = "2-2-YJZQDGm22Y77Gw".to_string();
    let share2 = "2-3-YJZQDGm22Y77Gw".to_string();

    let shares = vec![share1, share2];

    recover_secret(shares).unwrap();
}

#[test]
#[should_panic(expected = "Not enough shares provided!")]
fn test_recover_too_few_shares() {
    let share1 = "5-1-DbuicpLQiCf7bVWiAz8eCpQGpdZmYQ7z2j2+g351tWFLOQPTZkXY8BYfwGGGjkOoz1g9x0ScmLFcWk+2tign".to_string();
    let share2 = "5-2-nShdfkY5+SlfybMyqjHXCZ01bq5N/0Lkf0nQZw5x3bnHIEVfa0RA4YcJ4SjG/UdpgO/gOcyLRkSp2Dwf8bvw".to_string();

    let shares = vec![share1, share2];

    recover_secret(shares).unwrap();
}
