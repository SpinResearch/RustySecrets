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

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum VersionProto {
    INITIAL_RELEASE = 0,
}

impl ::protobuf::ProtobufEnum for VersionProto {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<VersionProto> {
        match value {
            0 => ::std::option::Option::Some(VersionProto::INITIAL_RELEASE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [VersionProto] = &[
            VersionProto::INITIAL_RELEASE,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<VersionProto>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("VersionProto", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for VersionProto {
}

impl ::std::default::Default for VersionProto {
    fn default() -> Self {
        VersionProto::INITIAL_RELEASE
    }
}

impl ::protobuf::reflect::ProtobufValue for VersionProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\rversion.proto*#\n\x0cVersionProto\x12\x13\n\x0fINITIAL_RELEASE\x10\0\
    JS\n\x06\x12\x04\x01\0\x05\x01\n\x08\n\x01\x0c\x12\x03\x01\0\x12\n\n\n\
    \x02\x05\0\x12\x04\x03\0\x05\x01\n\n\n\x03\x05\0\x01\x12\x03\x03\x05\x11\
    \n\x0b\n\x04\x05\0\x02\0\x12\x03\x04\x02\x16\n\x0c\n\x05\x05\0\x02\0\x01\
    \x12\x03\x04\x02\x11\n\x0c\n\x05\x05\0\x02\0\x02\x12\x03\x04\x14\x15b\
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
