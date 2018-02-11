// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

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
pub struct SecretProto {
    // message fields
    pub version: super::version::VersionProto,
    pub secret: ::std::vec::Vec<u8>,
    pub mime_type: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SecretProto {}

impl SecretProto {
    pub fn new() -> SecretProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SecretProto {
        static mut instance: ::protobuf::lazy::Lazy<SecretProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SecretProto,
        };
        unsafe {
            instance.get(SecretProto::new)
        }
    }

    // .VersionProto version = 1;

    pub fn clear_version(&mut self) {
        self.version = super::version::VersionProto::INITIAL_RELEASE;
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: super::version::VersionProto) {
        self.version = v;
    }

    pub fn get_version(&self) -> super::version::VersionProto {
        self.version
    }

    fn get_version_for_reflect(&self) -> &super::version::VersionProto {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut super::version::VersionProto {
        &mut self.version
    }

    // bytes secret = 2;

    pub fn clear_secret(&mut self) {
        self.secret.clear();
    }

    // Param is passed by value, moved
    pub fn set_secret(&mut self, v: ::std::vec::Vec<u8>) {
        self.secret = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_secret(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.secret
    }

    // Take field
    pub fn take_secret(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.secret, ::std::vec::Vec::new())
    }

    pub fn get_secret(&self) -> &[u8] {
        &self.secret
    }

    fn get_secret_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.secret
    }

    fn mut_secret_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.secret
    }

    // string mime_type = 3;

    pub fn clear_mime_type(&mut self) {
        self.mime_type.clear();
    }

    // Param is passed by value, moved
    pub fn set_mime_type(&mut self, v: ::std::string::String) {
        self.mime_type = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_mime_type(&mut self) -> &mut ::std::string::String {
        &mut self.mime_type
    }

    // Take field
    pub fn take_mime_type(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.mime_type, ::std::string::String::new())
    }

    pub fn get_mime_type(&self) -> &str {
        &self.mime_type
    }

    fn get_mime_type_for_reflect(&self) -> &::std::string::String {
        &self.mime_type
    }

    fn mut_mime_type_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.mime_type
    }
}

impl ::protobuf::Message for SecretProto {
    fn is_initialized(&self) -> bool {
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
                    let tmp = is.read_enum()?;
                    self.version = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.secret)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.mime_type)?;
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
        if self.version != super::version::VersionProto::INITIAL_RELEASE {
            my_size += ::protobuf::rt::enum_size(1, self.version);
        }
        if !self.secret.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.secret);
        }
        if !self.mime_type.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.mime_type);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.version != super::version::VersionProto::INITIAL_RELEASE {
            os.write_enum(1, self.version.value())?;
        }
        if !self.secret.is_empty() {
            os.write_bytes(2, &self.secret)?;
        }
        if !self.mime_type.is_empty() {
            os.write_string(3, &self.mime_type)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SecretProto {
    fn new() -> SecretProto {
        SecretProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<SecretProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::version::VersionProto>>(
                    "version",
                    SecretProto::get_version_for_reflect,
                    SecretProto::mut_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "secret",
                    SecretProto::get_secret_for_reflect,
                    SecretProto::mut_secret_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "mime_type",
                    SecretProto::get_mime_type_for_reflect,
                    SecretProto::mut_mime_type_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SecretProto>(
                    "SecretProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SecretProto {
    fn clear(&mut self) {
        self.clear_version();
        self.clear_secret();
        self.clear_mime_type();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SecretProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SecretProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x14wrapped/secret.proto\x12\x07wrapped\x1a\rversion.proto\"k\n\x0bSec\
    retProto\x12'\n\x07version\x18\x01\x20\x01(\x0e2\r.VersionProtoR\x07vers\
    ion\x12\x16\n\x06secret\x18\x02\x20\x01(\x0cR\x06secret\x12\x1b\n\tmime_\
    type\x18\x03\x20\x01(\tR\x08mimeTypeJ\x91\x02\n\x06\x12\x04\0\0\n\x01\n\
    \x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x02\x08\x0f\n\t\n\
    \x02\x03\0\x12\x03\x04\x07\x16\n\n\n\x02\x04\0\x12\x04\x06\0\n\x01\n\n\n\
    \x03\x04\0\x01\x12\x03\x06\x08\x13\n\x0b\n\x04\x04\0\x02\0\x12\x03\x07\
    \x08!\n\r\n\x05\x04\0\x02\0\x04\x12\x04\x07\x08\x06\x15\n\x0c\n\x05\x04\
    \0\x02\0\x06\x12\x03\x07\x08\x14\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x07\
    \x15\x1c\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x07\x1f\x20\n\x0b\n\x04\x04\
    \0\x02\x01\x12\x03\x08\x08\x19\n\r\n\x05\x04\0\x02\x01\x04\x12\x04\x08\
    \x08\x07!\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x08\x08\r\n\x0c\n\x05\
    \x04\0\x02\x01\x01\x12\x03\x08\x0e\x14\n\x0c\n\x05\x04\0\x02\x01\x03\x12\
    \x03\x08\x17\x18\n\x0b\n\x04\x04\0\x02\x02\x12\x03\t\x08\x1d\n\r\n\x05\
    \x04\0\x02\x02\x04\x12\x04\t\x08\x08\x19\n\x0c\n\x05\x04\0\x02\x02\x05\
    \x12\x03\t\x08\x0e\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\t\x0f\x18\n\x0c\
    \n\x05\x04\0\x02\x02\x03\x12\x03\t\x1b\x1cb\x06proto3\
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
