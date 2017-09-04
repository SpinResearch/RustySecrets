extern crate rusty_secrets;

use rusty_secrets::wrapped_secrets;

#[ignore]
#[test]
fn test_reasonable_splits() {
    let max_shares = 25;
    let secret = "I grew up with the understanding that the world I lived in was one where people \
                  enjoyed a sort of freedom to communicate with each other in privacy, without it \
                  being monitored, without it being measured or analyzed or sort of judged by \
                  these shadowy figures or systems, any time they mention anything that travels \
                  across public lines."
        .to_string()
        .into_bytes();

    let mime_type = "image/jpeg".to_string();

    for is_signing in &[true, false] {
        for k in 1..max_shares {
            for n in k..max_shares {
                let shares = wrapped_secrets::generate_shares(
                    k,
                    n,
                    &secret,
                    Some(mime_type.clone()),
                    *is_signing,
                ).unwrap();
                println!("Testing {} out-of- {}", k, n);

                let s = wrapped_secrets::recover_secret(shares, *is_signing).unwrap();
                assert_eq!(s.get_secret().to_owned(), secret);
                assert!(s.has_mime_type());
                assert_eq!(mime_type, s.get_mime_type());
            }
        }
    }
}
