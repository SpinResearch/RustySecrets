extern crate protobuf;
extern crate rustc_serialize as serialize;

use protobuf::Message;
use share_data;
use sss::recover_secret;
use serialize::base64::{self, FromBase64, ToBase64};

pub fn wrap_from_sellibitze(share: &str) -> String {
    let parts: Vec<_> = share.trim().split('-').collect();
    let share_data = parts[2].from_base64().unwrap();

    let config = base64::Config {
        pad: false,
        ..base64::STANDARD
    };

    let mut share_protobuf = share_data::ShareData::new();
    share_protobuf.set_shamir_data(share_data);

    let b64_share = share_protobuf.write_to_bytes().unwrap().to_base64(config);

    format!("{}-{}-{}", parts[0], parts[1], b64_share)
}

#[test]
fn test_recover_sellibitze() {
    let share1 = "2-1-1YAYwmOHqZ69jA";
    let share2 = "2-4-F7rAjX3UOa53KA";

    let shares = vec![share1, share2]
        .iter()
        .map(|x| wrap_from_sellibitze(x))
        .collect::<Vec<_>>();

    let mut secret = "My secret".to_string().into_bytes();
    secret.push(10);
    assert_eq!(recover_secret(shares, false).unwrap(), secret);
}

// Generated with code on master branch on the 6th of April.
#[test]
fn test_recover_es_test_vectors() {
    let share1 = "5-1-DbuicpLQiCf7bVWiAz8eCpQGpdZmYQ7z2j2+g351tWFLOQPTZkXY8BYfwGGGjkOoz1g9x0ScmLFcWk+2tign"
        .to_string();
    let share2 = "5-2-nShdfkY5+SlfybMyqjHXCZ01bq5N/0Lkf0nQZw5x3bnHIEVfa0RA4YcJ4SjG/UdpgO/gOcyLRkSp2Dwf8bvw"
        .to_string();
    let share3 = "5-3-qEhJ3IVEdbDkiRoy+jOJ/KuGE9jWyGeOYEcDwPfEV8E9rfD1Bc17BQAbJ51Xd8oexS2M1qMvNgJHZUQZbUgQ"
        .to_string();
    let share4 = "5-6-yyVPUeaYPPiWK0wIV5OQ/t61V0lSEO+7X++EWeHRlIq3sRBNwUpKNfx/C+Vc9xTzUftrqBKvkWDZQal7nyi2"
        .to_string();
    let share5 = "5-7-i8iL6bVf272B3qIjp0QqSny6AIm+DkP7oQjkVVLvx9EMhlvd4HJOxPpmtNF/RjA/zz21d7DY/B//saOPpBQa"
        .to_string();

    let shares = vec![share1, share2, share3, share4, share5]
        .iter()
        .map(|x| wrap_from_sellibitze(x))
        .collect::<Vec<_>>();

    let secret = "The immoral cannot be made moral through the use of secret law."
        .to_string()
        .into_bytes();
    assert_eq!(recover_secret(shares, false).unwrap(), secret);
}

#[test]
fn test_recover_sellibitze_more_than_threshold_shars() {
    let share1 = "2-1-1YAYwmOHqZ69jA".to_string();
    let share2 = "2-4-F7rAjX3UOa53KA".to_string();
    let share3 = "2-2-YJZQDGm22Y77Gw".to_string();
    let share4 = "2-5-j0P4PHsw4lW+rg".to_string();

    let shares = vec![share1, share2, share3, share4]
        .iter()
        .map(|x| wrap_from_sellibitze(x))
        .collect::<Vec<_>>();

    let mut secret = "My secret".to_string().into_bytes();
    secret.push(10);
    assert_eq!(recover_secret(shares, false).unwrap(), secret);
}
