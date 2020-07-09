// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct ShareProto {
    // message fields
    pub id: u32,
    pub threshold: u32,
    pub shares_count: u32,
    pub data: ::std::vec::Vec<u8>,
    pub hash: ::std::vec::Vec<u8>,
    pub meta_data: ::protobuf::SingularPtrField<super::metadata::MetaDataProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ShareProto {}

impl ShareProto {
    pub fn new() -> ShareProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ShareProto {
        static mut instance: ::protobuf::lazy::Lazy<ShareProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ShareProto,
        };
        unsafe {
            instance.get(ShareProto::new)
        }
    }

    // uint32 id = 1;

    pub fn clear_id(&mut self) {
        self.id = 0;
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u32) {
        self.id = v;
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    fn get_id_for_reflect(&self) -> &u32 {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut u32 {
        &mut self.id
    }

    // uint32 threshold = 2;

    pub fn clear_threshold(&mut self) {
        self.threshold = 0;
    }

    // Param is passed by value, moved
    pub fn set_threshold(&mut self, v: u32) {
        self.threshold = v;
    }

    pub fn get_threshold(&self) -> u32 {
        self.threshold
    }

    fn get_threshold_for_reflect(&self) -> &u32 {
        &self.threshold
    }

    fn mut_threshold_for_reflect(&mut self) -> &mut u32 {
        &mut self.threshold
    }

    // uint32 shares_count = 3;

    pub fn clear_shares_count(&mut self) {
        self.shares_count = 0;
    }

    // Param is passed by value, moved
    pub fn set_shares_count(&mut self, v: u32) {
        self.shares_count = v;
    }

    pub fn get_shares_count(&self) -> u32 {
        self.shares_count
    }

    fn get_shares_count_for_reflect(&self) -> &u32 {
        &self.shares_count
    }

    fn mut_shares_count_for_reflect(&mut self) -> &mut u32 {
        &mut self.shares_count
    }

    // bytes data = 4;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.data, ::std::vec::Vec::new())
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    fn get_data_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }

    // bytes hash = 5;

    pub fn clear_hash(&mut self) {
        self.hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.hash
    }

    // Take field
    pub fn take_hash(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.hash, ::std::vec::Vec::new())
    }

    pub fn get_hash(&self) -> &[u8] {
        &self.hash
    }

    fn get_hash_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.hash
    }

    fn mut_hash_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.hash
    }

    // .dss.MetaDataProto meta_data = 6;

    pub fn clear_meta_data(&mut self) {
        self.meta_data.clear();
    }

    pub fn has_meta_data(&self) -> bool {
        self.meta_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_meta_data(&mut self, v: super::metadata::MetaDataProto) {
        self.meta_data = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_meta_data(&mut self) -> &mut super::metadata::MetaDataProto {
        if self.meta_data.is_none() {
            self.meta_data.set_default();
        }
        self.meta_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_meta_data(&mut self) -> super::metadata::MetaDataProto {
        self.meta_data.take().unwrap_or_else(|| super::metadata::MetaDataProto::new())
    }

    pub fn get_meta_data(&self) -> &super::metadata::MetaDataProto {
        self.meta_data.as_ref().unwrap_or_else(|| super::metadata::MetaDataProto::default_instance())
    }

    fn get_meta_data_for_reflect(&self) -> &::protobuf::SingularPtrField<super::metadata::MetaDataProto> {
        &self.meta_data
    }

    fn mut_meta_data_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::metadata::MetaDataProto> {
        &mut self.meta_data
    }
}

impl ::protobuf::Message for ShareProto {
    fn is_initialized(&self) -> bool {
        for v in &self.meta_data {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.id = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.threshold = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.shares_count = tmp;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.data)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.hash)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.meta_data)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.id, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.threshold != 0 {
            my_size += ::protobuf::rt::value_size(2, self.threshold, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.shares_count != 0 {
            my_size += ::protobuf::rt::value_size(3, self.shares_count, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.data.is_empty() {
            my_size += ::protobuf::rt::bytes_size(4, &self.data);
        }
        if !self.hash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(5, &self.hash);
        }
        if let Some(ref v) = self.meta_data.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.id != 0 {
            os.write_uint32(1, self.id)?;
        }
        if self.threshold != 0 {
            os.write_uint32(2, self.threshold)?;
        }
        if self.shares_count != 0 {
            os.write_uint32(3, self.shares_count)?;
        }
        if !self.data.is_empty() {
            os.write_bytes(4, &self.data)?;
        }
        if !self.hash.is_empty() {
            os.write_bytes(5, &self.hash)?;
        }
        if let Some(ref v) = self.meta_data.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ShareProto {
    fn new() -> ShareProto {
        ShareProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ShareProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "id",
                    ShareProto::get_id_for_reflect,
                    ShareProto::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "threshold",
                    ShareProto::get_threshold_for_reflect,
                    ShareProto::mut_threshold_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "shares_count",
                    ShareProto::get_shares_count_for_reflect,
                    ShareProto::mut_shares_count_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    ShareProto::get_data_for_reflect,
                    ShareProto::mut_data_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "hash",
                    ShareProto::get_hash_for_reflect,
                    ShareProto::mut_hash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::metadata::MetaDataProto>>(
                    "meta_data",
                    ShareProto::get_meta_data_for_reflect,
                    ShareProto::mut_meta_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ShareProto>(
                    "ShareProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ShareProto {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_threshold();
        self.clear_shares_count();
        self.clear_data();
        self.clear_hash();
        self.clear_meta_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ShareProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ShareProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0fdss/share.proto\x12\x03dss\x1a\x12dss/metadata.proto\"\xb6\x01\n\n\
    ShareProto\x12\x0e\n\x02id\x18\x01\x20\x01(\rR\x02id\x12\x1c\n\tthreshol\
    d\x18\x02\x20\x01(\rR\tthreshold\x12!\n\x0cshares_count\x18\x03\x20\x01(\
    \rR\x0bsharesCount\x12\x12\n\x04data\x18\x04\x20\x01(\x0cR\x04data\x12\
    \x12\n\x04hash\x18\x05\x20\x01(\x0cR\x04hash\x12/\n\tmeta_data\x18\x06\
    \x20\x01(\x0b2\x12.dss.MetaDataProtoR\x08metaDataJ\xe3\x03\n\x06\x12\x04\
    \0\0\r\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x02\
    \x08\x0b\n\t\n\x02\x03\0\x12\x03\x04\x07\x1b\n\n\n\x02\x04\0\x12\x04\x06\
    \0\r\x01\n\n\n\x03\x04\0\x01\x12\x03\x06\x08\x12\n\x0b\n\x04\x04\0\x02\0\
    \x12\x03\x07\x02\x10\n\r\n\x05\x04\0\x02\0\x04\x12\x04\x07\x02\x06\x14\n\
    \x0c\n\x05\x04\0\x02\0\x05\x12\x03\x07\x02\x08\n\x0c\n\x05\x04\0\x02\0\
    \x01\x12\x03\x07\t\x0b\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x07\x0e\x0f\n\
    \x0b\n\x04\x04\0\x02\x01\x12\x03\x08\x02\x17\n\r\n\x05\x04\0\x02\x01\x04\
    \x12\x04\x08\x02\x07\x10\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x08\x02\
    \x08\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x08\t\x12\n\x0c\n\x05\x04\0\
    \x02\x01\x03\x12\x03\x08\x15\x16\n\x0b\n\x04\x04\0\x02\x02\x12\x03\t\x02\
    \x1a\n\r\n\x05\x04\0\x02\x02\x04\x12\x04\t\x02\x08\x17\n\x0c\n\x05\x04\0\
    \x02\x02\x05\x12\x03\t\x02\x08\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\t\t\
    \x15\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\t\x18\x19\n\x0b\n\x04\x04\0\
    \x02\x03\x12\x03\n\x02\x11\n\r\n\x05\x04\0\x02\x03\x04\x12\x04\n\x02\t\
    \x1a\n\x0c\n\x05\x04\0\x02\x03\x05\x12\x03\n\x02\x07\n\x0c\n\x05\x04\0\
    \x02\x03\x01\x12\x03\n\x08\x0c\n\x0c\n\x05\x04\0\x02\x03\x03\x12\x03\n\
    \x0f\x10\n\x0b\n\x04\x04\0\x02\x04\x12\x03\x0b\x02\x11\n\r\n\x05\x04\0\
    \x02\x04\x04\x12\x04\x0b\x02\n\x11\n\x0c\n\x05\x04\0\x02\x04\x05\x12\x03\
    \x0b\x02\x07\n\x0c\n\x05\x04\0\x02\x04\x01\x12\x03\x0b\x08\x0c\n\x0c\n\
    \x05\x04\0\x02\x04\x03\x12\x03\x0b\x0f\x10\n\x0b\n\x04\x04\0\x02\x05\x12\
    \x03\x0c\x02\"\n\r\n\x05\x04\0\x02\x05\x04\x12\x04\x0c\x02\x0b\x11\n\x0c\
    \n\x05\x04\0\x02\x05\x06\x12\x03\x0c\x02\x13\n\x0c\n\x05\x04\0\x02\x05\
    \x01\x12\x03\x0c\x14\x1d\n\x0c\n\x05\x04\0\x02\x05\x03\x12\x03\x0c\x20!b\
    \x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
