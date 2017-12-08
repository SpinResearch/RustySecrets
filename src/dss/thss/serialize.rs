
use errors::*;
use super::{Share, MetaData};
use dss::format::{format_share_protobuf, parse_share_protobuf};
use proto::dss::{ShareProto, MetaDataProto};
use dss::utils::{btreemap_to_hashmap, hashmap_to_btreemap};

pub(crate) fn share_to_string(share: Share) -> String {
    let proto = share_to_protobuf(share);
    format_share_protobuf(proto)
}

pub(crate) fn share_from_string(raw: String) -> Result<Share> {
    let mut proto = parse_share_protobuf(raw)?;

    let metadata_proto = if proto.has_meta_data() {
        Some(metadata_from_proto(proto.take_meta_data()))
    } else {
        None
    };

    let i = proto.get_id() as u8;
    let k = proto.get_threshold() as u8;
    let n = proto.get_shares_count() as u8;

    if k < 1 || i < 1 {
        bail! {
            ErrorKind::ShareParsingError(
                format!("Found illegal share info: threshold = {}, identifier = {}.", k, i),
            )
        }
    }

    if n < 1 || k > n || i > n {
        bail! {
            ErrorKind::ShareParsingError(
                format!("Found illegal share info: shares_count = {}, threshold = {}, identifier = {}.", n, k, i),
            )
        }
    }

    let share = Share {
        id: i,
        threshold: k,
        shares_count: n, // FIXME
        data: proto.take_data(), // FIXME
        metadata: metadata_proto, // FIXME
    };

    Ok(share)
}

pub(crate) fn share_to_protobuf(share: Share) -> ShareProto {
    let mut proto = ShareProto::new();

    proto.set_id(share.id.into());
    proto.set_threshold(share.threshold.into());
    proto.set_shares_count(share.shares_count.into());
    proto.set_data(share.data);

    if let Some(meta_data) = share.metadata {
        let metadata_proto = metadata_to_proto(meta_data);
        proto.set_meta_data(metadata_proto);
    }

    proto
}

fn metadata_to_proto(meta_data: MetaData) -> MetaDataProto {
    let mut proto = MetaDataProto::new();
    proto.set_tags(btreemap_to_hashmap(meta_data.tags));
    proto
}

fn metadata_from_proto(mut proto: MetaDataProto) -> MetaData {
    MetaData { tags: hashmap_to_btreemap(proto.take_tags()) }
}
