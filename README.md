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
