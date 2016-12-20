extern crate rusty_secrets;

use rusty_secrets::{generate_shares, recover_secret};

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
    for is_signing in vec![true, false] {
        for k in 1..max_shares {
            for n in k..max_shares {
                let shares = generate_shares(k, n, &secret, is_signing).unwrap();
                println!("Testing {} out-of- {}", k, n);
                assert_eq!(secret, recover_secret(shares, is_signing).unwrap());
            }
        }
    }
}
