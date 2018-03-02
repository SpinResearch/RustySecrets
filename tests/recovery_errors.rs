extern crate rusty_secrets;

use rusty_secrets::sss::recover_secret;

#[test]
#[should_panic(expected = "EmptyShares")]
fn test_recover_no_shares() {
    for signed in &[true, false] {
        let shares = vec![];
        recover_secret(&shares, *signed).unwrap();
    }
}

#[test]
#[should_panic(expected = "ShareParsingError")]
fn test_recover_2_parts_share() {
    let share1 = "2-1-CgmKQZHMO+5n5pU".to_string();
    let share2 = "2-2".to_string();

    let shares = vec![share1, share2];

    recover_secret(&shares, false).unwrap();
}

#[test]
#[should_panic(expected = "IntegerParsingError")]
fn test_recover_incorrect_share_num() {
    let share1 = "2-1-CgnlCxRNtnkzENE".to_string();
    let share2 = "2-DEFINITLY_NAN-CgkAnUgP3lfwjyM".to_string();

    let shares = vec![share1, share2];

    recover_secret(&shares, false).unwrap();
}

#[test]
#[should_panic(expected = "ShareParsingError")]
fn test_recover_0_share_num() {
    let share1 = "2-0-1YAYwmOHqZ69jA".to_string();
    let share2 = "2-1-YJZQDGm22Y77Gw".to_string();

    let shares = vec![share1, share2];

    recover_secret(&shares, false).unwrap();
}

#[test]
#[should_panic(expected = "ShareParsingError")]
fn test_recover_invalid_b64() {
    let share1 = "2-1-CgnlCxRNtnkzENE".to_string();
    let share2 = "2-2-YJZQDG((((m22Y)))77Gw".to_string();

    let shares = vec![share1, share2];

    recover_secret(&shares, false).unwrap();
}

#[test]
#[should_panic(expected = "DuplicateShareId")]
fn test_recover_duplicate_shares_number() {
    let share1 = "2-1-CgnlCxRNtnkzENE".to_string();
    let share2 = "2-1-CgkAnUgP3lfwjyM".to_string();

    let shares = vec![share1, share2];

    recover_secret(&shares, false).unwrap();
}

#[test]
#[should_panic(expected = "DuplicateShareData")]
fn test_recover_duplicate_shares_data() {
    let share1 = "2-1-CgnlCxRNtnkzENE".to_string();
    let share2 = "2-2-CgnlCxRNtnkzENE".to_string();

    let shares = vec![share1, share2];

    recover_secret(&shares, false).unwrap();
}

#[test]
#[should_panic(expected = "MissingShares")]
fn test_recover_too_few_shares() {
    let share1 = "3-1-ChbcCdSZOaMn6DM1jFca2P6/0WRlP7AK".to_string();
    let share2 = "3-2-ChbG46L1zRszs0PPn63XnnupmZTcgYJ3".to_string();

    let shares = vec![share1, share2];

    recover_secret(&shares, false).unwrap();
}

// See https://github.com/SpinResearch/RustySecrets/issues/43
#[test]
fn test_recover_too_few_shares_bug() {
    let original = b"Test for issue #43\n".to_vec();
    let share1 = "2-1-ChOCUGsi+CoZEqMpQyk/AAI2vigg".to_string();
    let share2 = "2-4-ChMrsRMxZ0uq7xZ0swZA7Kh3Jl+i".to_string();

    let shares = vec![share1, share2];

    match recover_secret(&shares, false) {
        Err(_) => assert!(true),
        Ok(recovered) => assert_ne!(original, recovered),
    }
}
