# secretshare

This program is an implementation of Shamir's secret sharing.
A secret can be split into multiple shares and a selectable number of shares is required to reconstruct the secret again.

# Example

Passing a secret to secretshare for encoding:

```
$ echo My secret | ./secretshare -e2,5
2-1-LiTyeXwEP71IUA-Qj6n
2-2-i8OZZ1et6MgMvg-xwsJ
2-3-6J5LbU7KpRAw5A-27nn
2-4-3BBPWwHiWyKEfw-0ADd
2-5-v02dURiFFvq4JQ-zLIz
```

The parameters following the `-e` option tell `secretshare` to create 5 shares of which 2 will be necessary for decoding.

Decoding a subset of shares (one share per line) can be done like this:

```
$ echo -e "2-2-i8OZZ1et6MgMvg-xwsJ \n 2-4-3BBPWwHiWyKEfw-0ADd" | ./secretshare -d
My secret
```

# Building

This project is Cargo-enabled. So, you should be able to build it with

```
$ cargo build --release
```

once you have made sure that `rustc` (the compiler) and `cargo`
(the build and dependency management tool) are installed.
Visit the [Rust homepage](http://www.rust-lang.org/) if you don't
know what they are.

# I/O

The secret data does not have to be text. `secretshare` treats it as
binary data. But, of course, you can feed it text as well. In the above
example the echo command terminated the string with a line feed which
is actually part of the secret and output as well after decoding.
Note that, while `secretshare` supports secrets of up to 64 KiB
it makes little sense to use such large secrets directly. In situations
where you want to share larger secrets, you would usually pick a random
password for encryption and use that password as secret for `secretshare`.

The generated shares are lines of ASCII text.

# Structure of the shares

A share is built out of three or four parts: K-N-D-C (C is optional).
K is one of the encoding parameters that tell you how many distinct
shares of a specific secret are necessary to be able to recover the
secret. The number N identifies the share (ranging from 1 to the number
of shares that have been created). The D part is a Base64 encoding of
a specific share's raw data. The optional part C is a Base64 encoding
of a CRC-24 checksum of the share's data.

# I/O

