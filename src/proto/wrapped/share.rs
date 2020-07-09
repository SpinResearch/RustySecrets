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
    pub shamir_data: ::std::vec::Vec<u8>,
    pub signature: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    pub proof: ::std::vec::Vec<u8>,
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

    // bytes shamir_data = 1;

    pub fn clear_shamir_data(&mut self) {
        self.shamir_data.clear();
    }

    // Param is passed by value, moved
    pub fn set_shamir_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.shamir_data = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_shamir_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.shamir_data
    }

    // Take field
    pub fn take_shamir_data(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.shamir_data, ::std::vec::Vec::new())
    }

    pub fn get_shamir_data(&self) -> &[u8] {
        &self.shamir_data
    }

    fn get_shamir_data_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.shamir_data
    }

    fn mut_shamir_data_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.shamir_data
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

    fn get_signature_for_reflect(&self) -> &::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &self.signature
    }

    fn mut_signature_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.signature
    }

    // bytes proof = 3;

    pub fn clear_proof(&mut self) {
        self.proof.clear();
    }

    // Param is passed by value, moved
    pub fn set_proof(&mut self, v: ::std::vec::Vec<u8>) {
        self.proof = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_proof(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.proof
    }

    // Take field
    pub fn take_proof(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.proof, ::std::vec::Vec::new())
    }

    pub fn get_proof(&self) -> &[u8] {
        &self.proof
    }

    fn get_proof_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.proof
    }

    fn mut_proof_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.proof
    }
}

impl ::protobuf::Message for ShareProto {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.shamir_data)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.signature)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.proof)?;
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
        if !self.shamir_data.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.shamir_data);
        }
        for value in &self.signature {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        if !self.proof.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.proof);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.shamir_data.is_empty() {
            os.write_bytes(1, &self.shamir_data)?;
        }
        for v in &self.signature {
            os.write_bytes(2, &v)?;
        };
        if !self.proof.is_empty() {
            os.write_bytes(3, &self.proof)?;
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
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "shamir_data",
                    ShareProto::get_shamir_data_for_reflect,
                    ShareProto::mut_shamir_data_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "signature",
                    ShareProto::get_signature_for_reflect,
                    ShareProto::mut_signature_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "proof",
                    ShareProto::get_proof_for_reflect,
                    ShareProto::mut_proof_for_reflect,
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
        self.clear_shamir_data();
        self.clear_signature();
        self.clear_proof();
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
    \n\x13wrapped/share.proto\x12\x07wrapped\"a\n\nShareProto\x12\x1f\n\x0bs\
    hamir_data\x18\x01\x20\x01(\x0cR\nshamirData\x12\x1c\n\tsignature\x18\
    \x02\x20\x03(\x0cR\tsignature\x12\x14\n\x05proof\x18\x03\x20\x01(\x0cR\
    \x05proofJ\x85\x02\n\x06\x12\x04\0\0\x08\x01\n\x08\n\x01\x0c\x12\x03\0\0\
    \x12\n\x08\n\x01\x02\x12\x03\x02\x08\x0f\n\n\n\x02\x04\0\x12\x04\x04\0\
    \x08\x01\n\n\n\x03\x04\0\x01\x12\x03\x04\x08\x12\n\x0b\n\x04\x04\0\x02\0\
    \x12\x03\x05\x08\x1e\n\r\n\x05\x04\0\x02\0\x04\x12\x04\x05\x08\x04\x14\n\
    \x0c\n\x05\x04\0\x02\0\x05\x12\x03\x05\x08\r\n\x0c\n\x05\x04\0\x02\0\x01\
    \x12\x03\x05\x0e\x19\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x05\x1c\x1d\n\
    \x0b\n\x04\x04\0\x02\x01\x12\x03\x06\x08%\n\x0c\n\x05\x04\0\x02\x01\x04\
    \x12\x03\x06\x08\x10\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x06\x11\x16\n\
    \x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x06\x17\x20\n\x0c\n\x05\x04\0\x02\
    \x01\x03\x12\x03\x06#$\n\x0b\n\x04\x04\0\x02\x02\x12\x03\x07\x08\x18\n\r\
    \n\x05\x04\0\x02\x02\x04\x12\x04\x07\x08\x06%\n\x0c\n\x05\x04\0\x02\x02\
    \x05\x12\x03\x07\x08\r\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x07\x0e\x13\
    \n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\x07\x16\x17b\x06proto3\
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
