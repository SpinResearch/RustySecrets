use errors::*;
use share::{IsShare, IsSignedShare};

// The order of validation that we think makes the most sense is the following:
// 1) Validate shares individually
// 2) Validate duplicate shares share num && data
// 2) Validate group consistency
// 3) Validate other properties, in no specific order
pub(crate) fn validate_initial_signed_shares<S: IsSignedShare>(
    shares: &[S],
    verify_signatures: bool,
) -> Result<(u8, usize, Option<Vec<u8>>)> {
    if verify_signatures {
        let root_hash = vec![];
        validate_additional_signed_shares(shares, None, None, None, Some(&root_hash))
    } else {
        validate_additional_signed_shares(shares, None, None, None, None)
    }
}

pub(crate) fn validate_additional_signed_shares<S: IsSignedShare>(
    shares: &[S],
    threshold: Option<u8>,
    slen: Option<usize>,
    already_verified_ids: Option<&[u8]>,
    root_hash: Option<&[u8]>,
) -> Result<(u8, usize, Option<Vec<u8>>)> {
    let (threshold, slen) = validate_shares(shares, threshold, slen, already_verified_ids)?;

    let root_hash = if root_hash.is_some() {
        Some(S::verify_signatures(shares, root_hash)?)
    } else {
        None
    };

    Ok((threshold, slen, root_hash))
}

/// Does check there at at least threshold shares.
pub(crate) fn validate_all_signed_shares<S: IsSignedShare>(
    shares: &[S],
    verify_signature: bool,
) -> Result<(u8, usize)> {
    let result = validate_all_shares(shares)?;

    if verify_signature {
        S::verify_signatures(shares, None)?;
    }

    Ok(result)
}

/// Does check there at at least threshold shares.
pub(crate) fn validate_all_shares<S: IsShare>(shares: &[S]) -> Result<(u8, usize)> {
    let (threshold, slen) = validate_shares(shares, None, None, None)?;

    // Safe to cast because `validate_shares` ensures `len() < 255`.
    let shares_count = shares.len() as u8;
    if shares_count < threshold {
        bail!(ErrorKind::MissingShares(shares_count, threshold))
    }

    Ok((threshold, slen))
}

/// Does not check there at at least threshold shares.
pub(crate) fn validate_shares<S: IsShare>(
    shares: &[S],
    threshold: Option<u8>,
    slen: Option<usize>,
    already_verified_ids: Option<&[u8]>,
) -> Result<(u8, usize)> {
    if shares.is_empty() {
        bail!(ErrorKind::EmptyShares);
    }

    let shares_count = shares.len();
    let mut ids = if already_verified_ids.is_some() {
        let mut ids = already_verified_ids.unwrap().to_vec();
        ids.reserve_exact(shares_count);
        ids
    } else {
        Vec::with_capacity(shares_count)
    };
    // Safe to index since we already confirmed `shares` is nonempty.
    let threshold = threshold.unwrap_or_else(|| shares[0].get_threshold());
    let slen = slen.unwrap_or_else(|| shares[0].get_data().len());

    for share in shares {
        let id = share.get_id();
        let threshold_ = share.get_threshold();
        let slen_ = share.get_data().len();

        // Public-facing `Share::share_from_string` performs these three tests, but in case another
        // type which implements `IsShare` is implemented later that doesn't do that validation,
        // we'll leave them.
        if id < 1 {
            bail!(ErrorKind::ShareParsingInvalidShareId(id))
        } else if threshold_ < 2 {
            bail!(ErrorKind::ShareParsingInvalidShareThreshold(threshold, id))
        } else if slen_ < 1 {
            bail!(ErrorKind::ShareParsingErrorEmptyShare(id))
        }

        if ids.iter().any(|&x| x == id) {
            bail!(ErrorKind::DuplicateShareId(id));
        } else if threshold_ != threshold {
            bail!(ErrorKind::InconsistentThresholds(
                id,
                threshold_,
                ids,
                threshold
            ))
        } else if slen_ != slen {
            bail!(ErrorKind::InconsistentSecretLengths(id, slen_, ids, slen))
        }

        ids.push(id);
    }

    Ok((threshold, slen))
}

pub(crate) fn validate_share_count(threshold: u8, shares_count: u8) -> Result<(u8, u8)> {
    if threshold < MIN_SHARES {
        bail!(ErrorKind::ThresholdTooSmall(threshold));
    }
    if shares_count > MAX_SHARES {
        bail!(ErrorKind::InvalidShareCountMax(shares_count, MAX_SHARES));
    }
    if shares_count < MIN_SHARES {
        bail!(ErrorKind::InvalidShareCountMin(shares_count, MIN_SHARES));
    }
    if threshold > shares_count {
        bail!(ErrorKind::ThresholdTooBig(threshold, shares_count));
    }

    Ok((threshold, shares_count))
}
