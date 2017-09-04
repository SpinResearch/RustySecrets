
#![cfg(test)]
#![feature(test)]

extern crate test;
extern crate rusty_secrets;

mod shared;

mod sss {

    use test::{Bencher, black_box};
    use rusty_secrets::sss::{recover_secret, generate_shares};
    use shared;

    macro_rules! bench_generate {
        ($name:ident, $k:expr, $n:expr, $secret:ident, $signed:expr) => (
            #[bench]
            fn $name(b: &mut Bencher) {
                let secret = shared::$secret();

                b.iter(move || {
                    let shares = generate_shares($k, $n, secret, $signed).unwrap();
                    black_box(shares);
                });
            }
        )
    }

    macro_rules! bench_recover {
        ($name:ident, $k:expr, $n:expr, $secret:ident, $signed:expr) => (
            #[bench]
            fn $name(b: &mut Bencher) {
                let secret = shared::$secret();
                let all_shares = generate_shares($k, $n, &secret, $signed).unwrap();
                let shares = &all_shares.into_iter().take($k).collect::<Vec<_>>().clone();

                b.iter(|| {
                    let result = recover_secret(shares.to_vec(), $signed).unwrap();
                    black_box(result);
                });
            }
        )
    }

    bench_generate!(generate_1kb_3_5, 3, 5, secret_1kb, false);
    bench_recover!(recover_1kb_3_5, 3, 5, secret_1kb, false);

    bench_generate!(generate_1kb_3_5_signed, 3, 5, secret_1kb, true);
    bench_recover!(recover_1kb_3_5_signed, 3, 5, secret_1kb, true);

    bench_generate!(generate_1kb_10_25, 10, 25, secret_1kb, false);
    bench_recover!(recover_1kb_10_25, 10, 25, secret_1kb, false);

    bench_generate!(generate_1kb_10_25_signed, 10, 25, secret_1kb, true);
    bench_recover!(recover_1kb_10_25_signed, 10, 25, secret_1kb, true);

    // bench_generate!(generate_1mb_3_5_unsigned, 3, 5, secret_1mb, false);
    // bench_recover!(recover_1mb_3_5_unsigned, 3, 5, secret_1mb, false);

}
