use std::collections::{HashMap, HashSet};

use errors::*;
use share::{IsShare, IsSignedShare};

// The order of validation that we think makes the most sense is the following:
// 1) Validate shares individually
// 2) Validate duplicate shares share num && data
// 2) Validate group consistency
// 3) Validate other properties, in no specific order

/// TODO: Doc
pub(crate) fn validate_signed_shares<S: IsSignedShare>(
    shares: &Vec<S>,
    verify_signatures: bool,
) -> Result<(u8, usize)> {
    let result = validate_shares(shares)?;

    if verify_signatures {
        S::verify_signatures(&shares)?;
    }

    Ok(result)
}

/// TODO: Doc
pub(crate) fn validate_shares<S: IsShare>(shares: &Vec<S>) -> Result<(u8, usize)> {
    if shares.is_empty() {
        bail!(ErrorKind::EmptyShares);
    }

    let shares_count = shares.len();

    let mut ids = Vec::with_capacity(shares_count);
    let mut k_compatibility_sets = HashMap::new();
    let mut slen_compatibility_sets = HashMap::new();

    for share in shares {
        let (id, threshold, slen) = (
            share.get_id(),
            share.get_threshold(),
            share.get_data().len(),
        );

        if id < 1 {
            bail!(ErrorKind::ShareParsingInvalidShareId(id))
        } else if threshold < 2 {
            bail!(ErrorKind::ShareParsingInvalidShareThreshold(threshold, id))
        } else if slen < 1 {
            bail!(ErrorKind::ShareParsingErrorEmptyShare(id))
        }

        k_compatibility_sets
            .entry(threshold)
            .or_insert_with(HashSet::new);
        let k_set = k_compatibility_sets.get_mut(&threshold).unwrap();
        k_set.insert(id);

        if ids.iter().any(|&x| x == id) {
            bail!(ErrorKind::DuplicateShareId(id));
        }

        slen_compatibility_sets
            .entry(slen)
            .or_insert_with(HashSet::new);
        let slen_set = slen_compatibility_sets.get_mut(&slen).unwrap();
        slen_set.insert(id);

        ids.push(id);
    }

    // Validate threshold
    let k_sets = k_compatibility_sets.keys().count();

    match k_sets {
        1 => {} // All shares have the same roothash.
        _ => {
            bail! {
                ErrorKind::IncompatibleThresholds(
                    k_compatibility_sets
                        .values()
                        .map(|x| x.to_owned())
                        .collect(),
                )
            }
        }
    }

    // It is safe to unwrap because k_sets == 1
    let threshold = k_compatibility_sets.keys().last().unwrap().to_owned();

    if shares_count < threshold as usize {
        bail!(ErrorKind::MissingShares(shares_count, threshold));
    }

    // Validate share length consistency
    let slen_sets = slen_compatibility_sets.keys().count();

    match slen_sets {
        1 => {} // All shares have the same `data` field len
        _ => {
            bail! {
                ErrorKind::IncompatibleDataLengths(
                    slen_compatibility_sets
                        .values()
                        .map(|x| x.to_owned())
                        .collect(),
                )
            }
        }
    }

    // It is safe to unwrap because slen_sets == 1
    let slen = slen_compatibility_sets.keys().last().unwrap().to_owned();

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
