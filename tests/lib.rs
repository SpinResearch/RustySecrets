extern crate rusty_secrets;

use rusty_secrets::{recover_secret, generate_shares};

#[test]
fn test_reasonable_splits() {
    let max_shares = 50;
    let secret = String::from("I grew up with the understanding that the world I lived in was one where people enjoyed a sort of freedom to communicate with each other in privacy, without it being monitored, without it being measured or analyzed or sort of judged by these shadowy figures or systems, any time they mention anything that travels across public lines.").into_bytes();
    for k in 2..max_shares {
        for n in k..max_shares{
            let shares = generate_shares(k,n, &secret).unwrap();
            println!("Testing {} out-of- {}", k, n);
            assert_eq!(secret, recover_secret(shares).unwrap());
        }
    }
}
