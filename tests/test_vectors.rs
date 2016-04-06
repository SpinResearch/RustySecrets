extern crate rusty_secrets;

use rusty_secrets::{recover_secret};

#[test]
fn test_recover_sellibitze() {
    let share1 = "2-1-1YAYwmOHqZ69jA".to_string();
    let share2 = "2-4-F7rAjX3UOa53KA".to_string();

    let shares = vec![share1, share2];

    let mut secret = "My secret".to_string().into_bytes();
    secret.push(10);
    assert_eq!(recover_secret(shares).unwrap(), secret);
}

// Generated with code on master branch on the 6th of April.
#[test]
fn test_recover_es_test_vectors() {
    let share1 = "5-1-DbuicpLQiCf7bVWiAz8eCpQGpdZmYQ7z2j2+g351tWFLOQPTZkXY8BYfwGGGjkOoz1g9x0ScmLFcWk+2tign".to_string();
    let share2 = "5-2-nShdfkY5+SlfybMyqjHXCZ01bq5N/0Lkf0nQZw5x3bnHIEVfa0RA4YcJ4SjG/UdpgO/gOcyLRkSp2Dwf8bvw".to_string();
    let share3 = "5-3-qEhJ3IVEdbDkiRoy+jOJ/KuGE9jWyGeOYEcDwPfEV8E9rfD1Bc17BQAbJ51Xd8oexS2M1qMvNgJHZUQZbUgQ".to_string();
    let share4 = "5-6-yyVPUeaYPPiWK0wIV5OQ/t61V0lSEO+7X++EWeHRlIq3sRBNwUpKNfx/C+Vc9xTzUftrqBKvkWDZQal7nyi2".to_string();
    let share5 = "5-7-i8iL6bVf272B3qIjp0QqSny6AIm+DkP7oQjkVVLvx9EMhlvd4HJOxPpmtNF/RjA/zz21d7DY/B//saOPpBQa".to_string();

    let shares = vec![share1, share2, share3, share4, share5];

    let secret = "The immoral cannot be made moral through the use of secret law.".to_string().into_bytes();
    assert_eq!(recover_secret(shares).unwrap(), secret);
}

#[test]
fn test_recover_sellibitze_more_than_threshold_shars() {
    let share1 = "2-1-1YAYwmOHqZ69jA".to_string();
    let share2 = "2-4-F7rAjX3UOa53KA".to_string();
    let share3 = "2-2-YJZQDGm22Y77Gw".to_string();
    let share4 = "2-5-j0P4PHsw4lW+rg".to_string();

    let shares = vec![share1, share2, share3, share4];

    let mut secret = "My secret".to_string().into_bytes();
    secret.push(10);
    assert_eq!(recover_secret(shares).unwrap(), secret);
}
