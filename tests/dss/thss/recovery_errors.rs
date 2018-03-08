use rusty_secrets::dss::thss::{recover_secret, Share};

#[test]
#[should_panic(expected = "EmptyShares")]
fn test_recover_no_shares() {
    let shares = vec![];
    recover_secret(&shares).unwrap();
}

#[test]
#[should_panic(expected = "ShareParsingErrorEmptyShare")]
fn test_recover_2_parts_share() {
    let share1 = Share {
        id: 1,
        threshold: 2,
        shares_count: 2,
        data: "CgmKQZHMO+5n5pU".to_string().into_bytes(),
        metadata: None,
    };
    let share2 = Share {
        id: 2,
        threshold: 2,
        shares_count: 2,
        data: "".to_string().into_bytes(),
        metadata: None,
    };

    let shares = vec![share1, share2];

    recover_secret(&shares).unwrap();
}

#[test]
#[should_panic(expected = "ShareParsingInvalidShareId")]
fn test_recover_0_share_num() {
    let share1 = Share {
        id: 0,
        threshold: 2,
        shares_count: 2,
        data: "1YAYwmOHqZ69jA".to_string().into_bytes(),
        metadata: None,
    };
    let share2 = Share {
        id: 1,
        threshold: 2,
        shares_count: 2,
        data: "YJZQDGm22Y77Gw".to_string().into_bytes(),
        metadata: None,
    };

    let shares = vec![share1, share2];

    recover_secret(&shares).unwrap();
}

// ---
// TODO: will be implemented when serialization is done for thss shares
// ---
// #[test]
// #[should_panic(expected = "ShareParsingError")]
// fn test_recover_invalid_b64() {
//     let share1 = Share {
//         id: 1,
//         threshold: 2,
//         shares_count: 2,
//         data: "1YAYwmOHqZ69jA".to_string().into_bytes(),
//         metadata: None
//     };
//     let share2 = Share {
//         id: 2,
//         threshold: 2,
//         shares_count: 2,
//         data: "YJZQDG((((m22Y)))77Gw".to_string().into_bytes(),
//         metadata: None
//     };
//
//     let shares = vec![share1, share2];
//
//     recover_secret(&shares).unwrap();
// }

#[test]
#[should_panic(expected = "DuplicateShareId")]
fn test_recover_duplicate_shares_number() {
    let share1 = Share {
        id: 1,
        threshold: 2,
        shares_count: 2,
        data: "1YAYwmOHqZ69jA".to_string().into_bytes(),
        metadata: None,
    };
    let share2 = Share {
        id: 1,
        threshold: 2,
        shares_count: 2,
        data: "YJZQDGm22Y77Gw".to_string().into_bytes(),
        metadata: None,
    };

    let shares = vec![share1, share2];

    recover_secret(&shares).unwrap();
}

#[test]
#[should_panic(expected = "MissingShares")]
fn test_recover_too_few_shares() {
    let share1 = Share {
        id: 1,
        threshold: 3,
        shares_count: 3,
        data: "1YAYwmOHqZ69jA".to_string().into_bytes(),
        metadata: None,
    };
    let share2 = Share {
        id: 2,
        threshold: 3,
        shares_count: 3,
        data: "YJZQDGm22Y77Gw".to_string().into_bytes(),
        metadata: None,
    };

    let shares = vec![share1, share2];

    recover_secret(&shares).unwrap();
}
