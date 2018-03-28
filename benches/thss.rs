#![cfg(test)]
#![feature(test)]
#![cfg(feature = "dss")]

extern crate rusty_secrets;
extern crate test;

mod shared;

mod thss {

    use rusty_secrets::dss::thss;
    use test::{black_box, Bencher};
    use shared;

    macro_rules! bench_generate {
        ($name:ident, $k:expr, $n:expr, $secret:ident) => (
            #[bench]
            fn $name(b: &mut Bencher) {
                let secret = shared::$secret();

                b.iter(move || {
                    let shares = thss::split_secret($k, $n, &secret, &None).unwrap();
                    black_box(shares);
                });
            }
        )
    }

    macro_rules! bench_recover {
        ($name:ident, $k:expr, $n:expr, $secret:ident) => (
            #[bench]
            fn $name(b: &mut Bencher) {
                let secret = shared::$secret();
                let all_shares = thss::split_secret($k, $n, &secret, &None).unwrap();
                let shares = &all_shares.into_iter().take($k).collect::<Vec<_>>().clone();

                b.iter(|| {
                    let result = thss::recover_secret(&shares.to_vec()).unwrap();
                    black_box(result);
                });
            }
        )
    }

    bench_generate!(generate_1kb_3_5, 3, 5, secret_1kb);
    bench_recover!(recover_1kb_3_5, 3, 5, secret_1kb);

    bench_generate!(generate_1kb_10_25, 10, 25, secret_1kb);
    bench_recover!(recover_1kb_10_25, 10, 25, secret_1kb);

}
