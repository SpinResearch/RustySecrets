// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct ShareData {
    // message fields
    shamir_data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    signature: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    proof: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ShareData {}

impl ShareData {
    pub fn new() -> ShareData {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ShareData {
        static mut instance: ::protobuf::lazy::Lazy<ShareData> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ShareData,
        };
        unsafe {
            instance.get(|| {
                ShareData {
                    shamir_data: ::protobuf::SingularField::none(),
                    signature: ::protobuf::RepeatedField::new(),
                    proof: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes shamir_data = 1;

    pub fn clear_shamir_data(&mut self) {
        self.shamir_data.clear();
    }

    pub fn has_shamir_data(&self) -> bool {
        self.shamir_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_shamir_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.shamir_data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_shamir_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.shamir_data.is_none() {
            self.shamir_data.set_default();
        };
        self.shamir_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_shamir_data(&mut self) -> ::std::vec::Vec<u8> {
        self.shamir_data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_shamir_data(&self) -> &[u8] {
        match self.shamir_data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // repeated bytes signature = 2;

    pub fn clear_signature(&mut self) {
        self.signature.clear();
    }

    // Param is passed by value, moved
    pub fn set_signature(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.signature = v;
    }

    // Mutable pointer to the field.
    pub fn mut_signature(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.signature
    }

    // Take field
    pub fn take_signature(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.signature, ::protobuf::RepeatedField::new())
    }

    pub fn get_signature(&self) -> &[::std::vec::Vec<u8>] {
        &self.signature
    }

    // optional bytes proof = 3;

    pub fn clear_proof(&mut self) {
        self.proof.clear();
    }

    pub fn has_proof(&self) -> bool {
        self.proof.is_some()
    }

    // Param is passed by value, moved
    pub fn set_proof(&mut self, v: ::std::vec::Vec<u8>) {
        self.proof = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_proof(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.proof.is_none() {
            self.proof.set_default();
        };
        self.proof.as_mut().unwrap()
    }

    // Take field
    pub fn take_proof(&mut self) -> ::std::vec::Vec<u8> {
        self.proof.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_proof(&self) -> &[u8] {
        match self.proof.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for ShareData {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.shamir_data));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.signature));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.proof));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.shamir_data {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in &self.signature {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        for value in &self.proof {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.shamir_data.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        for v in &self.signature {
            try!(os.write_bytes(2, &v));
        };
        if let Some(v) = self.proof.as_ref() {
            try!(os.write_bytes(3, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ShareData>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ShareData {
    fn new() -> ShareData {
        ShareData::new()
    }

    fn descriptor_static(_: ::std::option::Option<ShareData>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "shamir_data",
                    ShareData::has_shamir_data,
                    ShareData::get_shamir_data,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_bytes_accessor(
                    "signature",
                    ShareData::get_signature,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "proof",
                    ShareData::has_proof,
                    ShareData::get_proof,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ShareData>(
                    "ShareData",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ShareData {
    fn clear(&mut self) {
        self.clear_shamir_data();
        self.clear_signature();
        self.clear_proof();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ShareData {
    fn eq(&self, other: &ShareData) -> bool {
        self.shamir_data == other.shamir_data &&
        self.signature == other.signature &&
        self.proof == other.proof &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ShareData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x18, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x53, 0x68, 0x61, 0x72, 0x65,
    0x44, 0x61, 0x74, 0x61, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x60, 0x0a, 0x09, 0x53, 0x68,
    0x61, 0x72, 0x65, 0x44, 0x61, 0x74, 0x61, 0x12, 0x1f, 0x0a, 0x0b, 0x73, 0x68, 0x61, 0x6d, 0x69,
    0x72, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x0a, 0x73, 0x68,
    0x61, 0x6d, 0x69, 0x72, 0x44, 0x61, 0x74, 0x61, 0x12, 0x1c, 0x0a, 0x09, 0x73, 0x69, 0x67, 0x6e,
    0x61, 0x74, 0x75, 0x72, 0x65, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0c, 0x52, 0x09, 0x73, 0x69, 0x67,
    0x6e, 0x61, 0x74, 0x75, 0x72, 0x65, 0x12, 0x14, 0x0a, 0x05, 0x70, 0x72, 0x6f, 0x6f, 0x66, 0x18,
    0x03, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x05, 0x70, 0x72, 0x6f, 0x6f, 0x66, 0x4a, 0xfb, 0x01, 0x0a,
    0x06, 0x12, 0x04, 0x00, 0x00, 0x06, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00,
    0x12, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x02, 0x00, 0x06, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x02, 0x08, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x03, 0x08, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12,
    0x04, 0x03, 0x08, 0x02, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x03, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x03, 0x0e,
    0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x03, 0x1c, 0x1d, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x04, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x04, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x04, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x04, 0x17, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x04, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x05, 0x08,
    0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x04, 0x05, 0x08, 0x04, 0x25,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x05, 0x08, 0x0d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x05, 0x0e, 0x13, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x05, 0x16, 0x17, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x33,
];

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
