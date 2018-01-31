use std::error::Error;

use protobuf::{self, Message};
use serialize::base64::{self, FromBase64, ToBase64};

use errors::*;
use proto::dss::{MetaDataProto, SecretProto, ShareProto};

fn base64_config() -> base64::Config {
    base64::Config {
        pad: false,
        ..base64::STANDARD
    }
}

pub(crate) fn format_share_protobuf(share: ShareProto) -> String {
    let bytes = share.write_to_bytes().unwrap();
    let base64_data = bytes.to_base64(base64_config());
    format!("{}-{}-{}", share.threshold, share.id, base64_data)
}

pub(crate) fn parse_share_protobuf(raw: String) -> Result<ShareProto> {
    let (threshold, id, base64_data) = parse_raw_share(raw)?;

    let data = base64_data.from_base64().chain_err(|| {
        ErrorKind::ShareParsingError("Base64 decoding of data block failed".to_string())
    })?;

    let share_proto = protobuf::parse_from_bytes::<ShareProto>(data.as_slice()).map_err(|e| {
        ErrorKind::ShareParsingError(format!(
            "Protobuf decoding of data block failed with error: {} .",
            e.description()
        ))
    })?;

    if threshold != share_proto.threshold {
        bail! {
            ErrorKind::ShareParsingError(
                format!(
                "Incompatible thresholds between decoded Protobuf provided \
                 (k={}) and raw share (k={})", share_proto.threshold, threshold
            )
        )}
    }

    if id != share_proto.id {
        bail! {
            ErrorKind::ShareParsingError(
                format!(
                "Incompatible ids between decoded Protobuf provided \
                 (i={}) and raw share (i={})", share_proto.id, id
            )
        )}
    }

    Ok(share_proto)
}

fn parse_raw_share(raw: String) -> Result<(u32, u32, String)> {
    let parts: Vec<_> = raw.trim().split('-').collect();

    if parts.len() != 3 {
        bail! {
            ErrorKind::ShareParsingError(
                format!(
                    "Expected 3 parts separated by a minus sign. Found {}.",
                    raw
                ),
            )
        };
    }

    let mut iter = parts.into_iter();
    let k = iter.next().unwrap().parse::<u32>()?;
    let i = iter.next().unwrap().parse::<u32>()?;
    let data = iter.next().unwrap();
    Ok((k, i, data.to_string()))
}
