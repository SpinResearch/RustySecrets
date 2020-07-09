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
pub struct MetaDataProto {
    // message fields
    pub tags: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MetaDataProto {}

impl MetaDataProto {
    pub fn new() -> MetaDataProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MetaDataProto {
        static mut instance: ::protobuf::lazy::Lazy<MetaDataProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MetaDataProto,
        };
        unsafe { instance.get(MetaDataProto::new) }
    }

    // repeated .dss.MetaDataProto.TagsEntry tags = 1;

    pub fn clear_tags(&mut self) {
        self.tags.clear();
    }

    // Param is passed by value, moved
    pub fn set_tags(
        &mut self,
        v: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    ) {
        self.tags = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tags(
        &mut self,
    ) -> &mut ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &mut self.tags
    }

    // Take field
    pub fn take_tags(
        &mut self,
    ) -> ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        ::std::mem::replace(&mut self.tags, ::std::collections::HashMap::new())
    }

    pub fn get_tags(
        &self,
    ) -> &::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &self.tags
    }

    fn get_tags_for_reflect(
        &self,
    ) -> &::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &self.tags
    }

    fn mut_tags_for_reflect(
        &mut self,
    ) -> &mut ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &mut self.tags
    }
}

impl ::protobuf::Message for MetaDataProto {
    fn is_initialized(&self) -> bool {
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
                    ::protobuf::rt::read_map_into::<
                        ::protobuf::types::ProtobufTypeString,
                        ::protobuf::types::ProtobufTypeString,
                    >(wire_type, is, &mut self.tags)?;
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
        my_size += ::protobuf::rt::compute_map_size::<
            ::protobuf::types::ProtobufTypeString,
            ::protobuf::types::ProtobufTypeString,
        >(1, &self.tags);
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(
        &self,
        os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        ::protobuf::rt::write_map_with_cached_sizes::<
            ::protobuf::types::ProtobufTypeString,
            ::protobuf::types::ProtobufTypeString,
        >(1, &self.tags, os)?;
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

impl ::protobuf::MessageStatic for MetaDataProto {
    fn new() -> MetaDataProto {
        MetaDataProto::new()
    }

    fn descriptor_static(
        _: ::std::option::Option<MetaDataProto>,
    ) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> =
            ::protobuf::lazy::Lazy {
                lock: ::protobuf::lazy::ONCE_INIT,
                ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
            };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(
                    "tags",
                    MetaDataProto::get_tags_for_reflect,
                    MetaDataProto::mut_tags_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MetaDataProto>(
                    "MetaDataProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MetaDataProto {
    fn clear(&mut self) {
        self.clear_tags();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MetaDataProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MetaDataProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x12dss/metadata.proto\x12\x03dss\"z\n\rMetaDataProto\x120\n\x04tags\
    \x18\x01\x20\x03(\x0b2\x1c.dss.MetaDataProto.TagsEntryR\x04tags\x1a7\n\t\
    TagsEntry\x12\x10\n\x03key\x18\x01\x20\x01(\tR\x03key\x12\x14\n\x05value\
    \x18\x02\x20\x01(\tR\x05value:\x028\x01Jz\n\x06\x12\x04\0\0\x06\x01\n\
    \x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x02\x08\x0b\n\n\n\
    \x02\x04\0\x12\x04\x04\0\x06\x01\n\n\n\x03\x04\0\x01\x12\x03\x04\x08\x15\
    \n\x0b\n\x04\x04\0\x02\0\x12\x03\x05\x02\x1f\n\r\n\x05\x04\0\x02\0\x04\
    \x12\x04\x05\x02\x04\x17\n\x0c\n\x05\x04\0\x02\0\x06\x12\x03\x05\x02\x15\
    \n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x05\x16\x1a\n\x0c\n\x05\x04\0\x02\0\
    \x03\x12\x03\x05\x1d\x1eb\x06proto3\
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
