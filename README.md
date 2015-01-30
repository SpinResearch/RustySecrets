# secretshare

This program is an implementation of Shamir's secret sharing.
A secret can be split into multiple shares and a selectable number of shares is required to reconstruct the secret again.

# Example

Passing a secret to secretshare for encoding:

```
$ echo This is a secret | secretshare -e2,5
2-1-cfFLZV0QABT0RmqOCFVxW/w
2-2-HkctX9qblUhW7EGutTxNKvs
2-3-O94PSafi5nzDilhF3htZBQ0
2-4-wDbhK8mQovAPpRfu0u41yPU
2-5-5a/DPbTp0cSaww4Fuckh5wM
```

The parameters following the `-e` option tell `secretshare` to create 5 shares of which 2 will be necessary for decoding.

Decoding a subset of shares (one share per line) can be done like this:

```
$ echo -e "2-2-HkctX9qblUhW7EGutTxNKvs \n 2-3-O94PSafi5nzDilhF3htZBQ0" | secretshare -d
This is a secret
```

# Building

This project is Cargo-enabled. So, you should be able to build it with

```
$ cargo build --release
```

