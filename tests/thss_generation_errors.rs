extern crate rusty_secrets;

use rusty_secrets::dss::thss;

#[test]
#[should_panic(expected = "ThresholdTooBig")]
fn test_generate_invalid_k() {
    let secret = "These programs were never about terrorism: they’re about economic spying, \
                  social control, and diplomatic manipulation. They’re about power.";

    thss::split_secret(10, 7, secret, &None).unwrap();
}
