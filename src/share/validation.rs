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
    shares: Vec<S>,
    verify_signatures: bool,
) -> Result<(u8, Vec<S>)> {
    let (threshold, shares) = validate_shares(shares)?;

    if verify_signatures {
        S::verify_signatures(&shares)?;
    }

    Ok((threshold, shares))
}

/// TODO: Doc
pub(crate) fn validate_shares<S: IsShare>(shares: Vec<S>) -> Result<(u8, Vec<S>)> {
    if shares.is_empty() {
        bail!(ErrorKind::EmptyShares);
    }

    let shares_count = shares.len();
    let mut result: Vec<S> = Vec::with_capacity(shares_count);

    let mut k_compatibility_sets = HashMap::new();

    for share in shares {
        let (id, threshold) = (share.get_id(), share.get_threshold());

        if id > MAX_SHARES {
            bail!(ErrorKind::ShareIdentifierTooBig(id, MAX_SHARES,))
        }

        if id < 1 {
            bail!(ErrorKind::ShareParsingInvalidShareId(id,))
        }

        k_compatibility_sets.entry(threshold).or_insert_with(
            HashSet::new,
        );
        let k_set = k_compatibility_sets.get_mut(&threshold).unwrap();
        k_set.insert(id);

        if result.iter().any(|s| s.get_id() == id) {
            bail!(ErrorKind::DuplicateShareId(id));
        }

        if share.get_data().len() == 0 {
            bail!(ErrorKind::ShareParsingErrorEmptyShare(id,))
        }

        if result.iter().any(|s| s.get_data() == share.get_data()) && share.get_threshold() != 1 {
            // When threshold = 1, shares data can be the same
            bail!(ErrorKind::DuplicateShareData(id));
        }

        result.push(share);
    }

    // Validate threshold
    let k_sets = k_compatibility_sets.keys().count();

    match k_sets {
        0 => bail!(ErrorKind::EmptyShares),
        1 => {} // All shares have the same roothash.
        _ => {
            bail! {
                ErrorKind::IncompatibleSets(
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
        bail!(ErrorKind::MissingShares(threshold as usize, shares_count));
    }

    Ok((threshold, result))
}

pub(crate) fn validate_share_count(threshold: u8, total_shares_count: u8) -> Result<(u8,u8)> {
    if threshold < MIN_SHARES {
        bail!(ErrorKind::ThresholdTooSmall(
            threshold
        ));
    }
    if total_shares_count > MAX_SHARES {
        bail!(ErrorKind::InvalidShareCountMax(
            total_shares_count,
            MAX_SHARES,
        ));
    }
    if total_shares_count < MIN_SHARES {
        bail!(ErrorKind::InvalidShareCountMin(
            total_shares_count,
            MIN_SHARES,
        ));
    }
    if threshold > total_shares_count {
        bail!(ErrorKind::ThresholdTooBig(
            threshold,
            total_shares_count
        ));
    }

    Ok((threshold, total_shares_count))
}
