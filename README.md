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

Decoding a subset (2nd and 3rd) of shares (one share per line):

```
$ echo -e "2-2-HkctX9qblUhW7EGutTxNKvs \n 2-3-O94PSafi5nzDilhF3htZBQ0" | secretshare -d
2-1-cfFLZV0QABT0RmqOCFVxW/w
2-2-HkctX9qblUhW7EGutTxNKvs
2-3-O94PSafi5nzDilhF3htZBQ0
2-4-wDbhK8mQovAPpRfu0u41yPU
2-5-5a/DPbTp0cSaww4Fuckh5wM
```

# Building

This project is Cargo-enabled. So, you should be able to built it with

```
$ cargo build --release
```

