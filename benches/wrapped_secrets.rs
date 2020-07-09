#![cfg(test)]
#![feature(test)]

extern crate rusty_secrets;
extern crate test;

mod shared;

mod wrapped_secrets {
    use super::shared;

    use rusty_secrets::wrapped_secrets;
    use test::{black_box, Bencher};

    macro_rules! bench_generate {
        ($name:ident, $k:expr, $n:expr, $secret:ident, $signed:expr) => {
            #[bench]
            fn $name(b: &mut Bencher) {
                let secret = shared::$secret();

                b.iter(move || {
                    let shares =
                        wrapped_secrets::split_secret($k, $n, secret, None, $signed).unwrap();
                    black_box(shares);
                });
            }
        };
    }

    macro_rules! bench_recover {
        ($name:ident, $k:expr, $n:expr, $secret:ident, $signed:expr) => {
            #[bench]
            fn $name(b: &mut Bencher) {
                let secret = shared::$secret();
                let all_shares =
                    wrapped_secrets::split_secret($k, $n, &secret, None, $signed).unwrap();
                let shares = all_shares.into_iter().take($k).collect::<Vec<_>>();

                b.iter(|| {
                    let result = wrapped_secrets::recover_secret(&shares, $signed).unwrap();
                    black_box(result);
                });
            }
        };
    }

    bench_generate!(generate_1kb_3_5, 3, 5, secret_1kb, false);
    bench_recover!(recover_1kb_3_5, 3, 5, secret_1kb, false);

    bench_generate!(generate_1kb_3_5_signed, 3, 5, secret_1kb, true);
    bench_recover!(recover_1kb_3_5_signed, 3, 5, secret_1kb, true);

    bench_generate!(generate_1kb_10_25, 10, 25, secret_1kb, false);
    bench_recover!(recover_1kb_10_25, 10, 25, secret_1kb, false);

    bench_generate!(generate_1kb_10_25_signed, 10, 25, secret_1kb, true);
    bench_recover!(recover_1kb_10_25_signed, 10, 25, secret_1kb, true);
}
