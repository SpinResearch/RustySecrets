# Rusty Secrets [![Build Status](https://travis-ci.org/SpinResearch/RustySecrets.svg?branch=master)](https://travis-ci.org/SpinResearch/RustySecrets) [![Coverage Status](https://coveralls.io/repos/github/SpinResearch/RustySecrets/badge.svg?branch=master)](https://coveralls.io/github/SpinResearch/RustySecrets?branch=master)

> Rusty Secrets is an implementation of a threshold [Shamir's secret sharing scheme](https://en.wikipedia.org/wiki/Shamir%27s_Secret_Sharing).

[**Documentation (latest)**](https://docs.rs/rusty_secrets/)  
[**Documentation (master)**](http://spinresearch.github.io/RustySecrets/rusty_secrets/index.html)

## Warning: Please do not send pull requests for the moment

> This library is currently being heavily refactored in a private branch, and as such, we are currently unable to merge pull requests against the master branch. We will release the changes in this private branch publicly as soon as we get them audited. Until then, we definitely welcome bug reports, constructive feedback, or feature ideas.

> Thank you for checking out RustySecrets, we hope to be able to accept your contributions very soon!

## Design goals

The main use for this library is to split a secret of an arbitrary length in *n* different shares and *t*-out-of-*n* shares are required to recover it. The dealer is assumed to be honest (and competent). We further assume that our adversary will only be able to compromise at most *t-1* shares. Shares are kept offline.

A typical use case for this library would be splitting an encryption key to a TrueCrypt-like volume.

## Implementation

### Structure of the shares

```
  2-1-LiTyeXwEP71IUA
  ^ ^ ^^^^^^^^^^^^^^
  K N        D        
```

A share is built out of three parts separated with a dash: K-N-D.

- K specifies the number of shares necessary to recover the secret.
- N is the identifier of the share and varies between 1 and n where n is the total number of generated shares.
- The D part is a Base64 encoding of a `ShareData` protobuf containing information about the share, and if signed, the signature.

### Signatures

There are a few issues with regular Shamir's secret sharing that we wanted to address:

- a share can be corrupted or incorrectly entered. 
- a malicious share holder can modify the secret that would be recovered by modifying his share.
- a user has multiple shares from different secret shares and he doesn't know which one belongs to a specific instance.

All of these issues would result in a corrupted secret being outputted and the program, that wouldn't even know that the secret got corrupted, wouldn't be able to give any actionable information.

We addressed this by signing the shares by the dealer and encoding the public key into each share. After the generation of the shares, the dealer erases both the secret and the private signing key used to sign the shares. When recovering the secret, the program verifies that public keys and if some shares do not have the same public key, or a valid signature of that public key, signals the issue to the user with a helpful message.

Signing shares is optional and the usefulness of signing the shares depends on the use case. Since we're using hash-based signatures (using SHA-512 Merkle signing), there is a large overhead from using signatures.

## Bug Reporting

Please report bugs either as pull requests or as issues in [the issue
tracker](https://github.com/SpinResearch/RustySecrets/issues). *RustySecrets* has a
**full disclosure** vulnerability policy. **Please do NOT attempt to report
any security vulnerability in this code privately to anybody.**

## License

See [LICENSE](LICENSE).

## Vocabulary

- Dealer: Entity that will perform key splitting from a master secret
- Shares: Part of the split secret distributed

## Credits

Rusty Secrets was forked off [sellibitze's secretshare.](https://github.com/sellibitze/secretshare)
