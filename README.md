# secretshare

This program is an implementation of
[Shamir's secret sharing](https://en.wikipedia.org/wiki/Shamir%27s_Secret_Sharing).
A secret can be split into N shares in a way so that
a selectable number of shares K (with K ≤ N) is required
to reconstruct the secret again.

**Warning**: I don't yet recommend the serious use of this tool. The
encoding of the shares might change in a newer version in which case
you would have trouble decoding secrets that have been shared using
an older version of the program. For now, this is experimental.

# Example

Passing a secret to secretshare for encoding:

```
$ echo My secret | ./secretshare -e2,5
2-1-1YAYwmOHqZ69jA
2-2-YJZQDGm22Y77Gw
2-3-+G9ovW9SAnUynQ
2-4-F7rAjX3UOa53KA
2-5-j0P4PHsw4lW+rg
```

The parameters following the `-e` option tell `secretshare` to create 5 shares of which 2 will be necessary for decoding.

Decoding a subset of shares (one share per line) can be done like this:

```
$ echo -e "2-2-YJZQDGm22Y77Gw \n 2-4-F7rAjX3UOa53KA" | ./secretshare -d
My secret
```

# Building

This project is Cargo-enabled. So, you should be able to build it with

```
$ cargo build --release
```

once you have made sure that `rustc` (the compiler) and `cargo`
(the build and dependency management tool) are installed.
Visit the [Rust homepage](http://www.rust-lang.org/) if you are
don't know where to get these tools.

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

```
  2-1-LiTyeXwEP71IUA
  ^ ^ ^^^^^^^^^^^^^^
  K N        D        
```

A share is built out of three or four parts separated with a minus: K-N-D-C.
The last part is optional. K is one of the encoding parameters that tell you
how many distinct
shares of a specific secret are necessary to be able to recover the
secret. The number N identifies the share (ranging from 1 to the number
of shares that have been created). The D part is a Base64 encoding of
a specific share's raw data.

# A word on the secrecy

Shamir's secret sharing is known to have the perfect secrecy property.
In the context of (K,N)-threshold schemes this means that if you have
less than K shares available, you have absolutely no information about
what the secret is except for its length. The checksums that are included
in the shares
also don't reveal anything about the secret.
They are just a simple integrity protection of the shares themselves.
In other words, given a share without checksum, we can derive a share
with a checksum. This obviously does not add any new information.

# Galois field

Shamir's secret sharing algorithm requires the use of polynomials over
a finite field. One easy way of constructing a finite field is to pick
a prime number p, use the integers 0, 1, 2, ..., p-1 as field elements
and simply use modular arithmetic (mod p) for the field operations.

So, you *could* pick a prime like 257 to apply Shamir's algorithm
byte-wise. The downside of this is that the shares would consist of
sequences of values each between 0 and 256 *inclusive*. So, you would
need more than 8 bits to encode each of them.

But there is another way. We are not restricted to so-called
prime fields. There are also non-prime fields where the number of
elements is a *power* of a prime, for example 2^8=256. It's just
a bit harder to explain how they are constructed. The finite
field I used is the same as the one you can find in the RAID 6
implementation of the Linux kernel or the Anubis block cipher:
Gf(2^8) reduction polynomial is x^8 + x^4 + x^3 + x^2 + 1 or
alternatively 11D in hex.
