use custom_error::RustyError;
use merkle_sigs::verify_data_vec_signature;
use share_format;
use share_format::format_share_for_signing;
use std::error::Error;

pub fn process_and_validate_shares(shares_strings: Vec<String>,
                                   verify_signatures: bool)
                                   -> Result<(u8, Vec<(u8, Vec<u8>)>), RustyError> {

    let mut opt_k: Option<u8> = None;
    let mut opt_root_hash: Option<Vec<u8>> = None;

    let mut shares: Vec<(u8, Vec<u8>)> = Vec::new();

    for (counter, line) in shares_strings.iter().enumerate() {
        let share_index = counter as u8;
        let (share_data, k, n, sig_pair) = try!(share_format::share_from_string(line,
                                                                                counter as u8,
                                                                                verify_signatures));
        if verify_signatures {
            if sig_pair.is_none() {
                return Err(RustyError::new("Signature is missing while shares are required to be \
                                         signed.",
                                        None, Some(share_index)));
            }

            let (signature, p) = sig_pair.unwrap();
            let root_hash = p.root_hash.clone();

            if let Some(rh) = opt_root_hash.clone() {
                if root_hash != rh {
                    return Err(RustyError::new("Root hash not matching", None, Some(share_index)));
                }
                p.validate(&rh);
            } else {
                opt_root_hash = Some(root_hash.clone());
            }

            try!(verify_data_vec_signature(format_share_for_signing(k,
                                                                    n,
                                                                    &share_data.as_slice()),
                                                                    &(signature.to_vec(), p),
                                                                    &root_hash)
				 .map_err(|e| RustyError::new("Invalid signature", Some(String::from(e.description())), Some(share_index))));
        }

        if let Some(k_global) = opt_k {
            if k != k_global {
                return Err(RustyError::new("Incompatible shares", None, Some(share_index)));
            }
        } else {
            opt_k = Some(k);
        }

        if shares.iter().any(|s| s.0 == n) {
            return Err(RustyError::new("Duplicate Share Number", None, Some(share_index)));
        };

        if shares.iter().any(|s| s.1 == share_data) {
            return Err(RustyError::new("Duplicate Share Data", None, Some(share_index)));
        };

        shares.push((n, share_data));
        if counter + 1 == k as usize {
            return Ok((k, shares));
        }
    }
    Err(RustyError::new("Not enough shares provided!", None, None))
}
