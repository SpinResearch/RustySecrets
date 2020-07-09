#![cfg(test)]
#![feature(test)]
#![cfg(feature = "dss")]

extern crate rusty_secrets;
extern crate test;

mod shared;

mod ss1 {
    use super::shared;

    use rusty_secrets::dss::ss1;
    use test::{black_box, Bencher};

    macro_rules! bench_generate {
        ($name:ident, $k:expr, $n:expr, $secret:ident) => {
            #[bench]
            fn $name(b: &mut Bencher) {
                let secret = shared::$secret();

                b.iter(move || {
                    let shares = ss1::split_secret(
                        $k,
                        $n,
                        &secret,
                        ss1::Reproducibility::reproducible(),
                        &None,
                    )
                    .unwrap();
                    black_box(shares);
                });
            }
        };
    }

    macro_rules! bench_recover {
        ($name:ident, $k:expr, $n:expr, $secret:ident) => {
            #[bench]
            fn $name(b: &mut Bencher) {
                let secret = shared::$secret();
                let all_shares =
                    ss1::split_secret($k, $n, &secret, ss1::Reproducibility::reproducible(), &None)
                        .unwrap();
                let shares = &all_shares.into_iter().take($k).collect::<Vec<_>>().clone();

                b.iter(|| {
                    let result = ss1::recover_secret(&shares.to_vec()).unwrap();
                    black_box(result);
                });
            }
        };
    }

    bench_generate!(generate_1kb_3_5, 3, 5, secret_1kb);
    bench_recover!(recover_1kb_3_5, 3, 5, secret_1kb);

    bench_generate!(generate_1kb_10_25, 10, 25, secret_1kb);
    bench_recover!(recover_1kb_10_25, 10, 25, secret_1kb);
}
