use base64;
use errors::*;
use merkle_sigs::{MerklePublicKey, Proof, PublicKey};
use proto::wrapped::ShareProto;
use protobuf::{self, Message, RepeatedField};
use sss::{Share, HASH_ALGO};
use std::error::Error;

const BASE64_CONFIG: base64::Config = base64::STANDARD_NO_PAD;

pub(crate) fn share_to_string(
    share: Vec<u8>,
    threshold: u8,
    share_num: u8,
    signature_pair: Option<(Vec<Vec<u8>>, Proof<MerklePublicKey>)>,
) -> String {
    let mut share_protobuf = ShareProto::new();
    share_protobuf.set_shamir_data(share);

    if signature_pair.is_some() {
        let (signature, proof) = signature_pair.unwrap();
        share_protobuf.set_signature(RepeatedField::from_vec(signature));
        share_protobuf.set_proof(proof.write_to_bytes().unwrap());
    }

    let proto_buf = share_protobuf.write_to_bytes().unwrap();
    let b64_share = base64::encode_config(&proto_buf, BASE64_CONFIG);
    format!("{}-{}-{}", threshold, share_num, b64_share)
}

pub(crate) fn share_from_string(s: &str, is_signed: bool) -> Result<Share> {
    let parts: Vec<_> = s.trim().split('-').collect();

    if parts.len() != SSS_SHARE_PARTS_COUNT {
        bail! {
            ErrorKind::ShareParsingError(
                format!(
                    "Expected 3 parts separated by a minus sign. Found {}.",
                    s
                ),
            )
        };
    }
    let (k, i, p3) = {
        let mut iter = parts.into_iter();
        let k = iter.next().unwrap().parse::<u8>()?;
        let i = iter.next().unwrap().parse::<u8>()?;
        let p3 = iter.next().unwrap();
        (k, i, p3)
    };

    if i < 1 {
        bail!(ErrorKind::ShareParsingInvalidShareId(i))
    } else if k < 2 {
        bail!(ErrorKind::ShareParsingInvalidShareThreshold(k, i))
    } else if p3.is_empty() {
        bail!(ErrorKind::ShareParsingErrorEmptyShare(i))
    }

    let raw_data = base64::decode_config(p3, BASE64_CONFIG).chain_err(|| {
        ErrorKind::ShareParsingError("Base64 decoding of data block failed".to_owned())
    })?;

    let protobuf_data =
        protobuf::parse_from_bytes::<ShareProto>(raw_data.as_slice()).map_err(|e| {
            ErrorKind::ShareParsingError(format!(
                "Protobuf decoding of data block failed with error: {} .",
                e.description()
            ))
        })?;

    let data = Vec::from(protobuf_data.get_shamir_data());

    let signature_pair = if is_signed {
        let p_result = Proof::parse_from_bytes(protobuf_data.get_proof(), HASH_ALGO);

        let p_opt = p_result.unwrap();
        let p = p_opt.unwrap();

        let proof = Proof {
            algorithm: HASH_ALGO,
            lemma: p.lemma,
            root_hash: p.root_hash,
            value: MerklePublicKey::new(PublicKey::from_vec(p.value, HASH_ALGO).unwrap()),
        };

        let signature = protobuf_data.get_signature();
        Some((Vec::from(signature), proof).into())
    } else {
        None
    };

    Ok(Share {
        id: i,
        data,
        threshold: k,
        signature_pair,
    })
}

pub(crate) fn format_share_for_signing(k: u8, i: u8, data: &[u8]) -> Vec<u8> {
    let b64_data = base64::encode_config(data, BASE64_CONFIG);
    format!("{}-{}-{}", k, i, b64_data).into_bytes()
}
