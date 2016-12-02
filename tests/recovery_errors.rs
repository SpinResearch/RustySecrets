extern crate rusty_secrets;

use rusty_secrets::recover_secret;

#[test]
#[should_panic(expected = "Not enough shares provided!")]
fn test_recover_sellibitze_no_shares() {
    let shares = vec![];
    recover_secret(shares, false).unwrap();
}

#[test]
#[should_panic(expected = "Share parse error: Expected 3 parts separated by a minus sign")]
fn test_recover_2_parts_share() {
    let share1 = "2-1-CgmKQZHMO+5n5pU".to_string();
    let share2 = "2-2".to_string();

    let shares = vec![share1, share2];

    recover_secret(shares, false).unwrap();
}

#[test]
#[should_panic(expected = "Integer parsing error")]
fn test_recover_incorrect_share_num() {
    let share1 = "2-1-CgnlCxRNtnkzENE".to_string();
    let share2 = "2-DEFINITLY_NAN-CgkAnUgP3lfwjyM".to_string();

    let shares = vec![share1, share2];

    recover_secret(shares, false).unwrap();
}

#[test]
#[should_panic(expected = "Share parse error: Illegal K,N parameters")]
fn test_recover_0_share_num() {
    let share1 = "2-0-1YAYwmOHqZ69jA".to_string();
    let share2 = "2-1-YJZQDGm22Y77Gw".to_string();

    let shares = vec![share1, share2];

    recover_secret(shares, false).unwrap();
}

#[test]
#[should_panic(expected = "Share parse error: Base64 decoding of data block failed")]
fn test_recover_invalid_b64() {
    let share1 = "2-1-CgnlCxRNtnkzENE".to_string();
    let share2 = "2-1-YJZQDG((((m22Y)))77Gw".to_string();

    let shares = vec![share1, share2];

    recover_secret(shares, false).unwrap();
}

#[test]
#[should_panic(expected = "Duplicate Share Number")]
fn test_recover_duplicate_shares_number() {
    let share1 = "2-1-CgnlCxRNtnkzENE".to_string();
    let share2 = "2-1-CgkAnUgP3lfwjyM".to_string();

    let shares = vec![share1, share2];

    recover_secret(shares, false).unwrap();
}

#[test]
#[should_panic(expected = "Duplicate Share Data")]
fn test_recover_duplicate_shares_data() {
    let share1 = "2-1-CgnlCxRNtnkzENE".to_string();
    let share2 = "2-2-CgnlCxRNtnkzENE".to_string();

    let shares = vec![share1, share2];

    recover_secret(shares, false).unwrap();
}

#[test]
#[should_panic(expected = "Not enough shares provided!")]
fn test_recover_too_few_shares() {
    let share1 = "3-1-ChbcCdSZOaMn6DM1jFca2P6/0WRlP7AK".to_string();
    let share2 = "3-2-ChbG46L1zRszs0PPn63XnnupmZTcgYJ3".to_string();

    let shares = vec![share1, share2];

    recover_secret(shares, false).unwrap();
}
