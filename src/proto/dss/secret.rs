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

#[derive(PartialEq, Clone, Default)]
pub struct SecretProto {
    // message fields
    pub version: super::version::VersionProto,
    pub secret: ::std::vec::Vec<u8>,
    pub meta_data: ::protobuf::SingularPtrField<super::metadata::MetaDataProto>,
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
        unsafe { instance.get(SecretProto::new) }
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

    // .dss.MetaDataProto meta_data = 3;

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
        self.meta_data.take().unwrap_or_else(|| {
            super::metadata::MetaDataProto::new()
        })
    }

    pub fn get_meta_data(&self) -> &super::metadata::MetaDataProto {
        self.meta_data.as_ref().unwrap_or_else(|| {
            super::metadata::MetaDataProto::default_instance()
        })
    }

    fn get_meta_data_for_reflect(
        &self,
    ) -> &::protobuf::SingularPtrField<super::metadata::MetaDataProto> {
        &self.meta_data
    }

    fn mut_meta_data_for_reflect(
        &mut self,
    ) -> &mut ::protobuf::SingularPtrField<super::metadata::MetaDataProto> {
        &mut self.meta_data
    }
}

impl ::protobuf::Message for SecretProto {
    fn is_initialized(&self) -> bool {
        for v in &self.meta_data {
            if !v.is_initialized() {
                return false;
            }
        }
        true
    }

    fn merge_from(
        &mut self,
        is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(
                            ::protobuf::rt::unexpected_wire_type(wire_type),
                        );
                    }
                    let tmp = is.read_enum()?;
                    self.version = tmp;
                }
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(
                        wire_type,
                        is,
                        &mut self.secret,
                    )?;
                }
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.meta_data)?;
                }
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(
                        field_number,
                        wire_type,
                        is,
                        self.mut_unknown_fields(),
                    )?;
                }
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
        if let Some(ref v) = self.meta_data.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(
        &self,
        os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        if self.version != super::version::VersionProto::INITIAL_RELEASE {
            os.write_enum(1, self.version.value())?;
        }
        if !self.secret.is_empty() {
            os.write_bytes(2, &self.secret)?;
        }
        if let Some(ref v) = self.meta_data.as_ref() {
            os.write_tag(
                3,
                ::protobuf::wire_format::WireTypeLengthDelimited,
            )?;
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

impl ::protobuf::MessageStatic for SecretProto {
    fn new() -> SecretProto {
        SecretProto::new()
    }

    fn descriptor_static(
        _: ::std::option::Option<SecretProto>,
    ) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> =
            ::protobuf::lazy::Lazy {
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
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::metadata::MetaDataProto>>(
                    "meta_data",
                    SecretProto::get_meta_data_for_reflect,
                    SecretProto::mut_meta_data_for_reflect,
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
        self.clear_meta_data();
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
    \n\x10dss/secret.proto\x1a\rversion.proto\x1a\x12dss/metadata.proto\"\
    \x7f\n\x0bSecretProto\x12'\n\x07version\x18\x01\x20\x01(\x0e2\r.VersionP\
    rotoR\x07version\x12\x16\n\x06secret\x18\x02\x20\x01(\x0cR\x06secret\x12\
    /\n\tmeta_data\x18\x03\x20\x01(\x0b2\x12.dss.MetaDataProtoR\x08metaDataJ\
    \x92\x02\n\x06\x12\x04\0\0\t\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\t\n\
    \x02\x03\0\x12\x03\x02\x07\x16\n\t\n\x02\x03\x01\x12\x03\x03\x07\x1b\n\n\
    \n\x02\x04\0\x12\x04\x05\0\t\x01\n\n\n\x03\x04\0\x01\x12\x03\x05\x08\x13\
    \n\x0b\n\x04\x04\0\x02\0\x12\x03\x06\x08!\n\r\n\x05\x04\0\x02\0\x04\x12\
    \x04\x06\x08\x05\x15\n\x0c\n\x05\x04\0\x02\0\x06\x12\x03\x06\x08\x14\n\
    \x0c\n\x05\x04\0\x02\0\x01\x12\x03\x06\x15\x1c\n\x0c\n\x05\x04\0\x02\0\
    \x03\x12\x03\x06\x1f\x20\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x07\x08\x19\n\
    \r\n\x05\x04\0\x02\x01\x04\x12\x04\x07\x08\x06!\n\x0c\n\x05\x04\0\x02\
    \x01\x05\x12\x03\x07\x08\r\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x07\x0e\
    \x14\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x07\x17\x18\n\x0b\n\x04\x04\0\
    \x02\x02\x12\x03\x08\x08(\n\r\n\x05\x04\0\x02\x02\x04\x12\x04\x08\x08\
    \x07\x19\n\x0c\n\x05\x04\0\x02\x02\x06\x12\x03\x08\x08\x19\n\x0c\n\x05\
    \x04\0\x02\x02\x01\x12\x03\x08\x1a#\n\x0c\n\x05\x04\0\x02\x02\x03\x12\
    \x03\x08&'b\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe { file_descriptor_proto_lazy.get(|| parse_descriptor_proto()) }
}
