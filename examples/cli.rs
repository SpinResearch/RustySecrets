#[macro_use]
extern crate clap;
extern crate rusty_secrets;

use rusty_secrets::{generate_shares, recover_secret};

fn main() {
    let matches = clap_app!(sss =>
        (about: "generate/recover \"Shamir's Secret Sharing\" shares")
        (@subcommand generate =>
            (about: "Generate a new shared secret set")
            (@arg k: -k +required +takes_value "the number of shares needed to recover")
            (@arg n: -n +required +takes_value "the total shares to generate")
            (@arg secret: +required "the secret to be split"))
        (@subcommand recover =>
            (about: "Recover a secret from k/n shares")
            (@arg shares: ... +required "the shares to be used for recovery"))
        ).get_matches();

    match matches.subcommand_name() {
        Some("generate") => {
            let matches = matches.subcommand_matches("generate").unwrap();
            let k = value_t_or_exit!(matches, "k", u8);
            let n = value_t_or_exit!(matches, "n", u8);
            let secret = value_t_or_exit!(matches, "secret", String);
            match generate_shares(k, n, &secret.into_bytes()) {
                Ok(shares) => {
                    for share in shares {
                        println!("{}", share);
                    }
                }
                Err(_) => {
                    println!("failed to generate shares");
                }
            }
        }
        Some("recover") => {
            let matches = matches.subcommand_matches("recover").unwrap();
            let shares = values_t_or_exit!(matches, "shares", String);
            match recover_secret(shares) {
                Ok(secret) => {
                    match String::from_utf8(secret.clone()) {
                        Ok(s) => println!("{}", s),
                        Err(_) => println!("{:?}", secret)
                    }
                }
                Err(_) => {
                    println!("failed to recover secret");
                }
            }
        }
        _ => {
            println!("Please specify a subcommand.");
        }
    }
}
