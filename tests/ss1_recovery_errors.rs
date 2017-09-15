extern crate rusty_secrets;
use rusty_secrets::dss::ss1::{recover_secret, split_secret, Share};

const TEST_THRESHOLD: u8 = 2;
const TEST_SHARES_COUNT: u8 = 2;
const TEST_SECRET: &[u8] = b"These programs were never about terrorism: they're about economic spying, \
                             social control, and diplomatic manipulation. They're about power.";

fn get_test_hash() -> Vec<u8> {
    let good_shares = split_secret( TEST_THRESHOLD,
                                    TEST_SHARES_COUNT,
                                    TEST_SECRET,
                                    &None ).unwrap();

    good_shares[0].hash.clone()
}

 #[test]
 #[should_panic(expected = "EmptyShares")]
 fn test_recover_no_shares() {
     let shares = vec![];
     recover_secret(&shares).unwrap();
 }

#[test]
#[should_panic(expected = "ShareParsingErrorEmptyShare")]
fn test_recover_2_parts_share() {
    let hash = get_test_hash();

    let share1 = Share {
        id: 1,
        threshold: TEST_THRESHOLD,
        total_shares_count: TEST_SHARES_COUNT,
        data: "CgmKQZHMO+5n5pU".to_string().into_bytes(),
        hash: hash.clone(),
        metadata: None
    };
    let share2 = Share {
        id: 2,
        threshold: TEST_THRESHOLD,
        total_shares_count: TEST_SHARES_COUNT,
        data: "".to_string().into_bytes(),
        hash: hash.clone(),
        metadata: None
    };

    let shares = vec![share1, share2];

    recover_secret(&shares).unwrap();
}

#[test]
#[should_panic(expected = "ShareParsingInvalidShareId")]
fn test_recover_0_share_num() {
    let hash = get_test_hash();

    let share1 = Share {
        id: 0,
        threshold: TEST_THRESHOLD,
        total_shares_count: TEST_SHARES_COUNT,
        data: "1YAYwmOHqZ69jA".to_string().into_bytes(),
        hash: hash.clone(),
        metadata: None
    };
    let share2 = Share {
        id: 1,
        threshold: TEST_THRESHOLD,
        total_shares_count: TEST_SHARES_COUNT,
        data: "YJZQDGm22Y77Gw".to_string().into_bytes(),
        hash: hash.clone(),
        metadata: None
    };

    let shares = vec![share1, share2];

    recover_secret(&shares).unwrap();
}

// // ---
// // TODO: will be implemented when serialization is done for ss1 shares
// // ---
// // #[test]
// // #[should_panic(expected = "ShareParsingError")]
// // fn test_recover_invalid_b64() {
// //     let share1 = Share {
// //         id: 1,
// //         threshold: 2,
// //         total_shares_count: 2,
// //         data: "1YAYwmOHqZ69jA".to_string().into_bytes(),
// //         metadata: None
// //     };
// //     let share2 = Share {
// //         id: 2,
// //         threshold: 2,
// //         total_shares_count: 2,
// //         data: "YJZQDG((((m22Y)))77Gw".to_string().into_bytes(),
// //         metadata: None
// //     };
// //
// //     let shares = vec![share1, share2];
// //
// //     recover_secret(&shares).unwrap();
// // }
//
// #[test]
// #[should_panic(expected = "DuplicateShareId")]
// fn test_recover_duplicate_shares_number() {
//     let share1 = Share {
//         id: 1,
//         threshold: 2,
//         total_shares_count: 2,
//         data: "1YAYwmOHqZ69jA".to_string().into_bytes(),
//         metadata: None
//     };
//     let share2 = Share {
//         id: 1,
//         threshold: 2,
//         total_shares_count: 2,
//         data: "YJZQDGm22Y77Gw".to_string().into_bytes(),
//         metadata: None
//     };
//
//     let shares = vec![share1, share2];
//
//     recover_secret(&shares).unwrap();
// }
//
// #[test]
// #[should_panic(expected = "DuplicateShareData")]
// fn test_recover_duplicate_shares_data() {
//     let share1 = Share {
//         id: 1,
//         threshold: 2,
//         total_shares_count: 2,
//         data: "1YAYwmOHqZ69jA".to_string().into_bytes(),
//         metadata: None
//     };
//     let share2 = Share {
//         id: 2,
//         threshold: 2,
//         total_shares_count: 2,
//         data: "1YAYwmOHqZ69jA".to_string().into_bytes(),
//         metadata: None
//     };
//
//     let shares = vec![share1, share2];
//
//     recover_secret(&shares).unwrap();
// }
//
// #[test]
// #[should_panic(expected = "MissingShares")]
// fn test_recover_too_few_shares() {
//     let share1 = Share {
//         id: 1,
//         threshold: 3,
//         total_shares_count: 3,
//         data: "1YAYwmOHqZ69jA".to_string().into_bytes(),
//         metadata: None
//     };
//     let share2 = Share {
//         id: 2,
//         threshold: 3,
//         total_shares_count: 3,
//         data: "YJZQDGm22Y77Gw".to_string().into_bytes(),
//         metadata: None
//     };
//
//     let shares = vec![share1, share2];
//
//     recover_secret(&shares).unwrap();
// }
