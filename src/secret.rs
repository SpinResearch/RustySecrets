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
pub struct RustySecret {
    // message fields
    version: ::std::option::Option<RustySecretsVersions>,
    secret: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    mime_type: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RustySecret {}

impl RustySecret {
    pub fn new() -> RustySecret {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RustySecret {
        static mut instance: ::protobuf::lazy::Lazy<RustySecret> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RustySecret,
        };
        unsafe {
            instance.get(|| {
                RustySecret {
                    version: ::std::option::Option::None,
                    secret: ::protobuf::SingularField::none(),
                    mime_type: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .RustySecretsVersions version = 1;

    pub fn clear_version(&mut self) {
        self.version = ::std::option::Option::None;
    }

    pub fn has_version(&self) -> bool {
        self.version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: RustySecretsVersions) {
        self.version = ::std::option::Option::Some(v);
    }

    pub fn get_version(&self) -> RustySecretsVersions {
        self.version.unwrap_or(RustySecretsVersions::INITIAL_RELEASE)
    }

    // optional bytes secret = 2;

    pub fn clear_secret(&mut self) {
        self.secret.clear();
    }

    pub fn has_secret(&self) -> bool {
        self.secret.is_some()
    }

    // Param is passed by value, moved
    pub fn set_secret(&mut self, v: ::std::vec::Vec<u8>) {
        self.secret = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_secret(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.secret.is_none() {
            self.secret.set_default();
        };
        self.secret.as_mut().unwrap()
    }

    // Take field
    pub fn take_secret(&mut self) -> ::std::vec::Vec<u8> {
        self.secret.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_secret(&self) -> &[u8] {
        match self.secret.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional string mime_type = 3;

    pub fn clear_mime_type(&mut self) {
        self.mime_type.clear();
    }

    pub fn has_mime_type(&self) -> bool {
        self.mime_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_mime_type(&mut self, v: ::std::string::String) {
        self.mime_type = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_mime_type(&mut self) -> &mut ::std::string::String {
        if self.mime_type.is_none() {
            self.mime_type.set_default();
        };
        self.mime_type.as_mut().unwrap()
    }

    // Take field
    pub fn take_mime_type(&mut self) -> ::std::string::String {
        self.mime_type.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_mime_type(&self) -> &str {
        match self.mime_type.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for RustySecret {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.version = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.secret));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.mime_type));
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
        for value in &self.version {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in &self.secret {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        for value in &self.mime_type {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.version {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.secret.as_ref() {
            try!(os.write_bytes(2, &v));
        };
        if let Some(v) = self.mime_type.as_ref() {
            try!(os.write_string(3, &v));
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
        ::std::any::TypeId::of::<RustySecret>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RustySecret {
    fn new() -> RustySecret {
        RustySecret::new()
    }

    fn descriptor_static(_: ::std::option::Option<RustySecret>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "version",
                    RustySecret::has_version,
                    RustySecret::get_version,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "secret",
                    RustySecret::has_secret,
                    RustySecret::get_secret,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "mime_type",
                    RustySecret::has_mime_type,
                    RustySecret::get_mime_type,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RustySecret>(
                    "RustySecret",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RustySecret {
    fn clear(&mut self) {
        self.clear_version();
        self.clear_secret();
        self.clear_mime_type();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RustySecret {
    fn eq(&self, other: &RustySecret) -> bool {
        self.version == other.version &&
        self.secret == other.secret &&
        self.mime_type == other.mime_type &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RustySecret {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum RustySecretsVersions {
    INITIAL_RELEASE = 0,
}

impl ::protobuf::ProtobufEnum for RustySecretsVersions {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<RustySecretsVersions> {
        match value {
            0 => ::std::option::Option::Some(RustySecretsVersions::INITIAL_RELEASE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [RustySecretsVersions] = &[
            RustySecretsVersions::INITIAL_RELEASE,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<RustySecretsVersions>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("RustySecretsVersions", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for RustySecretsVersions {
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0c, 0x53, 0x65, 0x63, 0x72, 0x65, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x73,
    0x0a, 0x0b, 0x52, 0x75, 0x73, 0x74, 0x79, 0x53, 0x65, 0x63, 0x72, 0x65, 0x74, 0x12, 0x2f, 0x0a,
    0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x15,
    0x2e, 0x52, 0x75, 0x73, 0x74, 0x79, 0x53, 0x65, 0x63, 0x72, 0x65, 0x74, 0x73, 0x56, 0x65, 0x72,
    0x73, 0x69, 0x6f, 0x6e, 0x73, 0x52, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x16,
    0x0a, 0x06, 0x73, 0x65, 0x63, 0x72, 0x65, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x06,
    0x73, 0x65, 0x63, 0x72, 0x65, 0x74, 0x12, 0x1b, 0x0a, 0x09, 0x6d, 0x69, 0x6d, 0x65, 0x5f, 0x74,
    0x79, 0x70, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x6d, 0x69, 0x6d, 0x65, 0x54,
    0x79, 0x70, 0x65, 0x2a, 0x2b, 0x0a, 0x14, 0x52, 0x75, 0x73, 0x74, 0x79, 0x53, 0x65, 0x63, 0x72,
    0x65, 0x74, 0x73, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x12, 0x13, 0x0a, 0x0f, 0x49,
    0x4e, 0x49, 0x54, 0x49, 0x41, 0x4c, 0x5f, 0x52, 0x45, 0x4c, 0x45, 0x41, 0x53, 0x45, 0x10, 0x00,
    0x4a, 0xbd, 0x02, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x0a, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c,
    0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04, 0x02, 0x00, 0x04,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x00, 0x01, 0x12, 0x03, 0x02, 0x05, 0x19, 0x0a, 0x0b, 0x0a,
    0x04, 0x05, 0x00, 0x02, 0x00, 0x12, 0x03, 0x03, 0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x03, 0x08, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00,
    0x02, 0x12, 0x03, 0x03, 0x1a, 0x1b, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x06, 0x00,
    0x0a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x06, 0x08, 0x13, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x07, 0x08, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x07, 0x08, 0x06, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x06, 0x12, 0x03, 0x07, 0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x07, 0x1d, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x07, 0x27, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x08, 0x08,
    0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0x08, 0x08, 0x07, 0x29,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x08, 0x08, 0x0d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x08, 0x0e, 0x14, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x08, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x02, 0x12, 0x03, 0x09, 0x08, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04,
    0x12, 0x04, 0x09, 0x08, 0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12,
    0x03, 0x09, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x09,
    0x0f, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x09, 0x1b, 0x1c,
    0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
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
