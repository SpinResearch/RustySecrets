# Rusty Secrets [![Build Status](https://travis-ci.org/freedomofpress/RustySecrets.svg?branch=master)](https://travis-ci.org/freedomofpress/RustySecrets) [![Coverage Status](https://coveralls.io/repos/github/freedomofpress/RustySecrets/badge.svg?branch=master)](https://coveralls.io/github/freedomofpress/RustySecrets?branch=master)

[**Documentation**](http://freedomofpress.github.io/RustySecrets/rusty_secrets/index.html)

Rusty Secrets is an implementation of a threshold [Shamir's secret sharing scheme](https://en.wikipedia.org/wiki/Shamir%27s_Secret_Sharing).

## Design goals

The main use for this library is to split a secret of an arbitrary length in n different shares and t-out-of-n shares are required to recover it. The dealer is assumed to be honest (and competent). We further assume that our adversary will only be able to compromise at most k-1 shares.

## Choosing a scheme

The Shamir's Secret Sharing scheme has been chosen for this implementation for the following reasons.

### Information-theoretic security

Shamir's secret sharing is known to have the perfect secrecy property.
In the context of (K,N)-threshold schemes this means that if you have
less than K shares available, you have absolutely no information about
what the secret is except for its length (typical secrets would be an AES-256 key, all have the same length).

Information-theoretic security gives us strong guarantees:

1) That there are provably no faster attacks than brute force exhaustion of key space.
2) An encryption protocol that has information-theoretic security does not depend for its effectiveness on unproven assumptions about computational hardness, and such an algorithm is not vulnerable to future developments in computer power such as quantum computing. Source: [Wikipedia]( https://en.wikipedia.org/wiki/Information-theoretic_security “Information Theoretic Security”)


### Peer-review

The Shamir secret sharing scheme has been around since 1979 and has been [well studied](https://scholar.google.ch/scholar?cites=12714240754634232446&as_sdt=2005&sciodt=0,5&hl=en).

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
- The D part is a Base64 encoding of a specific share's raw data.

### Command-line encoding

Passing a secret to rustysecrets for encoding:

```
$ echo My secret | ./rusty_secrets_bin -e2,5
2-1-1YAYwmOHqZ69jA
2-2-YJZQDGm22Y77Gw
2-3-+G9ovW9SAnUynQ
2-4-F7rAjX3UOa53KA
2-5-j0P4PHsw4lW+rg
```

The parameters following the `-e` option tell rustysecrets to create 5 shares of which 2 will be necessary for decoding.

Decoding a subset of shares (one share per line) can be done like this:

```
$ echo -e "2-2-YJZQDGm22Y77Gw \n 2-4-F7rAjX3UOa53KA" | ./rusty_secrets_bin -d
My secret
```

## Vocabulary

- Dealer: Entity that will perform key splitting from a master secret
- Shares: Part of the split secret distributed

## Credits

Rusty Secrets was forked off [sellibitze's secretshare.](https://github.com/sellibitze/secretshare)
