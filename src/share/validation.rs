use errors::*;
use share::{IsShare, IsSignedShare};

// The order of validation that we think makes the most sense is the following:
// 1) Validate shares individually
// 2) Validate duplicate shares share num && data
// 2) Validate group consistency
// 3) Validate other properties, in no specific order

/// TODO
pub(crate) fn validate_signed_shares<S: IsSignedShare>(
    shares: &Vec<S>,
    verify_signatures: bool,
) -> Result<(u8, usize)> {
    let result = validate_shares(shares)?;

    if verify_signatures {
        S::verify_signatures(&shares)?;
    };

    Ok(result)
}

pub(crate) fn begin_signed_share_validation<S: IsSignedShare>(
    shares: &Vec<S>,
    verify_signatures: bool,
) -> Result<(u8, usize, Vec<u8>, Option<Vec<u8>>)> {
    let (threshold, slen, ids) = _validate_shares(shares, None, None, None)?;

    let root_hash = if verify_signatures {
        Some(S::verify_signatures(&shares)?)
    } else {
        None
    };

    Ok((threshold, slen, ids, root_hash))
}

pub(crate) fn continue_signed_share_validation<S: IsSignedShare>(
    shares: &Vec<S>,
    already_verified_ids: &Vec<u8>,
    threshold: u8,
    slen: usize,
    root_hash: Option<&Vec<u8>>,
) -> Result<(Vec<u8>)> {
    let (_, _, new_ids) = _validate_shares(
        shares,
        Some(threshold),
        Some(slen),
        Some(already_verified_ids),
    )?;

    if root_hash.is_some() {
        S::continue_verify_signatures(shares, root_hash.unwrap(), already_verified_ids)?;
    }

    Ok(new_ids)
}

/// Validates a full set of shares.
pub(crate) fn validate_shares<S: IsShare>(shares: &Vec<S>) -> Result<(u8, usize)> {
    let (threshold, slen, _) = _validate_shares(shares, None, None, None)?;
    let shares_count = shares.len();
    if shares_count < threshold as usize {
        bail!(ErrorKind::MissingShares(shares_count, threshold))
    }
    Ok((threshold, slen))
}

/// TODO: Doc
fn _validate_shares<S: IsShare>(
    shares: &Vec<S>,
    threshold: Option<u8>,
    slen: Option<usize>,
    already_verified_ids: Option<&Vec<u8>>,
) -> Result<(u8, usize, Vec<u8>)> {
    if shares.is_empty() {
        bail!(ErrorKind::EmptyShares);
    }

    let shares_count = shares.len();
    let mut ids = if already_verified_ids.is_some() {
        let mut ids = already_verified_ids.unwrap().clone();
        ids.reserve_exact(shares_count);
        ids
    } else {
        Vec::with_capacity(shares_count)
    };
    let mut threshold = threshold.unwrap_or(0);
    let mut slen = slen.unwrap_or(0);

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
        }

        if threshold == 0 {
            threshold = threshold_;
        } else if threshold_ != threshold {
            bail!(ErrorKind::InconsistentThresholds(
                id,
                threshold_,
                ids,
                threshold
            ))
        }

        if slen == 0 {
            slen = slen_;
        } else if slen_ != slen {
            bail!(ErrorKind::InconsistentSecretLengths(id, slen_, ids, slen))
        }

        ids.push(id);
    }

    Ok((threshold, slen, ids))
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
