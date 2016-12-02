use custom_error::RustyError;
use custom_error::pie2error;
use digest;
use merkle_sigs::Proof;
use merkle_sigs::PublicKey;
use protobuf;
use protobuf::{Message, RepeatedField};
use serialize;
use serialize::base64::{self, FromBase64, ToBase64};
use share_data::ShareData;

fn base64_config() -> serialize::base64::Config {
    base64::Config { pad: false, ..base64::STANDARD }
}

pub fn share_string_from(share: Vec<u8>,
                         threshold: u8,
                         share_num: u8,
                         signature_pair: Option<(Vec<Vec<u8>>, Proof<PublicKey>)>)
                         -> String {
    let mut share_protobuf = ShareData::new();
    share_protobuf.set_shamir_data(share);

    if signature_pair.is_some() {
        let (signature, proof) = signature_pair.unwrap();
        share_protobuf.set_signature(RepeatedField::from_vec(signature));
        share_protobuf.set_proof(proof.write_to_bytes().unwrap());
    }

    let b64_share = share_protobuf.write_to_bytes().unwrap().to_base64(base64_config());
    format!("{}-{}-{}", threshold, share_num, b64_share)
}

pub fn share_from_string
    (s: &str,
     is_signed: bool)
     -> Result<(Vec<u8>, u8, u8, Option<(Vec<Vec<u8>>, Proof<PublicKey>)>), RustyError> {
    let parts: Vec<_> = s.trim().split('-').collect();

    if parts.len() != 3 {
        return Err(RustyError::new("Share parse error: Expected 3 parts separated by a minus sign",
                                None, None));
    }
    let (k, n, p3) = {
        let mut iter = parts.into_iter();
        let k = try!(iter.next().unwrap().parse::<u8>().map_err(pie2error));
        let n = try!(iter.next().unwrap().parse::<u8>().map_err(pie2error));
        let p3 = iter.next().unwrap();
        (k, n, p3)
    };
    if k < 1 || n < 1 {
        return Err(RustyError::new("Share parse error: Illegal K,N parameters", None, None));
    }

    let raw_data = try!(p3.from_base64().map_err(|_| {
        RustyError::new("Share parse error: Base64 decoding of data block failed",
                     None, None)
    }));

    let protobuf_data = try!(protobuf::parse_from_bytes::<ShareData>(raw_data.as_slice())
        .map_err(|_| RustyError::new("Share parse error: Protobuffer could not be decoded.", None, None)));


    let share = Vec::from(protobuf_data.get_shamir_data());

    if is_signed {
        let p = Proof::parse_from_bytes(protobuf_data.get_proof(), digest).unwrap().unwrap();

        let proof = Proof {
            algorithm: digest,
            lemma: p.lemma,
            root_hash: p.root_hash,
            value: PublicKey::from_vec(p.value, digest).unwrap(),
        };

        let signature = protobuf_data.get_signature();

        Ok((share, k, n, Some((Vec::from(signature), proof))))
    } else {
        Ok((share, k, n, None))
    }
}

pub fn format_share_for_signing(k: u8, i: u8, data: &[u8]) -> Vec<u8> {
    format!("{}-{}-{}", k, i, data.to_base64(base64_config())).into_bytes()
}
