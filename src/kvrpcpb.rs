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
pub struct LockInfo {
    // message fields
    pub primary_lock: ::bytes::Bytes,
    pub lock_version: u64,
    pub key: ::bytes::Bytes,
    pub lock_ttl: u64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LockInfo {}

impl LockInfo {
    pub fn new() -> LockInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LockInfo {
        static mut instance: ::protobuf::lazy::Lazy<LockInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LockInfo,
        };
        unsafe {
            instance.get(LockInfo::new)
        }
    }

    // bytes primary_lock = 1;

    pub fn clear_primary_lock(&mut self) {
        self.primary_lock.clear();
    }

    // Param is passed by value, moved
    pub fn set_primary_lock(&mut self, v: ::bytes::Bytes) {
        self.primary_lock = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_primary_lock(&mut self) -> &mut ::bytes::Bytes {
        &mut self.primary_lock
    }

    // Take field
    pub fn take_primary_lock(&mut self) -> ::bytes::Bytes {
        ::std::mem::replace(&mut self.primary_lock, ::bytes::Bytes::new())
    }

    pub fn get_primary_lock(&self) -> &[u8] {
        &self.primary_lock
    }

    fn get_primary_lock_for_reflect(&self) -> &::bytes::Bytes {
        &self.primary_lock
    }

    fn mut_primary_lock_for_reflect(&mut self) -> &mut ::bytes::Bytes {
        &mut self.primary_lock
    }

    // uint64 lock_version = 2;

    pub fn clear_lock_version(&mut self) {
        self.lock_version = 0;
    }

    // Param is passed by value, moved
    pub fn set_lock_version(&mut self, v: u64) {
        self.lock_version = v;
    }

    pub fn get_lock_version(&self) -> u64 {
        self.lock_version
    }

    fn get_lock_version_for_reflect(&self) -> &u64 {
        &self.lock_version
    }

    fn mut_lock_version_for_reflect(&mut self) -> &mut u64 {
        &mut self.lock_version
    }

    // bytes key = 3;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::bytes::Bytes) {
        self.key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::bytes::Bytes {
        &mut self.key
    }

    // Take field
    pub fn take_key(&mut self) -> ::bytes::Bytes {
        ::std::mem::replace(&mut self.key, ::bytes::Bytes::new())
    }

    pub fn get_key(&self) -> &[u8] {
        &self.key
    }

    fn get_key_for_reflect(&self) -> &::bytes::Bytes {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::bytes::Bytes {
        &mut self.key
    }

    // uint64 lock_ttl = 4;

    pub fn clear_lock_ttl(&mut self) {
        self.lock_ttl = 0;
    }

    // Param is passed by value, moved
    pub fn set_lock_ttl(&mut self, v: u64) {
        self.lock_ttl = v;
    }

    pub fn get_lock_ttl(&self) -> u64 {
        self.lock_ttl
    }

    fn get_lock_ttl_for_reflect(&self) -> &u64 {
        &self.lock_ttl
    }

    fn mut_lock_ttl_for_reflect(&mut self) -> &mut u64 {
        &mut self.lock_ttl
    }
}

impl ::protobuf::Message for LockInfo {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_carllerche_bytes_into(wire_type, is, &mut self.primary_lock)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.lock_version = tmp;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_carllerche_bytes_into(wire_type, is, &mut self.key)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.lock_ttl = tmp;
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
        if !self.primary_lock.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.primary_lock);
        }
        if self.lock_version != 0 {
            my_size += ::protobuf::rt::value_size(2, self.lock_version, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.key);
        }
        if self.lock_ttl != 0 {
            my_size += ::protobuf::rt::value_size(4, self.lock_ttl, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.primary_lock.is_empty() {
            os.write_bytes(1, &self.primary_lock)?;
        }
        if self.lock_version != 0 {
            os.write_uint64(2, self.lock_version)?;
        }
        if !self.key.is_empty() {
            os.write_bytes(3, &self.key)?;
        }
        if self.lock_ttl != 0 {
            os.write_uint64(4, self.lock_ttl)?;
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

impl ::protobuf::MessageStatic for LockInfo {
    fn new() -> LockInfo {
        LockInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<LockInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheBytes>(
                    "primary_lock",
                    LockInfo::get_primary_lock_for_reflect,
                    LockInfo::mut_primary_lock_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "lock_version",
                    LockInfo::get_lock_version_for_reflect,
                    LockInfo::mut_lock_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheBytes>(
                    "key",
                    LockInfo::get_key_for_reflect,
                    LockInfo::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "lock_ttl",
                    LockInfo::get_lock_ttl_for_reflect,
                    LockInfo::mut_lock_ttl_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LockInfo>(
                    "LockInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LockInfo {
    fn clear(&mut self) {
        self.clear_primary_lock();
        self.clear_lock_version();
        self.clear_key();
        self.clear_lock_ttl();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LockInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LockInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct KeyError {
    // message fields
    locked: ::protobuf::SingularPtrField<LockInfo>,
    pub retryable: ::protobuf::chars::Chars,
    pub abort: ::protobuf::chars::Chars,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for KeyError {}

impl KeyError {
    pub fn new() -> KeyError {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static KeyError {
        static mut instance: ::protobuf::lazy::Lazy<KeyError> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const KeyError,
        };
        unsafe {
            instance.get(KeyError::new)
        }
    }

    // .kvrpcpb.LockInfo locked = 1;

    pub fn clear_locked(&mut self) {
        self.locked.clear();
    }

    pub fn has_locked(&self) -> bool {
        self.locked.is_some()
    }

    // Param is passed by value, moved
    pub fn set_locked(&mut self, v: LockInfo) {
        self.locked = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_locked(&mut self) -> &mut LockInfo {
        if self.locked.is_none() {
            self.locked.set_default();
        }
        self.locked.as_mut().unwrap()
    }

    // Take field
    pub fn take_locked(&mut self) -> LockInfo {
        self.locked.take().unwrap_or_else(|| LockInfo::new())
    }

    pub fn get_locked(&self) -> &LockInfo {
        self.locked.as_ref().unwrap_or_else(|| LockInfo::default_instance())
    }

    fn get_locked_for_reflect(&self) -> &::protobuf::SingularPtrField<LockInfo> {
        &self.locked
    }

    fn mut_locked_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<LockInfo> {
        &mut self.locked
    }

    // string retryable = 2;

    pub fn clear_retryable(&mut self) {
        ::protobuf::Clear::clear(&mut self.retryable);
    }

    // Param is passed by value, moved
    pub fn set_retryable(&mut self, v: ::protobuf::chars::Chars) {
        self.retryable = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_retryable(&mut self) -> &mut ::protobuf::chars::Chars {
        &mut self.retryable
    }

    // Take field
    pub fn take_retryable(&mut self) -> ::protobuf::chars::Chars {
        ::std::mem::replace(&mut self.retryable, ::protobuf::chars::Chars::new())
    }

    pub fn get_retryable(&self) -> &str {
        &self.retryable
    }

    fn get_retryable_for_reflect(&self) -> &::protobuf::chars::Chars {
        &self.retryable
    }

    fn mut_retryable_for_reflect(&mut self) -> &mut ::protobuf::chars::Chars {
        &mut self.retryable
    }

    // string abort = 3;

    pub fn clear_abort(&mut self) {
        ::protobuf::Clear::clear(&mut self.abort);
    }

    // Param is passed by value, moved
    pub fn set_abort(&mut self, v: ::protobuf::chars::Chars) {
        self.abort = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_abort(&mut self) -> &mut ::protobuf::chars::Chars {
        &mut self.abort
    }

    // Take field
    pub fn take_abort(&mut self) -> ::protobuf::chars::Chars {
        ::std::mem::replace(&mut self.abort, ::protobuf::chars::Chars::new())
    }

    pub fn get_abort(&self) -> &str {
        &self.abort
    }

    fn get_abort_for_reflect(&self) -> &::protobuf::chars::Chars {
        &self.abort
    }

    fn mut_abort_for_reflect(&mut self) -> &mut ::protobuf::chars::Chars {
        &mut self.abort
    }
}

impl ::protobuf::Message for KeyError {
    fn is_initialized(&self) -> bool {
        for v in &self.locked {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.locked)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_carllerche_string_into(wire_type, is, &mut self.retryable)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_carllerche_string_into(wire_type, is, &mut self.abort)?;
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
        if let Some(ref v) = self.locked.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.retryable.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.retryable);
        }
        if !self.abort.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.abort);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.locked.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.retryable.is_empty() {
            os.write_string(2, &self.retryable)?;
        }
        if !self.abort.is_empty() {
            os.write_string(3, &self.abort)?;
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

impl ::protobuf::MessageStatic for KeyError {
    fn new() -> KeyError {
        KeyError::new()
    }

    fn descriptor_static(_: ::std::option::Option<KeyError>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<LockInfo>>(
                    "locked",
                    KeyError::get_locked_for_reflect,
                    KeyError::mut_locked_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheChars>(
                    "retryable",
                    KeyError::get_retryable_for_reflect,
                    KeyError::mut_retryable_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheChars>(
                    "abort",
                    KeyError::get_abort_for_reflect,
                    KeyError::mut_abort_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<KeyError>(
                    "KeyError",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for KeyError {
    fn clear(&mut self) {
        self.clear_locked();
        self.clear_retryable();
        self.clear_abort();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for KeyError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for KeyError {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Context {
    // message fields
    pub region_id: u64,
    region_epoch: ::protobuf::SingularPtrField<super::metapb::RegionEpoch>,
    peer: ::protobuf::SingularPtrField<super::metapb::Peer>,
    pub read_quorum: bool,
    pub term: u64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Context {}

impl Context {
    pub fn new() -> Context {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Context {
        static mut instance: ::protobuf::lazy::Lazy<Context> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Context,
        };
        unsafe {
            instance.get(Context::new)
        }
    }

    // uint64 region_id = 1;

    pub fn clear_region_id(&mut self) {
        self.region_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_region_id(&mut self, v: u64) {
        self.region_id = v;
    }

    pub fn get_region_id(&self) -> u64 {
        self.region_id
    }

    fn get_region_id_for_reflect(&self) -> &u64 {
        &self.region_id
    }

    fn mut_region_id_for_reflect(&mut self) -> &mut u64 {
        &mut self.region_id
    }

    // .metapb.RegionEpoch region_epoch = 2;

    pub fn clear_region_epoch(&mut self) {
        self.region_epoch.clear();
    }

    pub fn has_region_epoch(&self) -> bool {
        self.region_epoch.is_some()
    }

    // Param is passed by value, moved
    pub fn set_region_epoch(&mut self, v: super::metapb::RegionEpoch) {
        self.region_epoch = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_region_epoch(&mut self) -> &mut super::metapb::RegionEpoch {
        if self.region_epoch.is_none() {
            self.region_epoch.set_default();
        }
        self.region_epoch.as_mut().unwrap()
    }

    // Take field
    pub fn take_region_epoch(&mut self) -> super::metapb::RegionEpoch {
        self.region_epoch.take().unwrap_or_else(|| super::metapb::RegionEpoch::new())
    }

    pub fn get_region_epoch(&self) -> &super::metapb::RegionEpoch {
        self.region_epoch.as_ref().unwrap_or_else(|| super::metapb::RegionEpoch::default_instance())
    }

    fn get_region_epoch_for_reflect(&self) -> &::protobuf::SingularPtrField<super::metapb::RegionEpoch> {
        &self.region_epoch
    }

    fn mut_region_epoch_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::metapb::RegionEpoch> {
        &mut self.region_epoch
    }

    // .metapb.Peer peer = 3;

    pub fn clear_peer(&mut self) {
        self.peer.clear();
    }

    pub fn has_peer(&self) -> bool {
        self.peer.is_some()
    }

    // Param is passed by value, moved
    pub fn set_peer(&mut self, v: super::metapb::Peer) {
        self.peer = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_peer(&mut self) -> &mut super::metapb::Peer {
        if self.peer.is_none() {
            self.peer.set_default();
        }
        self.peer.as_mut().unwrap()
    }

    // Take field
    pub fn take_peer(&mut self) -> super::metapb::Peer {
        self.peer.take().unwrap_or_else(|| super::metapb::Peer::new())
    }

    pub fn get_peer(&self) -> &super::metapb::Peer {
        self.peer.as_ref().unwrap_or_else(|| super::metapb::Peer::default_instance())
    }

    fn get_peer_for_reflect(&self) -> &::protobuf::SingularPtrField<super::metapb::Peer> {
        &self.peer
    }

    fn mut_peer_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::metapb::Peer> {
        &mut self.peer
    }

    // bool read_quorum = 4;

    pub fn clear_read_quorum(&mut self) {
        self.read_quorum = false;
    }

    // Param is passed by value, moved
    pub fn set_read_quorum(&mut self, v: bool) {
        self.read_quorum = v;
    }

    pub fn get_read_quorum(&self) -> bool {
        self.read_quorum
    }

    fn get_read_quorum_for_reflect(&self) -> &bool {
        &self.read_quorum
    }

    fn mut_read_quorum_for_reflect(&mut self) -> &mut bool {
        &mut self.read_quorum
    }

    // uint64 term = 5;

    pub fn clear_term(&mut self) {
        self.term = 0;
    }

    // Param is passed by value, moved
    pub fn set_term(&mut self, v: u64) {
        self.term = v;
    }

    pub fn get_term(&self) -> u64 {
        self.term
    }

    fn get_term_for_reflect(&self) -> &u64 {
        &self.term
    }

    fn mut_term_for_reflect(&mut self) -> &mut u64 {
        &mut self.term
    }
}

impl ::protobuf::Message for Context {
    fn is_initialized(&self) -> bool {
        for v in &self.region_epoch {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.peer {
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
                    let tmp = is.read_uint64()?;
                    self.region_id = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.region_epoch)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.peer)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.read_quorum = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.term = tmp;
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
        if self.region_id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.region_id, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.region_epoch.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.peer.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.read_quorum != false {
            my_size += 2;
        }
        if self.term != 0 {
            my_size += ::protobuf::rt::value_size(5, self.term, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.region_id != 0 {
            os.write_uint64(1, self.region_id)?;
        }
        if let Some(ref v) = self.region_epoch.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.peer.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.read_quorum != false {
            os.write_bool(4, self.read_quorum)?;
        }
        if self.term != 0 {
            os.write_uint64(5, self.term)?;
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

impl ::protobuf::MessageStatic for Context {
    fn new() -> Context {
        Context::new()
    }

    fn descriptor_static(_: ::std::option::Option<Context>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "region_id",
                    Context::get_region_id_for_reflect,
                    Context::mut_region_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::metapb::RegionEpoch>>(
                    "region_epoch",
                    Context::get_region_epoch_for_reflect,
                    Context::mut_region_epoch_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::metapb::Peer>>(
                    "peer",
                    Context::get_peer_for_reflect,
                    Context::mut_peer_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "read_quorum",
                    Context::get_read_quorum_for_reflect,
                    Context::mut_read_quorum_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "term",
                    Context::get_term_for_reflect,
                    Context::mut_term_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Context>(
                    "Context",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Context {
    fn clear(&mut self) {
        self.clear_region_id();
        self.clear_region_epoch();
        self.clear_peer();
        self.clear_read_quorum();
        self.clear_term();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Context {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Context {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetRequest {
    // message fields
    context: ::protobuf::SingularPtrField<Context>,
    pub key: ::bytes::Bytes,
    pub version: u64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetRequest {}

impl GetRequest {
    pub fn new() -> GetRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetRequest {
        static mut instance: ::protobuf::lazy::Lazy<GetRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetRequest,
        };
        unsafe {
            instance.get(GetRequest::new)
        }
    }

    // .kvrpcpb.Context context = 1;

    pub fn clear_context(&mut self) {
        self.context.clear();
    }

    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }

    // Param is passed by value, moved
    pub fn set_context(&mut self, v: Context) {
        self.context = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context.set_default();
        }
        self.context.as_mut().unwrap()
    }

    // Take field
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(|| Context::new())
    }

    pub fn get_context(&self) -> &Context {
        self.context.as_ref().unwrap_or_else(|| Context::default_instance())
    }

    fn get_context_for_reflect(&self) -> &::protobuf::SingularPtrField<Context> {
        &self.context
    }

    fn mut_context_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Context> {
        &mut self.context
    }

    // bytes key = 2;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::bytes::Bytes) {
        self.key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::bytes::Bytes {
        &mut self.key
    }

    // Take field
    pub fn take_key(&mut self) -> ::bytes::Bytes {
        ::std::mem::replace(&mut self.key, ::bytes::Bytes::new())
    }

    pub fn get_key(&self) -> &[u8] {
        &self.key
    }

    fn get_key_for_reflect(&self) -> &::bytes::Bytes {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::bytes::Bytes {
        &mut self.key
    }

    // uint64 version = 3;

    pub fn clear_version(&mut self) {
        self.version = 0;
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: u64) {
        self.version = v;
    }

    pub fn get_version(&self) -> u64 {
        self.version
    }

    fn get_version_for_reflect(&self) -> &u64 {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut u64 {
        &mut self.version
    }
}

impl ::protobuf::Message for GetRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.context {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.context)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_carllerche_bytes_into(wire_type, is, &mut self.key)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.version = tmp;
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
        if let Some(ref v) = self.context.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.key);
        }
        if self.version != 0 {
            my_size += ::protobuf::rt::value_size(3, self.version, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.context.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.key.is_empty() {
            os.write_bytes(2, &self.key)?;
        }
        if self.version != 0 {
            os.write_uint64(3, self.version)?;
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

impl ::protobuf::MessageStatic for GetRequest {
    fn new() -> GetRequest {
        GetRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Context>>(
                    "context",
                    GetRequest::get_context_for_reflect,
                    GetRequest::mut_context_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheBytes>(
                    "key",
                    GetRequest::get_key_for_reflect,
                    GetRequest::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "version",
                    GetRequest::get_version_for_reflect,
                    GetRequest::mut_version_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetRequest>(
                    "GetRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetRequest {
    fn clear(&mut self) {
        self.clear_context();
        self.clear_key();
        self.clear_version();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetResponse {
    // message fields
    region_error: ::protobuf::SingularPtrField<super::errorpb::Error>,
    error: ::protobuf::SingularPtrField<KeyError>,
    pub value: ::bytes::Bytes,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetResponse {}

impl GetResponse {
    pub fn new() -> GetResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetResponse {
        static mut instance: ::protobuf::lazy::Lazy<GetResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetResponse,
        };
        unsafe {
            instance.get(GetResponse::new)
        }
    }

    // .errorpb.Error region_error = 1;

    pub fn clear_region_error(&mut self) {
        self.region_error.clear();
    }

    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error.set_default();
        }
        self.region_error.as_mut().unwrap()
    }

    // Take field
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error.take().unwrap_or_else(|| super::errorpb::Error::new())
    }

    pub fn get_region_error(&self) -> &super::errorpb::Error {
        self.region_error.as_ref().unwrap_or_else(|| super::errorpb::Error::default_instance())
    }

    fn get_region_error_for_reflect(&self) -> &::protobuf::SingularPtrField<super::errorpb::Error> {
        &self.region_error
    }

    fn mut_region_error_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::errorpb::Error> {
        &mut self.region_error
    }

    // .kvrpcpb.KeyError error = 2;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: KeyError) {
        self.error = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut KeyError {
        if self.error.is_none() {
            self.error.set_default();
        }
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> KeyError {
        self.error.take().unwrap_or_else(|| KeyError::new())
    }

    pub fn get_error(&self) -> &KeyError {
        self.error.as_ref().unwrap_or_else(|| KeyError::default_instance())
    }

    fn get_error_for_reflect(&self) -> &::protobuf::SingularPtrField<KeyError> {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<KeyError> {
        &mut self.error
    }

    // bytes value = 3;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::bytes::Bytes) {
        self.value = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::bytes::Bytes {
        &mut self.value
    }

    // Take field
    pub fn take_value(&mut self) -> ::bytes::Bytes {
        ::std::mem::replace(&mut self.value, ::bytes::Bytes::new())
    }

    pub fn get_value(&self) -> &[u8] {
        &self.value
    }

    fn get_value_for_reflect(&self) -> &::bytes::Bytes {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::bytes::Bytes {
        &mut self.value
    }
}

impl ::protobuf::Message for GetResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.region_error {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.error {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.region_error)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.error)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_carllerche_bytes_into(wire_type, is, &mut self.value)?;
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
        if let Some(ref v) = self.region_error.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.error.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.value.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.value);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.region_error.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.error.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.value.is_empty() {
            os.write_bytes(3, &self.value)?;
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

impl ::protobuf::MessageStatic for GetResponse {
    fn new() -> GetResponse {
        GetResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::errorpb::Error>>(
                    "region_error",
                    GetResponse::get_region_error_for_reflect,
                    GetResponse::mut_region_error_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<KeyError>>(
                    "error",
                    GetResponse::get_error_for_reflect,
                    GetResponse::mut_error_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheBytes>(
                    "value",
                    GetResponse::get_value_for_reflect,
                    GetResponse::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetResponse>(
                    "GetResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetResponse {
    fn clear(&mut self) {
        self.clear_region_error();
        self.clear_error();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ScanRequest {
    // message fields
    context: ::protobuf::SingularPtrField<Context>,
    pub start_key: ::bytes::Bytes,
    pub limit: u32,
    pub version: u64,
    pub key_only: bool,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ScanRequest {}

impl ScanRequest {
    pub fn new() -> ScanRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ScanRequest {
        static mut instance: ::protobuf::lazy::Lazy<ScanRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ScanRequest,
        };
        unsafe {
            instance.get(ScanRequest::new)
        }
    }

    // .kvrpcpb.Context context = 1;

    pub fn clear_context(&mut self) {
        self.context.clear();
    }

    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }

    // Param is passed by value, moved
    pub fn set_context(&mut self, v: Context) {
        self.context = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context.set_default();
        }
        self.context.as_mut().unwrap()
    }

    // Take field
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(|| Context::new())
    }

    pub fn get_context(&self) -> &Context {
        self.context.as_ref().unwrap_or_else(|| Context::default_instance())
    }

    fn get_context_for_reflect(&self) -> &::protobuf::SingularPtrField<Context> {
        &self.context
    }

    fn mut_context_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Context> {
        &mut self.context
    }

    // bytes start_key = 2;

    pub fn clear_start_key(&mut self) {
        self.start_key.clear();
    }

    // Param is passed by value, moved
    pub fn set_start_key(&mut self, v: ::bytes::Bytes) {
        self.start_key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_start_key(&mut self) -> &mut ::bytes::Bytes {
        &mut self.start_key
    }

    // Take field
    pub fn take_start_key(&mut self) -> ::bytes::Bytes {
        ::std::mem::replace(&mut self.start_key, ::bytes::Bytes::new())
    }

    pub fn get_start_key(&self) -> &[u8] {
        &self.start_key
    }

    fn get_start_key_for_reflect(&self) -> &::bytes::Bytes {
        &self.start_key
    }

    fn mut_start_key_for_reflect(&mut self) -> &mut ::bytes::Bytes {
        &mut self.start_key
    }

    // uint32 limit = 3;

    pub fn clear_limit(&mut self) {
        self.limit = 0;
    }

    // Param is passed by value, moved
    pub fn set_limit(&mut self, v: u32) {
        self.limit = v;
    }

    pub fn get_limit(&self) -> u32 {
        self.limit
    }

    fn get_limit_for_reflect(&self) -> &u32 {
        &self.limit
    }

    fn mut_limit_for_reflect(&mut self) -> &mut u32 {
        &mut self.limit
    }

    // uint64 version = 4;

    pub fn clear_version(&mut self) {
        self.version = 0;
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: u64) {
        self.version = v;
    }

    pub fn get_version(&self) -> u64 {
        self.version
    }

    fn get_version_for_reflect(&self) -> &u64 {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut u64 {
        &mut self.version
    }

    // bool key_only = 5;

    pub fn clear_key_only(&mut self) {
        self.key_only = false;
    }

    // Param is passed by value, moved
    pub fn set_key_only(&mut self, v: bool) {
        self.key_only = v;
    }

    pub fn get_key_only(&self) -> bool {
        self.key_only
    }

    fn get_key_only_for_reflect(&self) -> &bool {
        &self.key_only
    }

    fn mut_key_only_for_reflect(&mut self) -> &mut bool {
        &mut self.key_only
    }
}

impl ::protobuf::Message for ScanRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.context {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.context)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_carllerche_bytes_into(wire_type, is, &mut self.start_key)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.limit = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.version = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.key_only = tmp;
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
        if let Some(ref v) = self.context.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.start_key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.start_key);
        }
        if self.limit != 0 {
            my_size += ::protobuf::rt::value_size(3, self.limit, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.version != 0 {
            my_size += ::protobuf::rt::value_size(4, self.version, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.key_only != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.context.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.start_key.is_empty() {
            os.write_bytes(2, &self.start_key)?;
        }
        if self.limit != 0 {
            os.write_uint32(3, self.limit)?;
        }
        if self.version != 0 {
            os.write_uint64(4, self.version)?;
        }
        if self.key_only != false {
            os.write_bool(5, self.key_only)?;
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

impl ::protobuf::MessageStatic for ScanRequest {
    fn new() -> ScanRequest {
        ScanRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<ScanRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Context>>(
                    "context",
                    ScanRequest::get_context_for_reflect,
                    ScanRequest::mut_context_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheBytes>(
                    "start_key",
                    ScanRequest::get_start_key_for_reflect,
                    ScanRequest::mut_start_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "limit",
                    ScanRequest::get_limit_for_reflect,
                    ScanRequest::mut_limit_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "version",
                    ScanRequest::get_version_for_reflect,
                    ScanRequest::mut_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "key_only",
                    ScanRequest::get_key_only_for_reflect,
                    ScanRequest::mut_key_only_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ScanRequest>(
                    "ScanRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ScanRequest {
    fn clear(&mut self) {
        self.clear_context();
        self.clear_start_key();
        self.clear_limit();
        self.clear_version();
        self.clear_key_only();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ScanRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ScanRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct KvPair {
    // message fields
    error: ::protobuf::SingularPtrField<KeyError>,
    pub key: ::bytes::Bytes,
    pub value: ::bytes::Bytes,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for KvPair {}

impl KvPair {
    pub fn new() -> KvPair {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static KvPair {
        static mut instance: ::protobuf::lazy::Lazy<KvPair> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const KvPair,
        };
        unsafe {
            instance.get(KvPair::new)
        }
    }

    // .kvrpcpb.KeyError error = 1;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: KeyError) {
        self.error = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut KeyError {
        if self.error.is_none() {
            self.error.set_default();
        }
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> KeyError {
        self.error.take().unwrap_or_else(|| KeyError::new())
    }

    pub fn get_error(&self) -> &KeyError {
        self.error.as_ref().unwrap_or_else(|| KeyError::default_instance())
    }

    fn get_error_for_reflect(&self) -> &::protobuf::SingularPtrField<KeyError> {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<KeyError> {
        &mut self.error
    }

    // bytes key = 2;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::bytes::Bytes) {
        self.key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::bytes::Bytes {
        &mut self.key
    }

    // Take field
    pub fn take_key(&mut self) -> ::bytes::Bytes {
        ::std::mem::replace(&mut self.key, ::bytes::Bytes::new())
    }

    pub fn get_key(&self) -> &[u8] {
        &self.key
    }

    fn get_key_for_reflect(&self) -> &::bytes::Bytes {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::bytes::Bytes {
        &mut self.key
    }

    // bytes value = 3;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::bytes::Bytes) {
        self.value = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::bytes::Bytes {
        &mut self.value
    }

    // Take field
    pub fn take_value(&mut self) -> ::bytes::Bytes {
        ::std::mem::replace(&mut self.value, ::bytes::Bytes::new())
    }

    pub fn get_value(&self) -> &[u8] {
        &self.value
    }

    fn get_value_for_reflect(&self) -> &::bytes::Bytes {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::bytes::Bytes {
        &mut self.value
    }
}

impl ::protobuf::Message for KvPair {
    fn is_initialized(&self) -> bool {
        for v in &self.error {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.error)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_carllerche_bytes_into(wire_type, is, &mut self.key)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_carllerche_bytes_into(wire_type, is, &mut self.value)?;
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
        if let Some(ref v) = self.error.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.key);
        }
        if !self.value.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.value);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.error.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.key.is_empty() {
            os.write_bytes(2, &self.key)?;
        }
        if !self.value.is_empty() {
            os.write_bytes(3, &self.value)?;
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

impl ::protobuf::MessageStatic for KvPair {
    fn new() -> KvPair {
        KvPair::new()
    }

    fn descriptor_static(_: ::std::option::Option<KvPair>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<KeyError>>(
                    "error",
                    KvPair::get_error_for_reflect,
                    KvPair::mut_error_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheBytes>(
                    "key",
                    KvPair::get_key_for_reflect,
                    KvPair::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheBytes>(
                    "value",
                    KvPair::get_value_for_reflect,
                    KvPair::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<KvPair>(
                    "KvPair",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for KvPair {
    fn clear(&mut self) {
        self.clear_error();
        self.clear_key();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for KvPair {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for KvPair {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ScanResponse {
    // message fields
    region_error: ::protobuf::SingularPtrField<super::errorpb::Error>,
    pairs: ::protobuf::RepeatedField<KvPair>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ScanResponse {}

impl ScanResponse {
    pub fn new() -> ScanResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ScanResponse {
        static mut instance: ::protobuf::lazy::Lazy<ScanResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ScanResponse,
        };
        unsafe {
            instance.get(ScanResponse::new)
        }
    }

    // .errorpb.Error region_error = 1;

    pub fn clear_region_error(&mut self) {
        self.region_error.clear();
    }

    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error.set_default();
        }
        self.region_error.as_mut().unwrap()
    }

    // Take field
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error.take().unwrap_or_else(|| super::errorpb::Error::new())
    }

    pub fn get_region_error(&self) -> &super::errorpb::Error {
        self.region_error.as_ref().unwrap_or_else(|| super::errorpb::Error::default_instance())
    }

    fn get_region_error_for_reflect(&self) -> &::protobuf::SingularPtrField<super::errorpb::Error> {
        &self.region_error
    }

    fn mut_region_error_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::errorpb::Error> {
        &mut self.region_error
    }

    // repeated .kvrpcpb.KvPair pairs = 2;

    pub fn clear_pairs(&mut self) {
        self.pairs.clear();
    }

    // Param is passed by value, moved
    pub fn set_pairs(&mut self, v: ::protobuf::RepeatedField<KvPair>) {
        self.pairs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_pairs(&mut self) -> &mut ::protobuf::RepeatedField<KvPair> {
        &mut self.pairs
    }

    // Take field
    pub fn take_pairs(&mut self) -> ::protobuf::RepeatedField<KvPair> {
        ::std::mem::replace(&mut self.pairs, ::protobuf::RepeatedField::new())
    }

    pub fn get_pairs(&self) -> &[KvPair] {
        &self.pairs
    }

    fn get_pairs_for_reflect(&self) -> &::protobuf::RepeatedField<KvPair> {
        &self.pairs
    }

    fn mut_pairs_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<KvPair> {
        &mut self.pairs
    }
}

impl ::protobuf::Message for ScanResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.region_error {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.pairs {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.region_error)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.pairs)?;
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
        if let Some(ref v) = self.region_error.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.pairs {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.region_error.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.pairs {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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

impl ::protobuf::MessageStatic for ScanResponse {
    fn new() -> ScanResponse {
        ScanResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<ScanResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::errorpb::Error>>(
                    "region_error",
                    ScanResponse::get_region_error_for_reflect,
                    ScanResponse::mut_region_error_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<KvPair>>(
                    "pairs",
                    ScanResponse::get_pairs_for_reflect,
                    ScanResponse::mut_pairs_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ScanResponse>(
                    "ScanResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ScanResponse {
    fn clear(&mut self) {
        self.clear_region_error();
        self.clear_pairs();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ScanResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ScanResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Mutation {
    // message fields
    pub op: Op,
    pub key: ::bytes::Bytes,
    pub value: ::bytes::Bytes,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Mutation {}

impl Mutation {
    pub fn new() -> Mutation {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Mutation {
        static mut instance: ::protobuf::lazy::Lazy<Mutation> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Mutation,
        };
        unsafe {
            instance.get(Mutation::new)
        }
    }

    // .kvrpcpb.Op op = 1;

    pub fn clear_op(&mut self) {
        self.op = Op::Put;
    }

    // Param is passed by value, moved
    pub fn set_op(&mut self, v: Op) {
        self.op = v;
    }

    pub fn get_op(&self) -> Op {
        self.op
    }

    fn get_op_for_reflect(&self) -> &Op {
        &self.op
    }

    fn mut_op_for_reflect(&mut self) -> &mut Op {
        &mut self.op
    }

    // bytes key = 2;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::bytes::Bytes) {
        self.key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::bytes::Bytes {
        &mut self.key
    }

    // Take field
    pub fn take_key(&mut self) -> ::bytes::Bytes {
        ::std::mem::replace(&mut self.key, ::bytes::Bytes::new())
    }

    pub fn get_key(&self) -> &[u8] {
        &self.key
    }

    fn get_key_for_reflect(&self) -> &::bytes::Bytes {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::bytes::Bytes {
        &mut self.key
    }

    // bytes value = 3;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::bytes::Bytes) {
        self.value = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::bytes::Bytes {
        &mut self.value
    }

    // Take field
    pub fn take_value(&mut self) -> ::bytes::Bytes {
        ::std::mem::replace(&mut self.value, ::bytes::Bytes::new())
    }

    pub fn get_value(&self) -> &[u8] {
        &self.value
    }

    fn get_value_for_reflect(&self) -> &::bytes::Bytes {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::bytes::Bytes {
        &mut self.value
    }
}

impl ::protobuf::Message for Mutation {
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
                    self.op = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_carllerche_bytes_into(wire_type, is, &mut self.key)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_carllerche_bytes_into(wire_type, is, &mut self.value)?;
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
        if self.op != Op::Put {
            my_size += ::protobuf::rt::enum_size(1, self.op);
        }
        if !self.key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.key);
        }
        if !self.value.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.value);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.op != Op::Put {
            os.write_enum(1, self.op.value())?;
        }
        if !self.key.is_empty() {
            os.write_bytes(2, &self.key)?;
        }
        if !self.value.is_empty() {
            os.write_bytes(3, &self.value)?;
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

impl ::protobuf::MessageStatic for Mutation {
    fn new() -> Mutation {
        Mutation::new()
    }

    fn descriptor_static(_: ::std::option::Option<Mutation>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Op>>(
                    "op",
                    Mutation::get_op_for_reflect,
                    Mutation::mut_op_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheBytes>(
                    "key",
                    Mutation::get_key_for_reflect,
                    Mutation::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheBytes>(
                    "value",
                    Mutation::get_value_for_reflect,
                    Mutation::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Mutation>(
                    "Mutation",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Mutation {
    fn clear(&mut self) {
        self.clear_op();
        self.clear_key();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Mutation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Mutation {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PrewriteRequest {
    // message fields
    context: ::protobuf::SingularPtrField<Context>,
    mutations: ::protobuf::RepeatedField<Mutation>,
    pub primary_lock: ::bytes::Bytes,
    pub start_version: u64,
    pub lock_ttl: u64,
    pub skip_constraint_check: bool,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PrewriteRequest {}

impl PrewriteRequest {
    pub fn new() -> PrewriteRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PrewriteRequest {
        static mut instance: ::protobuf::lazy::Lazy<PrewriteRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PrewriteRequest,
        };
        unsafe {
            instance.get(PrewriteRequest::new)
        }
    }

    // .kvrpcpb.Context context = 1;

    pub fn clear_context(&mut self) {
        self.context.clear();
    }

    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }

    // Param is passed by value, moved
    pub fn set_context(&mut self, v: Context) {
        self.context = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context.set_default();
        }
        self.context.as_mut().unwrap()
    }

    // Take field
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(|| Context::new())
    }

    pub fn get_context(&self) -> &Context {
        self.context.as_ref().unwrap_or_else(|| Context::default_instance())
    }

    fn get_context_for_reflect(&self) -> &::protobuf::SingularPtrField<Context> {
        &self.context
    }

    fn mut_context_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Context> {
        &mut self.context
    }

    // repeated .kvrpcpb.Mutation mutations = 2;

    pub fn clear_mutations(&mut self) {
        self.mutations.clear();
    }

    // Param is passed by value, moved
    pub fn set_mutations(&mut self, v: ::protobuf::RepeatedField<Mutation>) {
        self.mutations = v;
    }

    // Mutable pointer to the field.
    pub fn mut_mutations(&mut self) -> &mut ::protobuf::RepeatedField<Mutation> {
        &mut self.mutations
    }

    // Take field
    pub fn take_mutations(&mut self) -> ::protobuf::RepeatedField<Mutation> {
        ::std::mem::replace(&mut self.mutations, ::protobuf::RepeatedField::new())
    }

    pub fn get_mutations(&self) -> &[Mutation] {
        &self.mutations
    }

    fn get_mutations_for_reflect(&self) -> &::protobuf::RepeatedField<Mutation> {
        &self.mutations
    }

    fn mut_mutations_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Mutation> {
        &mut self.mutations
    }

    // bytes primary_lock = 3;

    pub fn clear_primary_lock(&mut self) {
        self.primary_lock.clear();
    }

    // Param is passed by value, moved
    pub fn set_primary_lock(&mut self, v: ::bytes::Bytes) {
        self.primary_lock = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_primary_lock(&mut self) -> &mut ::bytes::Bytes {
        &mut self.primary_lock
    }

    // Take field
    pub fn take_primary_lock(&mut self) -> ::bytes::Bytes {
        ::std::mem::replace(&mut self.primary_lock, ::bytes::Bytes::new())
    }

    pub fn get_primary_lock(&self) -> &[u8] {
        &self.primary_lock
    }

    fn get_primary_lock_for_reflect(&self) -> &::bytes::Bytes {
        &self.primary_lock
    }

    fn mut_primary_lock_for_reflect(&mut self) -> &mut ::bytes::Bytes {
        &mut self.primary_lock
    }

    // uint64 start_version = 4;

    pub fn clear_start_version(&mut self) {
        self.start_version = 0;
    }

    // Param is passed by value, moved
    pub fn set_start_version(&mut self, v: u64) {
        self.start_version = v;
    }

    pub fn get_start_version(&self) -> u64 {
        self.start_version
    }

    fn get_start_version_for_reflect(&self) -> &u64 {
        &self.start_version
    }

    fn mut_start_version_for_reflect(&mut self) -> &mut u64 {
        &mut self.start_version
    }

    // uint64 lock_ttl = 5;

    pub fn clear_lock_ttl(&mut self) {
        self.lock_ttl = 0;
    }

    // Param is passed by value, moved
    pub fn set_lock_ttl(&mut self, v: u64) {
        self.lock_ttl = v;
    }

    pub fn get_lock_ttl(&self) -> u64 {
        self.lock_ttl
    }

    fn get_lock_ttl_for_reflect(&self) -> &u64 {
        &self.lock_ttl
    }

    fn mut_lock_ttl_for_reflect(&mut self) -> &mut u64 {
        &mut self.lock_ttl
    }

    // bool skip_constraint_check = 6;

    pub fn clear_skip_constraint_check(&mut self) {
        self.skip_constraint_check = false;
    }

    // Param is passed by value, moved
    pub fn set_skip_constraint_check(&mut self, v: bool) {
        self.skip_constraint_check = v;
    }

    pub fn get_skip_constraint_check(&self) -> bool {
        self.skip_constraint_check
    }

    fn get_skip_constraint_check_for_reflect(&self) -> &bool {
        &self.skip_constraint_check
    }

    fn mut_skip_constraint_check_for_reflect(&mut self) -> &mut bool {
        &mut self.skip_constraint_check
    }
}

impl ::protobuf::Message for PrewriteRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.context {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.mutations {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.context)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.mutations)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_carllerche_bytes_into(wire_type, is, &mut self.primary_lock)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.start_version = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.lock_ttl = tmp;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.skip_constraint_check = tmp;
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
        if let Some(ref v) = self.context.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.mutations {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if !self.primary_lock.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.primary_lock);
        }
        if self.start_version != 0 {
            my_size += ::protobuf::rt::value_size(4, self.start_version, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.lock_ttl != 0 {
            my_size += ::protobuf::rt::value_size(5, self.lock_ttl, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.skip_constraint_check != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.context.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.mutations {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if !self.primary_lock.is_empty() {
            os.write_bytes(3, &self.primary_lock)?;
        }
        if self.start_version != 0 {
            os.write_uint64(4, self.start_version)?;
        }
        if self.lock_ttl != 0 {
            os.write_uint64(5, self.lock_ttl)?;
        }
        if self.skip_constraint_check != false {
            os.write_bool(6, self.skip_constraint_check)?;
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

impl ::protobuf::MessageStatic for PrewriteRequest {
    fn new() -> PrewriteRequest {
        PrewriteRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<PrewriteRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Context>>(
                    "context",
                    PrewriteRequest::get_context_for_reflect,
                    PrewriteRequest::mut_context_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Mutation>>(
                    "mutations",
                    PrewriteRequest::get_mutations_for_reflect,
                    PrewriteRequest::mut_mutations_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheBytes>(
                    "primary_lock",
                    PrewriteRequest::get_primary_lock_for_reflect,
                    PrewriteRequest::mut_primary_lock_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "start_version",
                    PrewriteRequest::get_start_version_for_reflect,
                    PrewriteRequest::mut_start_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "lock_ttl",
                    PrewriteRequest::get_lock_ttl_for_reflect,
                    PrewriteRequest::mut_lock_ttl_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "skip_constraint_check",
                    PrewriteRequest::get_skip_constraint_check_for_reflect,
                    PrewriteRequest::mut_skip_constraint_check_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PrewriteRequest>(
                    "PrewriteRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PrewriteRequest {
    fn clear(&mut self) {
        self.clear_context();
        self.clear_mutations();
        self.clear_primary_lock();
        self.clear_start_version();
        self.clear_lock_ttl();
        self.clear_skip_constraint_check();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PrewriteRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PrewriteRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PrewriteResponse {
    // message fields
    region_error: ::protobuf::SingularPtrField<super::errorpb::Error>,
    errors: ::protobuf::RepeatedField<KeyError>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PrewriteResponse {}

impl PrewriteResponse {
    pub fn new() -> PrewriteResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PrewriteResponse {
        static mut instance: ::protobuf::lazy::Lazy<PrewriteResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PrewriteResponse,
        };
        unsafe {
            instance.get(PrewriteResponse::new)
        }
    }

    // .errorpb.Error region_error = 1;

    pub fn clear_region_error(&mut self) {
        self.region_error.clear();
    }

    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error.set_default();
        }
        self.region_error.as_mut().unwrap()
    }

    // Take field
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error.take().unwrap_or_else(|| super::errorpb::Error::new())
    }

    pub fn get_region_error(&self) -> &super::errorpb::Error {
        self.region_error.as_ref().unwrap_or_else(|| super::errorpb::Error::default_instance())
    }

    fn get_region_error_for_reflect(&self) -> &::protobuf::SingularPtrField<super::errorpb::Error> {
        &self.region_error
    }

    fn mut_region_error_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::errorpb::Error> {
        &mut self.region_error
    }

    // repeated .kvrpcpb.KeyError errors = 2;

    pub fn clear_errors(&mut self) {
        self.errors.clear();
    }

    // Param is passed by value, moved
    pub fn set_errors(&mut self, v: ::protobuf::RepeatedField<KeyError>) {
        self.errors = v;
    }

    // Mutable pointer to the field.
    pub fn mut_errors(&mut self) -> &mut ::protobuf::RepeatedField<KeyError> {
        &mut self.errors
    }

    // Take field
    pub fn take_errors(&mut self) -> ::protobuf::RepeatedField<KeyError> {
        ::std::mem::replace(&mut self.errors, ::protobuf::RepeatedField::new())
    }

    pub fn get_errors(&self) -> &[KeyError] {
        &self.errors
    }

    fn get_errors_for_reflect(&self) -> &::protobuf::RepeatedField<KeyError> {
        &self.errors
    }

    fn mut_errors_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<KeyError> {
        &mut self.errors
    }
}

impl ::protobuf::Message for PrewriteResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.region_error {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.errors {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.region_error)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.errors)?;
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
        if let Some(ref v) = self.region_error.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.errors {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.region_error.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.errors {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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

impl ::protobuf::MessageStatic for PrewriteResponse {
    fn new() -> PrewriteResponse {
        PrewriteResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<PrewriteResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::errorpb::Error>>(
                    "region_error",
                    PrewriteResponse::get_region_error_for_reflect,
                    PrewriteResponse::mut_region_error_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<KeyError>>(
                    "errors",
                    PrewriteResponse::get_errors_for_reflect,
                    PrewriteResponse::mut_errors_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PrewriteResponse>(
                    "PrewriteResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PrewriteResponse {
    fn clear(&mut self) {
        self.clear_region_error();
        self.clear_errors();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PrewriteResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PrewriteResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CommitRequest {
    // message fields
    context: ::protobuf::SingularPtrField<Context>,
    pub start_version: u64,
    keys: ::std::vec::Vec<::bytes::Bytes>,
    pub commit_version: u64,
    pub binlog: ::bytes::Bytes,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CommitRequest {}

impl CommitRequest {
    pub fn new() -> CommitRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CommitRequest {
        static mut instance: ::protobuf::lazy::Lazy<CommitRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CommitRequest,
        };
        unsafe {
            instance.get(CommitRequest::new)
        }
    }

    // .kvrpcpb.Context context = 1;

    pub fn clear_context(&mut self) {
        self.context.clear();
    }

    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }

    // Param is passed by value, moved
    pub fn set_context(&mut self, v: Context) {
        self.context = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context.set_default();
        }
        self.context.as_mut().unwrap()
    }

    // Take field
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(|| Context::new())
    }

    pub fn get_context(&self) -> &Context {
        self.context.as_ref().unwrap_or_else(|| Context::default_instance())
    }

    fn get_context_for_reflect(&self) -> &::protobuf::SingularPtrField<Context> {
        &self.context
    }

    fn mut_context_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Context> {
        &mut self.context
    }

    // uint64 start_version = 2;

    pub fn clear_start_version(&mut self) {
        self.start_version = 0;
    }

    // Param is passed by value, moved
    pub fn set_start_version(&mut self, v: u64) {
        self.start_version = v;
    }

    pub fn get_start_version(&self) -> u64 {
        self.start_version
    }

    fn get_start_version_for_reflect(&self) -> &u64 {
        &self.start_version
    }

    fn mut_start_version_for_reflect(&mut self) -> &mut u64 {
        &mut self.start_version
    }

    // repeated bytes keys = 3;

    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }

    // Param is passed by value, moved
    pub fn set_keys(&mut self, v: ::std::vec::Vec<::bytes::Bytes>) {
        self.keys = v;
    }

    // Mutable pointer to the field.
    pub fn mut_keys(&mut self) -> &mut ::std::vec::Vec<::bytes::Bytes> {
        &mut self.keys
    }

    // Take field
    pub fn take_keys(&mut self) -> ::std::vec::Vec<::bytes::Bytes> {
        ::std::mem::replace(&mut self.keys, ::std::vec::Vec::new())
    }

    pub fn get_keys(&self) -> &[::bytes::Bytes] {
        &self.keys
    }

    fn get_keys_for_reflect(&self) -> &::std::vec::Vec<::bytes::Bytes> {
        &self.keys
    }

    fn mut_keys_for_reflect(&mut self) -> &mut ::std::vec::Vec<::bytes::Bytes> {
        &mut self.keys
    }

    // uint64 commit_version = 4;

    pub fn clear_commit_version(&mut self) {
        self.commit_version = 0;
    }

    // Param is passed by value, moved
    pub fn set_commit_version(&mut self, v: u64) {
        self.commit_version = v;
    }

    pub fn get_commit_version(&self) -> u64 {
        self.commit_version
    }

    fn get_commit_version_for_reflect(&self) -> &u64 {
        &self.commit_version
    }

    fn mut_commit_version_for_reflect(&mut self) -> &mut u64 {
        &mut self.commit_version
    }

    // bytes binlog = 5;

    pub fn clear_binlog(&mut self) {
        self.binlog.clear();
    }

    // Param is passed by value, moved
    pub fn set_binlog(&mut self, v: ::bytes::Bytes) {
        self.binlog = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_binlog(&mut self) -> &mut ::bytes::Bytes {
        &mut self.binlog
    }

    // Take field
    pub fn take_binlog(&mut self) -> ::bytes::Bytes {
        ::std::mem::replace(&mut self.binlog, ::bytes::Bytes::new())
    }

    pub fn get_binlog(&self) -> &[u8] {
        &self.binlog
    }

    fn get_binlog_for_reflect(&self) -> &::bytes::Bytes {
        &self.binlog
    }

    fn mut_binlog_for_reflect(&mut self) -> &mut ::bytes::Bytes {
        &mut self.binlog
    }
}

impl ::protobuf::Message for CommitRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.context {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.context)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.start_version = tmp;
                },
                3 => {
                    ::protobuf::rt::read_repeated_carllerche_bytes_into(wire_type, is, &mut self.keys)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.commit_version = tmp;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_carllerche_bytes_into(wire_type, is, &mut self.binlog)?;
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
        if let Some(ref v) = self.context.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.start_version != 0 {
            my_size += ::protobuf::rt::value_size(2, self.start_version, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.keys {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        };
        if self.commit_version != 0 {
            my_size += ::protobuf::rt::value_size(4, self.commit_version, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.binlog.is_empty() {
            my_size += ::protobuf::rt::bytes_size(5, &self.binlog);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.context.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.start_version != 0 {
            os.write_uint64(2, self.start_version)?;
        }
        for v in &self.keys {
            os.write_bytes(3, &v)?;
        };
        if self.commit_version != 0 {
            os.write_uint64(4, self.commit_version)?;
        }
        if !self.binlog.is_empty() {
            os.write_bytes(5, &self.binlog)?;
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

impl ::protobuf::MessageStatic for CommitRequest {
    fn new() -> CommitRequest {
        CommitRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CommitRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Context>>(
                    "context",
                    CommitRequest::get_context_for_reflect,
                    CommitRequest::mut_context_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "start_version",
                    CommitRequest::get_start_version_for_reflect,
                    CommitRequest::mut_start_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheBytes>(
                    "keys",
                    CommitRequest::get_keys_for_reflect,
                    CommitRequest::mut_keys_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "commit_version",
                    CommitRequest::get_commit_version_for_reflect,
                    CommitRequest::mut_commit_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheBytes>(
                    "binlog",
                    CommitRequest::get_binlog_for_reflect,
                    CommitRequest::mut_binlog_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CommitRequest>(
                    "CommitRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CommitRequest {
    fn clear(&mut self) {
        self.clear_context();
        self.clear_start_version();
        self.clear_keys();
        self.clear_commit_version();
        self.clear_binlog();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CommitRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CommitRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CommitResponse {
    // message fields
    region_error: ::protobuf::SingularPtrField<super::errorpb::Error>,
    error: ::protobuf::SingularPtrField<KeyError>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CommitResponse {}

impl CommitResponse {
    pub fn new() -> CommitResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CommitResponse {
        static mut instance: ::protobuf::lazy::Lazy<CommitResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CommitResponse,
        };
        unsafe {
            instance.get(CommitResponse::new)
        }
    }

    // .errorpb.Error region_error = 1;

    pub fn clear_region_error(&mut self) {
        self.region_error.clear();
    }

    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error.set_default();
        }
        self.region_error.as_mut().unwrap()
    }

    // Take field
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error.take().unwrap_or_else(|| super::errorpb::Error::new())
    }

    pub fn get_region_error(&self) -> &super::errorpb::Error {
        self.region_error.as_ref().unwrap_or_else(|| super::errorpb::Error::default_instance())
    }

    fn get_region_error_for_reflect(&self) -> &::protobuf::SingularPtrField<super::errorpb::Error> {
        &self.region_error
    }

    fn mut_region_error_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::errorpb::Error> {
        &mut self.region_error
    }

    // .kvrpcpb.KeyError error = 2;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: KeyError) {
        self.error = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut KeyError {
        if self.error.is_none() {
            self.error.set_default();
        }
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> KeyError {
        self.error.take().unwrap_or_else(|| KeyError::new())
    }

    pub fn get_error(&self) -> &KeyError {
        self.error.as_ref().unwrap_or_else(|| KeyError::default_instance())
    }

    fn get_error_for_reflect(&self) -> &::protobuf::SingularPtrField<KeyError> {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<KeyError> {
        &mut self.error
    }
}

impl ::protobuf::Message for CommitResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.region_error {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.error {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.region_error)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.error)?;
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
        if let Some(ref v) = self.region_error.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.error.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.region_error.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.error.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for CommitResponse {
    fn new() -> CommitResponse {
        CommitResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CommitResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::errorpb::Error>>(
                    "region_error",
                    CommitResponse::get_region_error_for_reflect,
                    CommitResponse::mut_region_error_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<KeyError>>(
                    "error",
                    CommitResponse::get_error_for_reflect,
                    CommitResponse::mut_error_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CommitResponse>(
                    "CommitResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CommitResponse {
    fn clear(&mut self) {
        self.clear_region_error();
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CommitResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CommitResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ImportRequest {
    // message fields
    mutations: ::protobuf::RepeatedField<Mutation>,
    pub commit_version: u64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ImportRequest {}

impl ImportRequest {
    pub fn new() -> ImportRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ImportRequest {
        static mut instance: ::protobuf::lazy::Lazy<ImportRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ImportRequest,
        };
        unsafe {
            instance.get(ImportRequest::new)
        }
    }

    // repeated .kvrpcpb.Mutation mutations = 1;

    pub fn clear_mutations(&mut self) {
        self.mutations.clear();
    }

    // Param is passed by value, moved
    pub fn set_mutations(&mut self, v: ::protobuf::RepeatedField<Mutation>) {
        self.mutations = v;
    }

    // Mutable pointer to the field.
    pub fn mut_mutations(&mut self) -> &mut ::protobuf::RepeatedField<Mutation> {
        &mut self.mutations
    }

    // Take field
    pub fn take_mutations(&mut self) -> ::protobuf::RepeatedField<Mutation> {
        ::std::mem::replace(&mut self.mutations, ::protobuf::RepeatedField::new())
    }

    pub fn get_mutations(&self) -> &[Mutation] {
        &self.mutations
    }

    fn get_mutations_for_reflect(&self) -> &::protobuf::RepeatedField<Mutation> {
        &self.mutations
    }

    fn mut_mutations_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Mutation> {
        &mut self.mutations
    }

    // uint64 commit_version = 2;

    pub fn clear_commit_version(&mut self) {
        self.commit_version = 0;
    }

    // Param is passed by value, moved
    pub fn set_commit_version(&mut self, v: u64) {
        self.commit_version = v;
    }

    pub fn get_commit_version(&self) -> u64 {
        self.commit_version
    }

    fn get_commit_version_for_reflect(&self) -> &u64 {
        &self.commit_version
    }

    fn mut_commit_version_for_reflect(&mut self) -> &mut u64 {
        &mut self.commit_version
    }
}

impl ::protobuf::Message for ImportRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.mutations {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.mutations)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.commit_version = tmp;
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
        for value in &self.mutations {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.commit_version != 0 {
            my_size += ::protobuf::rt::value_size(2, self.commit_version, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.mutations {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if self.commit_version != 0 {
            os.write_uint64(2, self.commit_version)?;
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

impl ::protobuf::MessageStatic for ImportRequest {
    fn new() -> ImportRequest {
        ImportRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<ImportRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Mutation>>(
                    "mutations",
                    ImportRequest::get_mutations_for_reflect,
                    ImportRequest::mut_mutations_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "commit_version",
                    ImportRequest::get_commit_version_for_reflect,
                    ImportRequest::mut_commit_version_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ImportRequest>(
                    "ImportRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ImportRequest {
    fn clear(&mut self) {
        self.clear_mutations();
        self.clear_commit_version();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ImportRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ImportRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ImportResponse {
    // message fields
    region_error: ::protobuf::SingularPtrField<super::errorpb::Error>,
    pub error: ::protobuf::chars::Chars,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ImportResponse {}

impl ImportResponse {
    pub fn new() -> ImportResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ImportResponse {
        static mut instance: ::protobuf::lazy::Lazy<ImportResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ImportResponse,
        };
        unsafe {
            instance.get(ImportResponse::new)
        }
    }

    // .errorpb.Error region_error = 1;

    pub fn clear_region_error(&mut self) {
        self.region_error.clear();
    }

    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error.set_default();
        }
        self.region_error.as_mut().unwrap()
    }

    // Take field
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error.take().unwrap_or_else(|| super::errorpb::Error::new())
    }

    pub fn get_region_error(&self) -> &super::errorpb::Error {
        self.region_error.as_ref().unwrap_or_else(|| super::errorpb::Error::default_instance())
    }

    fn get_region_error_for_reflect(&self) -> &::protobuf::SingularPtrField<super::errorpb::Error> {
        &self.region_error
    }

    fn mut_region_error_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::errorpb::Error> {
        &mut self.region_error
    }

    // string error = 2;

    pub fn clear_error(&mut self) {
        ::protobuf::Clear::clear(&mut self.error);
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ::protobuf::chars::Chars) {
        self.error = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut ::protobuf::chars::Chars {
        &mut self.error
    }

    // Take field
    pub fn take_error(&mut self) -> ::protobuf::chars::Chars {
        ::std::mem::replace(&mut self.error, ::protobuf::chars::Chars::new())
    }

    pub fn get_error(&self) -> &str {
        &self.error
    }

    fn get_error_for_reflect(&self) -> &::protobuf::chars::Chars {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::protobuf::chars::Chars {
        &mut self.error
    }
}

impl ::protobuf::Message for ImportResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.region_error {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.region_error)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_carllerche_string_into(wire_type, is, &mut self.error)?;
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
        if let Some(ref v) = self.region_error.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.error.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.error);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.region_error.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.error.is_empty() {
            os.write_string(2, &self.error)?;
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

impl ::protobuf::MessageStatic for ImportResponse {
    fn new() -> ImportResponse {
        ImportResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<ImportResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::errorpb::Error>>(
                    "region_error",
                    ImportResponse::get_region_error_for_reflect,
                    ImportResponse::mut_region_error_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheChars>(
                    "error",
                    ImportResponse::get_error_for_reflect,
                    ImportResponse::mut_error_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ImportResponse>(
                    "ImportResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ImportResponse {
    fn clear(&mut self) {
        self.clear_region_error();
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ImportResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ImportResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BatchRollbackRequest {
    // message fields
    context: ::protobuf::SingularPtrField<Context>,
    pub start_version: u64,
    keys: ::std::vec::Vec<::bytes::Bytes>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BatchRollbackRequest {}

impl BatchRollbackRequest {
    pub fn new() -> BatchRollbackRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BatchRollbackRequest {
        static mut instance: ::protobuf::lazy::Lazy<BatchRollbackRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BatchRollbackRequest,
        };
        unsafe {
            instance.get(BatchRollbackRequest::new)
        }
    }

    // .kvrpcpb.Context context = 1;

    pub fn clear_context(&mut self) {
        self.context.clear();
    }

    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }

    // Param is passed by value, moved
    pub fn set_context(&mut self, v: Context) {
        self.context = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context.set_default();
        }
        self.context.as_mut().unwrap()
    }

    // Take field
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(|| Context::new())
    }

    pub fn get_context(&self) -> &Context {
        self.context.as_ref().unwrap_or_else(|| Context::default_instance())
    }

    fn get_context_for_reflect(&self) -> &::protobuf::SingularPtrField<Context> {
        &self.context
    }

    fn mut_context_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Context> {
        &mut self.context
    }

    // uint64 start_version = 2;

    pub fn clear_start_version(&mut self) {
        self.start_version = 0;
    }

    // Param is passed by value, moved
    pub fn set_start_version(&mut self, v: u64) {
        self.start_version = v;
    }

    pub fn get_start_version(&self) -> u64 {
        self.start_version
    }

    fn get_start_version_for_reflect(&self) -> &u64 {
        &self.start_version
    }

    fn mut_start_version_for_reflect(&mut self) -> &mut u64 {
        &mut self.start_version
    }

    // repeated bytes keys = 3;

    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }

    // Param is passed by value, moved
    pub fn set_keys(&mut self, v: ::std::vec::Vec<::bytes::Bytes>) {
        self.keys = v;
    }

    // Mutable pointer to the field.
    pub fn mut_keys(&mut self) -> &mut ::std::vec::Vec<::bytes::Bytes> {
        &mut self.keys
    }

    // Take field
    pub fn take_keys(&mut self) -> ::std::vec::Vec<::bytes::Bytes> {
        ::std::mem::replace(&mut self.keys, ::std::vec::Vec::new())
    }

    pub fn get_keys(&self) -> &[::bytes::Bytes] {
        &self.keys
    }

    fn get_keys_for_reflect(&self) -> &::std::vec::Vec<::bytes::Bytes> {
        &self.keys
    }

    fn mut_keys_for_reflect(&mut self) -> &mut ::std::vec::Vec<::bytes::Bytes> {
        &mut self.keys
    }
}

impl ::protobuf::Message for BatchRollbackRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.context {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.context)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.start_version = tmp;
                },
                3 => {
                    ::protobuf::rt::read_repeated_carllerche_bytes_into(wire_type, is, &mut self.keys)?;
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
        if let Some(ref v) = self.context.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.start_version != 0 {
            my_size += ::protobuf::rt::value_size(2, self.start_version, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.keys {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.context.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.start_version != 0 {
            os.write_uint64(2, self.start_version)?;
        }
        for v in &self.keys {
            os.write_bytes(3, &v)?;
        };
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

impl ::protobuf::MessageStatic for BatchRollbackRequest {
    fn new() -> BatchRollbackRequest {
        BatchRollbackRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<BatchRollbackRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Context>>(
                    "context",
                    BatchRollbackRequest::get_context_for_reflect,
                    BatchRollbackRequest::mut_context_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "start_version",
                    BatchRollbackRequest::get_start_version_for_reflect,
                    BatchRollbackRequest::mut_start_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheBytes>(
                    "keys",
                    BatchRollbackRequest::get_keys_for_reflect,
                    BatchRollbackRequest::mut_keys_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BatchRollbackRequest>(
                    "BatchRollbackRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BatchRollbackRequest {
    fn clear(&mut self) {
        self.clear_context();
        self.clear_start_version();
        self.clear_keys();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BatchRollbackRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BatchRollbackRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BatchRollbackResponse {
    // message fields
    region_error: ::protobuf::SingularPtrField<super::errorpb::Error>,
    error: ::protobuf::SingularPtrField<KeyError>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BatchRollbackResponse {}

impl BatchRollbackResponse {
    pub fn new() -> BatchRollbackResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BatchRollbackResponse {
        static mut instance: ::protobuf::lazy::Lazy<BatchRollbackResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BatchRollbackResponse,
        };
        unsafe {
            instance.get(BatchRollbackResponse::new)
        }
    }

    // .errorpb.Error region_error = 1;

    pub fn clear_region_error(&mut self) {
        self.region_error.clear();
    }

    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error.set_default();
        }
        self.region_error.as_mut().unwrap()
    }

    // Take field
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error.take().unwrap_or_else(|| super::errorpb::Error::new())
    }

    pub fn get_region_error(&self) -> &super::errorpb::Error {
        self.region_error.as_ref().unwrap_or_else(|| super::errorpb::Error::default_instance())
    }

    fn get_region_error_for_reflect(&self) -> &::protobuf::SingularPtrField<super::errorpb::Error> {
        &self.region_error
    }

    fn mut_region_error_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::errorpb::Error> {
        &mut self.region_error
    }

    // .kvrpcpb.KeyError error = 2;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: KeyError) {
        self.error = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut KeyError {
        if self.error.is_none() {
            self.error.set_default();
        }
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> KeyError {
        self.error.take().unwrap_or_else(|| KeyError::new())
    }

    pub fn get_error(&self) -> &KeyError {
        self.error.as_ref().unwrap_or_else(|| KeyError::default_instance())
    }

    fn get_error_for_reflect(&self) -> &::protobuf::SingularPtrField<KeyError> {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<KeyError> {
        &mut self.error
    }
}

impl ::protobuf::Message for BatchRollbackResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.region_error {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.error {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.region_error)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.error)?;
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
        if let Some(ref v) = self.region_error.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.error.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.region_error.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.error.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for BatchRollbackResponse {
    fn new() -> BatchRollbackResponse {
        BatchRollbackResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<BatchRollbackResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::errorpb::Error>>(
                    "region_error",
                    BatchRollbackResponse::get_region_error_for_reflect,
                    BatchRollbackResponse::mut_region_error_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<KeyError>>(
                    "error",
                    BatchRollbackResponse::get_error_for_reflect,
                    BatchRollbackResponse::mut_error_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BatchRollbackResponse>(
                    "BatchRollbackResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BatchRollbackResponse {
    fn clear(&mut self) {
        self.clear_region_error();
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BatchRollbackResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BatchRollbackResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CleanupRequest {
    // message fields
    context: ::protobuf::SingularPtrField<Context>,
    pub key: ::bytes::Bytes,
    pub start_version: u64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CleanupRequest {}

impl CleanupRequest {
    pub fn new() -> CleanupRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CleanupRequest {
        static mut instance: ::protobuf::lazy::Lazy<CleanupRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CleanupRequest,
        };
        unsafe {
            instance.get(CleanupRequest::new)
        }
    }

    // .kvrpcpb.Context context = 1;

    pub fn clear_context(&mut self) {
        self.context.clear();
    }

    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }

    // Param is passed by value, moved
    pub fn set_context(&mut self, v: Context) {
        self.context = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context.set_default();
        }
        self.context.as_mut().unwrap()
    }

    // Take field
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(|| Context::new())
    }

    pub fn get_context(&self) -> &Context {
        self.context.as_ref().unwrap_or_else(|| Context::default_instance())
    }

    fn get_context_for_reflect(&self) -> &::protobuf::SingularPtrField<Context> {
        &self.context
    }

    fn mut_context_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Context> {
        &mut self.context
    }

    // bytes key = 2;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::bytes::Bytes) {
        self.key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::bytes::Bytes {
        &mut self.key
    }

    // Take field
    pub fn take_key(&mut self) -> ::bytes::Bytes {
        ::std::mem::replace(&mut self.key, ::bytes::Bytes::new())
    }

    pub fn get_key(&self) -> &[u8] {
        &self.key
    }

    fn get_key_for_reflect(&self) -> &::bytes::Bytes {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::bytes::Bytes {
        &mut self.key
    }

    // uint64 start_version = 3;

    pub fn clear_start_version(&mut self) {
        self.start_version = 0;
    }

    // Param is passed by value, moved
    pub fn set_start_version(&mut self, v: u64) {
        self.start_version = v;
    }

    pub fn get_start_version(&self) -> u64 {
        self.start_version
    }

    fn get_start_version_for_reflect(&self) -> &u64 {
        &self.start_version
    }

    fn mut_start_version_for_reflect(&mut self) -> &mut u64 {
        &mut self.start_version
    }
}

impl ::protobuf::Message for CleanupRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.context {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.context)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_carllerche_bytes_into(wire_type, is, &mut self.key)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.start_version = tmp;
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
        if let Some(ref v) = self.context.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.key);
        }
        if self.start_version != 0 {
            my_size += ::protobuf::rt::value_size(3, self.start_version, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.context.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.key.is_empty() {
            os.write_bytes(2, &self.key)?;
        }
        if self.start_version != 0 {
            os.write_uint64(3, self.start_version)?;
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

impl ::protobuf::MessageStatic for CleanupRequest {
    fn new() -> CleanupRequest {
        CleanupRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CleanupRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Context>>(
                    "context",
                    CleanupRequest::get_context_for_reflect,
                    CleanupRequest::mut_context_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheBytes>(
                    "key",
                    CleanupRequest::get_key_for_reflect,
                    CleanupRequest::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "start_version",
                    CleanupRequest::get_start_version_for_reflect,
                    CleanupRequest::mut_start_version_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CleanupRequest>(
                    "CleanupRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CleanupRequest {
    fn clear(&mut self) {
        self.clear_context();
        self.clear_key();
        self.clear_start_version();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CleanupRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CleanupRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CleanupResponse {
    // message fields
    region_error: ::protobuf::SingularPtrField<super::errorpb::Error>,
    error: ::protobuf::SingularPtrField<KeyError>,
    pub commit_version: u64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CleanupResponse {}

impl CleanupResponse {
    pub fn new() -> CleanupResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CleanupResponse {
        static mut instance: ::protobuf::lazy::Lazy<CleanupResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CleanupResponse,
        };
        unsafe {
            instance.get(CleanupResponse::new)
        }
    }

    // .errorpb.Error region_error = 1;

    pub fn clear_region_error(&mut self) {
        self.region_error.clear();
    }

    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error.set_default();
        }
        self.region_error.as_mut().unwrap()
    }

    // Take field
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error.take().unwrap_or_else(|| super::errorpb::Error::new())
    }

    pub fn get_region_error(&self) -> &super::errorpb::Error {
        self.region_error.as_ref().unwrap_or_else(|| super::errorpb::Error::default_instance())
    }

    fn get_region_error_for_reflect(&self) -> &::protobuf::SingularPtrField<super::errorpb::Error> {
        &self.region_error
    }

    fn mut_region_error_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::errorpb::Error> {
        &mut self.region_error
    }

    // .kvrpcpb.KeyError error = 2;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: KeyError) {
        self.error = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut KeyError {
        if self.error.is_none() {
            self.error.set_default();
        }
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> KeyError {
        self.error.take().unwrap_or_else(|| KeyError::new())
    }

    pub fn get_error(&self) -> &KeyError {
        self.error.as_ref().unwrap_or_else(|| KeyError::default_instance())
    }

    fn get_error_for_reflect(&self) -> &::protobuf::SingularPtrField<KeyError> {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<KeyError> {
        &mut self.error
    }

    // uint64 commit_version = 3;

    pub fn clear_commit_version(&mut self) {
        self.commit_version = 0;
    }

    // Param is passed by value, moved
    pub fn set_commit_version(&mut self, v: u64) {
        self.commit_version = v;
    }

    pub fn get_commit_version(&self) -> u64 {
        self.commit_version
    }

    fn get_commit_version_for_reflect(&self) -> &u64 {
        &self.commit_version
    }

    fn mut_commit_version_for_reflect(&mut self) -> &mut u64 {
        &mut self.commit_version
    }
}

impl ::protobuf::Message for CleanupResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.region_error {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.error {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.region_error)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.error)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.commit_version = tmp;
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
        if let Some(ref v) = self.region_error.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.error.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.commit_version != 0 {
            my_size += ::protobuf::rt::value_size(3, self.commit_version, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.region_error.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.error.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.commit_version != 0 {
            os.write_uint64(3, self.commit_version)?;
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

impl ::protobuf::MessageStatic for CleanupResponse {
    fn new() -> CleanupResponse {
        CleanupResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CleanupResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::errorpb::Error>>(
                    "region_error",
                    CleanupResponse::get_region_error_for_reflect,
                    CleanupResponse::mut_region_error_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<KeyError>>(
                    "error",
                    CleanupResponse::get_error_for_reflect,
                    CleanupResponse::mut_error_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "commit_version",
                    CleanupResponse::get_commit_version_for_reflect,
                    CleanupResponse::mut_commit_version_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CleanupResponse>(
                    "CleanupResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CleanupResponse {
    fn clear(&mut self) {
        self.clear_region_error();
        self.clear_error();
        self.clear_commit_version();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CleanupResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CleanupResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BatchGetRequest {
    // message fields
    context: ::protobuf::SingularPtrField<Context>,
    keys: ::std::vec::Vec<::bytes::Bytes>,
    pub version: u64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BatchGetRequest {}

impl BatchGetRequest {
    pub fn new() -> BatchGetRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BatchGetRequest {
        static mut instance: ::protobuf::lazy::Lazy<BatchGetRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BatchGetRequest,
        };
        unsafe {
            instance.get(BatchGetRequest::new)
        }
    }

    // .kvrpcpb.Context context = 1;

    pub fn clear_context(&mut self) {
        self.context.clear();
    }

    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }

    // Param is passed by value, moved
    pub fn set_context(&mut self, v: Context) {
        self.context = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context.set_default();
        }
        self.context.as_mut().unwrap()
    }

    // Take field
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(|| Context::new())
    }

    pub fn get_context(&self) -> &Context {
        self.context.as_ref().unwrap_or_else(|| Context::default_instance())
    }

    fn get_context_for_reflect(&self) -> &::protobuf::SingularPtrField<Context> {
        &self.context
    }

    fn mut_context_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Context> {
        &mut self.context
    }

    // repeated bytes keys = 2;

    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }

    // Param is passed by value, moved
    pub fn set_keys(&mut self, v: ::std::vec::Vec<::bytes::Bytes>) {
        self.keys = v;
    }

    // Mutable pointer to the field.
    pub fn mut_keys(&mut self) -> &mut ::std::vec::Vec<::bytes::Bytes> {
        &mut self.keys
    }

    // Take field
    pub fn take_keys(&mut self) -> ::std::vec::Vec<::bytes::Bytes> {
        ::std::mem::replace(&mut self.keys, ::std::vec::Vec::new())
    }

    pub fn get_keys(&self) -> &[::bytes::Bytes] {
        &self.keys
    }

    fn get_keys_for_reflect(&self) -> &::std::vec::Vec<::bytes::Bytes> {
        &self.keys
    }

    fn mut_keys_for_reflect(&mut self) -> &mut ::std::vec::Vec<::bytes::Bytes> {
        &mut self.keys
    }

    // uint64 version = 3;

    pub fn clear_version(&mut self) {
        self.version = 0;
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: u64) {
        self.version = v;
    }

    pub fn get_version(&self) -> u64 {
        self.version
    }

    fn get_version_for_reflect(&self) -> &u64 {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut u64 {
        &mut self.version
    }
}

impl ::protobuf::Message for BatchGetRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.context {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.context)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_carllerche_bytes_into(wire_type, is, &mut self.keys)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.version = tmp;
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
        if let Some(ref v) = self.context.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.keys {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        if self.version != 0 {
            my_size += ::protobuf::rt::value_size(3, self.version, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.context.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.keys {
            os.write_bytes(2, &v)?;
        };
        if self.version != 0 {
            os.write_uint64(3, self.version)?;
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

impl ::protobuf::MessageStatic for BatchGetRequest {
    fn new() -> BatchGetRequest {
        BatchGetRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<BatchGetRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Context>>(
                    "context",
                    BatchGetRequest::get_context_for_reflect,
                    BatchGetRequest::mut_context_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheBytes>(
                    "keys",
                    BatchGetRequest::get_keys_for_reflect,
                    BatchGetRequest::mut_keys_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "version",
                    BatchGetRequest::get_version_for_reflect,
                    BatchGetRequest::mut_version_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BatchGetRequest>(
                    "BatchGetRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BatchGetRequest {
    fn clear(&mut self) {
        self.clear_context();
        self.clear_keys();
        self.clear_version();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BatchGetRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BatchGetRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BatchGetResponse {
    // message fields
    region_error: ::protobuf::SingularPtrField<super::errorpb::Error>,
    pairs: ::protobuf::RepeatedField<KvPair>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BatchGetResponse {}

impl BatchGetResponse {
    pub fn new() -> BatchGetResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BatchGetResponse {
        static mut instance: ::protobuf::lazy::Lazy<BatchGetResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BatchGetResponse,
        };
        unsafe {
            instance.get(BatchGetResponse::new)
        }
    }

    // .errorpb.Error region_error = 1;

    pub fn clear_region_error(&mut self) {
        self.region_error.clear();
    }

    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error.set_default();
        }
        self.region_error.as_mut().unwrap()
    }

    // Take field
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error.take().unwrap_or_else(|| super::errorpb::Error::new())
    }

    pub fn get_region_error(&self) -> &super::errorpb::Error {
        self.region_error.as_ref().unwrap_or_else(|| super::errorpb::Error::default_instance())
    }

    fn get_region_error_for_reflect(&self) -> &::protobuf::SingularPtrField<super::errorpb::Error> {
        &self.region_error
    }

    fn mut_region_error_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::errorpb::Error> {
        &mut self.region_error
    }

    // repeated .kvrpcpb.KvPair pairs = 2;

    pub fn clear_pairs(&mut self) {
        self.pairs.clear();
    }

    // Param is passed by value, moved
    pub fn set_pairs(&mut self, v: ::protobuf::RepeatedField<KvPair>) {
        self.pairs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_pairs(&mut self) -> &mut ::protobuf::RepeatedField<KvPair> {
        &mut self.pairs
    }

    // Take field
    pub fn take_pairs(&mut self) -> ::protobuf::RepeatedField<KvPair> {
        ::std::mem::replace(&mut self.pairs, ::protobuf::RepeatedField::new())
    }

    pub fn get_pairs(&self) -> &[KvPair] {
        &self.pairs
    }

    fn get_pairs_for_reflect(&self) -> &::protobuf::RepeatedField<KvPair> {
        &self.pairs
    }

    fn mut_pairs_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<KvPair> {
        &mut self.pairs
    }
}

impl ::protobuf::Message for BatchGetResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.region_error {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.pairs {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.region_error)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.pairs)?;
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
        if let Some(ref v) = self.region_error.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.pairs {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.region_error.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.pairs {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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

impl ::protobuf::MessageStatic for BatchGetResponse {
    fn new() -> BatchGetResponse {
        BatchGetResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<BatchGetResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::errorpb::Error>>(
                    "region_error",
                    BatchGetResponse::get_region_error_for_reflect,
                    BatchGetResponse::mut_region_error_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<KvPair>>(
                    "pairs",
                    BatchGetResponse::get_pairs_for_reflect,
                    BatchGetResponse::mut_pairs_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BatchGetResponse>(
                    "BatchGetResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BatchGetResponse {
    fn clear(&mut self) {
        self.clear_region_error();
        self.clear_pairs();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BatchGetResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BatchGetResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ScanLockRequest {
    // message fields
    context: ::protobuf::SingularPtrField<Context>,
    pub max_version: u64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ScanLockRequest {}

impl ScanLockRequest {
    pub fn new() -> ScanLockRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ScanLockRequest {
        static mut instance: ::protobuf::lazy::Lazy<ScanLockRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ScanLockRequest,
        };
        unsafe {
            instance.get(ScanLockRequest::new)
        }
    }

    // .kvrpcpb.Context context = 1;

    pub fn clear_context(&mut self) {
        self.context.clear();
    }

    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }

    // Param is passed by value, moved
    pub fn set_context(&mut self, v: Context) {
        self.context = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context.set_default();
        }
        self.context.as_mut().unwrap()
    }

    // Take field
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(|| Context::new())
    }

    pub fn get_context(&self) -> &Context {
        self.context.as_ref().unwrap_or_else(|| Context::default_instance())
    }

    fn get_context_for_reflect(&self) -> &::protobuf::SingularPtrField<Context> {
        &self.context
    }

    fn mut_context_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Context> {
        &mut self.context
    }

    // uint64 max_version = 2;

    pub fn clear_max_version(&mut self) {
        self.max_version = 0;
    }

    // Param is passed by value, moved
    pub fn set_max_version(&mut self, v: u64) {
        self.max_version = v;
    }

    pub fn get_max_version(&self) -> u64 {
        self.max_version
    }

    fn get_max_version_for_reflect(&self) -> &u64 {
        &self.max_version
    }

    fn mut_max_version_for_reflect(&mut self) -> &mut u64 {
        &mut self.max_version
    }
}

impl ::protobuf::Message for ScanLockRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.context {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.context)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.max_version = tmp;
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
        if let Some(ref v) = self.context.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.max_version != 0 {
            my_size += ::protobuf::rt::value_size(2, self.max_version, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.context.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.max_version != 0 {
            os.write_uint64(2, self.max_version)?;
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

impl ::protobuf::MessageStatic for ScanLockRequest {
    fn new() -> ScanLockRequest {
        ScanLockRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<ScanLockRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Context>>(
                    "context",
                    ScanLockRequest::get_context_for_reflect,
                    ScanLockRequest::mut_context_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "max_version",
                    ScanLockRequest::get_max_version_for_reflect,
                    ScanLockRequest::mut_max_version_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ScanLockRequest>(
                    "ScanLockRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ScanLockRequest {
    fn clear(&mut self) {
        self.clear_context();
        self.clear_max_version();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ScanLockRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ScanLockRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ScanLockResponse {
    // message fields
    region_error: ::protobuf::SingularPtrField<super::errorpb::Error>,
    error: ::protobuf::SingularPtrField<KeyError>,
    locks: ::protobuf::RepeatedField<LockInfo>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ScanLockResponse {}

impl ScanLockResponse {
    pub fn new() -> ScanLockResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ScanLockResponse {
        static mut instance: ::protobuf::lazy::Lazy<ScanLockResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ScanLockResponse,
        };
        unsafe {
            instance.get(ScanLockResponse::new)
        }
    }

    // .errorpb.Error region_error = 1;

    pub fn clear_region_error(&mut self) {
        self.region_error.clear();
    }

    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error.set_default();
        }
        self.region_error.as_mut().unwrap()
    }

    // Take field
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error.take().unwrap_or_else(|| super::errorpb::Error::new())
    }

    pub fn get_region_error(&self) -> &super::errorpb::Error {
        self.region_error.as_ref().unwrap_or_else(|| super::errorpb::Error::default_instance())
    }

    fn get_region_error_for_reflect(&self) -> &::protobuf::SingularPtrField<super::errorpb::Error> {
        &self.region_error
    }

    fn mut_region_error_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::errorpb::Error> {
        &mut self.region_error
    }

    // .kvrpcpb.KeyError error = 2;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: KeyError) {
        self.error = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut KeyError {
        if self.error.is_none() {
            self.error.set_default();
        }
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> KeyError {
        self.error.take().unwrap_or_else(|| KeyError::new())
    }

    pub fn get_error(&self) -> &KeyError {
        self.error.as_ref().unwrap_or_else(|| KeyError::default_instance())
    }

    fn get_error_for_reflect(&self) -> &::protobuf::SingularPtrField<KeyError> {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<KeyError> {
        &mut self.error
    }

    // repeated .kvrpcpb.LockInfo locks = 3;

    pub fn clear_locks(&mut self) {
        self.locks.clear();
    }

    // Param is passed by value, moved
    pub fn set_locks(&mut self, v: ::protobuf::RepeatedField<LockInfo>) {
        self.locks = v;
    }

    // Mutable pointer to the field.
    pub fn mut_locks(&mut self) -> &mut ::protobuf::RepeatedField<LockInfo> {
        &mut self.locks
    }

    // Take field
    pub fn take_locks(&mut self) -> ::protobuf::RepeatedField<LockInfo> {
        ::std::mem::replace(&mut self.locks, ::protobuf::RepeatedField::new())
    }

    pub fn get_locks(&self) -> &[LockInfo] {
        &self.locks
    }

    fn get_locks_for_reflect(&self) -> &::protobuf::RepeatedField<LockInfo> {
        &self.locks
    }

    fn mut_locks_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<LockInfo> {
        &mut self.locks
    }
}

impl ::protobuf::Message for ScanLockResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.region_error {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.error {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.locks {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.region_error)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.error)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.locks)?;
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
        if let Some(ref v) = self.region_error.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.error.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.locks {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.region_error.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.error.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.locks {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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

impl ::protobuf::MessageStatic for ScanLockResponse {
    fn new() -> ScanLockResponse {
        ScanLockResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<ScanLockResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::errorpb::Error>>(
                    "region_error",
                    ScanLockResponse::get_region_error_for_reflect,
                    ScanLockResponse::mut_region_error_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<KeyError>>(
                    "error",
                    ScanLockResponse::get_error_for_reflect,
                    ScanLockResponse::mut_error_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<LockInfo>>(
                    "locks",
                    ScanLockResponse::get_locks_for_reflect,
                    ScanLockResponse::mut_locks_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ScanLockResponse>(
                    "ScanLockResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ScanLockResponse {
    fn clear(&mut self) {
        self.clear_region_error();
        self.clear_error();
        self.clear_locks();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ScanLockResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ScanLockResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ResolveLockRequest {
    // message fields
    context: ::protobuf::SingularPtrField<Context>,
    pub start_version: u64,
    pub commit_version: u64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResolveLockRequest {}

impl ResolveLockRequest {
    pub fn new() -> ResolveLockRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResolveLockRequest {
        static mut instance: ::protobuf::lazy::Lazy<ResolveLockRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResolveLockRequest,
        };
        unsafe {
            instance.get(ResolveLockRequest::new)
        }
    }

    // .kvrpcpb.Context context = 1;

    pub fn clear_context(&mut self) {
        self.context.clear();
    }

    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }

    // Param is passed by value, moved
    pub fn set_context(&mut self, v: Context) {
        self.context = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context.set_default();
        }
        self.context.as_mut().unwrap()
    }

    // Take field
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(|| Context::new())
    }

    pub fn get_context(&self) -> &Context {
        self.context.as_ref().unwrap_or_else(|| Context::default_instance())
    }

    fn get_context_for_reflect(&self) -> &::protobuf::SingularPtrField<Context> {
        &self.context
    }

    fn mut_context_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Context> {
        &mut self.context
    }

    // uint64 start_version = 2;

    pub fn clear_start_version(&mut self) {
        self.start_version = 0;
    }

    // Param is passed by value, moved
    pub fn set_start_version(&mut self, v: u64) {
        self.start_version = v;
    }

    pub fn get_start_version(&self) -> u64 {
        self.start_version
    }

    fn get_start_version_for_reflect(&self) -> &u64 {
        &self.start_version
    }

    fn mut_start_version_for_reflect(&mut self) -> &mut u64 {
        &mut self.start_version
    }

    // uint64 commit_version = 3;

    pub fn clear_commit_version(&mut self) {
        self.commit_version = 0;
    }

    // Param is passed by value, moved
    pub fn set_commit_version(&mut self, v: u64) {
        self.commit_version = v;
    }

    pub fn get_commit_version(&self) -> u64 {
        self.commit_version
    }

    fn get_commit_version_for_reflect(&self) -> &u64 {
        &self.commit_version
    }

    fn mut_commit_version_for_reflect(&mut self) -> &mut u64 {
        &mut self.commit_version
    }
}

impl ::protobuf::Message for ResolveLockRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.context {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.context)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.start_version = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.commit_version = tmp;
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
        if let Some(ref v) = self.context.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.start_version != 0 {
            my_size += ::protobuf::rt::value_size(2, self.start_version, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.commit_version != 0 {
            my_size += ::protobuf::rt::value_size(3, self.commit_version, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.context.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.start_version != 0 {
            os.write_uint64(2, self.start_version)?;
        }
        if self.commit_version != 0 {
            os.write_uint64(3, self.commit_version)?;
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

impl ::protobuf::MessageStatic for ResolveLockRequest {
    fn new() -> ResolveLockRequest {
        ResolveLockRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResolveLockRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Context>>(
                    "context",
                    ResolveLockRequest::get_context_for_reflect,
                    ResolveLockRequest::mut_context_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "start_version",
                    ResolveLockRequest::get_start_version_for_reflect,
                    ResolveLockRequest::mut_start_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "commit_version",
                    ResolveLockRequest::get_commit_version_for_reflect,
                    ResolveLockRequest::mut_commit_version_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ResolveLockRequest>(
                    "ResolveLockRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResolveLockRequest {
    fn clear(&mut self) {
        self.clear_context();
        self.clear_start_version();
        self.clear_commit_version();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ResolveLockRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResolveLockRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ResolveLockResponse {
    // message fields
    region_error: ::protobuf::SingularPtrField<super::errorpb::Error>,
    error: ::protobuf::SingularPtrField<KeyError>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResolveLockResponse {}

impl ResolveLockResponse {
    pub fn new() -> ResolveLockResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResolveLockResponse {
        static mut instance: ::protobuf::lazy::Lazy<ResolveLockResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResolveLockResponse,
        };
        unsafe {
            instance.get(ResolveLockResponse::new)
        }
    }

    // .errorpb.Error region_error = 1;

    pub fn clear_region_error(&mut self) {
        self.region_error.clear();
    }

    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error.set_default();
        }
        self.region_error.as_mut().unwrap()
    }

    // Take field
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error.take().unwrap_or_else(|| super::errorpb::Error::new())
    }

    pub fn get_region_error(&self) -> &super::errorpb::Error {
        self.region_error.as_ref().unwrap_or_else(|| super::errorpb::Error::default_instance())
    }

    fn get_region_error_for_reflect(&self) -> &::protobuf::SingularPtrField<super::errorpb::Error> {
        &self.region_error
    }

    fn mut_region_error_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::errorpb::Error> {
        &mut self.region_error
    }

    // .kvrpcpb.KeyError error = 2;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: KeyError) {
        self.error = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut KeyError {
        if self.error.is_none() {
            self.error.set_default();
        }
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> KeyError {
        self.error.take().unwrap_or_else(|| KeyError::new())
    }

    pub fn get_error(&self) -> &KeyError {
        self.error.as_ref().unwrap_or_else(|| KeyError::default_instance())
    }

    fn get_error_for_reflect(&self) -> &::protobuf::SingularPtrField<KeyError> {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<KeyError> {
        &mut self.error
    }
}

impl ::protobuf::Message for ResolveLockResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.region_error {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.error {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.region_error)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.error)?;
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
        if let Some(ref v) = self.region_error.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.error.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.region_error.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.error.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for ResolveLockResponse {
    fn new() -> ResolveLockResponse {
        ResolveLockResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResolveLockResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::errorpb::Error>>(
                    "region_error",
                    ResolveLockResponse::get_region_error_for_reflect,
                    ResolveLockResponse::mut_region_error_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<KeyError>>(
                    "error",
                    ResolveLockResponse::get_error_for_reflect,
                    ResolveLockResponse::mut_error_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ResolveLockResponse>(
                    "ResolveLockResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResolveLockResponse {
    fn clear(&mut self) {
        self.clear_region_error();
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ResolveLockResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResolveLockResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GCRequest {
    // message fields
    context: ::protobuf::SingularPtrField<Context>,
    pub safe_point: u64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GCRequest {}

impl GCRequest {
    pub fn new() -> GCRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GCRequest {
        static mut instance: ::protobuf::lazy::Lazy<GCRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GCRequest,
        };
        unsafe {
            instance.get(GCRequest::new)
        }
    }

    // .kvrpcpb.Context context = 1;

    pub fn clear_context(&mut self) {
        self.context.clear();
    }

    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }

    // Param is passed by value, moved
    pub fn set_context(&mut self, v: Context) {
        self.context = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context.set_default();
        }
        self.context.as_mut().unwrap()
    }

    // Take field
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(|| Context::new())
    }

    pub fn get_context(&self) -> &Context {
        self.context.as_ref().unwrap_or_else(|| Context::default_instance())
    }

    fn get_context_for_reflect(&self) -> &::protobuf::SingularPtrField<Context> {
        &self.context
    }

    fn mut_context_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Context> {
        &mut self.context
    }

    // uint64 safe_point = 2;

    pub fn clear_safe_point(&mut self) {
        self.safe_point = 0;
    }

    // Param is passed by value, moved
    pub fn set_safe_point(&mut self, v: u64) {
        self.safe_point = v;
    }

    pub fn get_safe_point(&self) -> u64 {
        self.safe_point
    }

    fn get_safe_point_for_reflect(&self) -> &u64 {
        &self.safe_point
    }

    fn mut_safe_point_for_reflect(&mut self) -> &mut u64 {
        &mut self.safe_point
    }
}

impl ::protobuf::Message for GCRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.context {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.context)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.safe_point = tmp;
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
        if let Some(ref v) = self.context.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.safe_point != 0 {
            my_size += ::protobuf::rt::value_size(2, self.safe_point, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.context.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.safe_point != 0 {
            os.write_uint64(2, self.safe_point)?;
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

impl ::protobuf::MessageStatic for GCRequest {
    fn new() -> GCRequest {
        GCRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<GCRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Context>>(
                    "context",
                    GCRequest::get_context_for_reflect,
                    GCRequest::mut_context_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "safe_point",
                    GCRequest::get_safe_point_for_reflect,
                    GCRequest::mut_safe_point_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GCRequest>(
                    "GCRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GCRequest {
    fn clear(&mut self) {
        self.clear_context();
        self.clear_safe_point();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GCRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GCRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GCResponse {
    // message fields
    region_error: ::protobuf::SingularPtrField<super::errorpb::Error>,
    error: ::protobuf::SingularPtrField<KeyError>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GCResponse {}

impl GCResponse {
    pub fn new() -> GCResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GCResponse {
        static mut instance: ::protobuf::lazy::Lazy<GCResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GCResponse,
        };
        unsafe {
            instance.get(GCResponse::new)
        }
    }

    // .errorpb.Error region_error = 1;

    pub fn clear_region_error(&mut self) {
        self.region_error.clear();
    }

    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error.set_default();
        }
        self.region_error.as_mut().unwrap()
    }

    // Take field
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error.take().unwrap_or_else(|| super::errorpb::Error::new())
    }

    pub fn get_region_error(&self) -> &super::errorpb::Error {
        self.region_error.as_ref().unwrap_or_else(|| super::errorpb::Error::default_instance())
    }

    fn get_region_error_for_reflect(&self) -> &::protobuf::SingularPtrField<super::errorpb::Error> {
        &self.region_error
    }

    fn mut_region_error_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::errorpb::Error> {
        &mut self.region_error
    }

    // .kvrpcpb.KeyError error = 2;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: KeyError) {
        self.error = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut KeyError {
        if self.error.is_none() {
            self.error.set_default();
        }
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> KeyError {
        self.error.take().unwrap_or_else(|| KeyError::new())
    }

    pub fn get_error(&self) -> &KeyError {
        self.error.as_ref().unwrap_or_else(|| KeyError::default_instance())
    }

    fn get_error_for_reflect(&self) -> &::protobuf::SingularPtrField<KeyError> {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<KeyError> {
        &mut self.error
    }
}

impl ::protobuf::Message for GCResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.region_error {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.error {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.region_error)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.error)?;
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
        if let Some(ref v) = self.region_error.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.error.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.region_error.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.error.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for GCResponse {
    fn new() -> GCResponse {
        GCResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<GCResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::errorpb::Error>>(
                    "region_error",
                    GCResponse::get_region_error_for_reflect,
                    GCResponse::mut_region_error_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<KeyError>>(
                    "error",
                    GCResponse::get_error_for_reflect,
                    GCResponse::mut_error_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GCResponse>(
                    "GCResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GCResponse {
    fn clear(&mut self) {
        self.clear_region_error();
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GCResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GCResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RawGetRequest {
    // message fields
    context: ::protobuf::SingularPtrField<Context>,
    pub key: ::bytes::Bytes,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RawGetRequest {}

impl RawGetRequest {
    pub fn new() -> RawGetRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RawGetRequest {
        static mut instance: ::protobuf::lazy::Lazy<RawGetRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RawGetRequest,
        };
        unsafe {
            instance.get(RawGetRequest::new)
        }
    }

    // .kvrpcpb.Context context = 1;

    pub fn clear_context(&mut self) {
        self.context.clear();
    }

    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }

    // Param is passed by value, moved
    pub fn set_context(&mut self, v: Context) {
        self.context = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context.set_default();
        }
        self.context.as_mut().unwrap()
    }

    // Take field
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(|| Context::new())
    }

    pub fn get_context(&self) -> &Context {
        self.context.as_ref().unwrap_or_else(|| Context::default_instance())
    }

    fn get_context_for_reflect(&self) -> &::protobuf::SingularPtrField<Context> {
        &self.context
    }

    fn mut_context_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Context> {
        &mut self.context
    }

    // bytes key = 2;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::bytes::Bytes) {
        self.key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::bytes::Bytes {
        &mut self.key
    }

    // Take field
    pub fn take_key(&mut self) -> ::bytes::Bytes {
        ::std::mem::replace(&mut self.key, ::bytes::Bytes::new())
    }

    pub fn get_key(&self) -> &[u8] {
        &self.key
    }

    fn get_key_for_reflect(&self) -> &::bytes::Bytes {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::bytes::Bytes {
        &mut self.key
    }
}

impl ::protobuf::Message for RawGetRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.context {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.context)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_carllerche_bytes_into(wire_type, is, &mut self.key)?;
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
        if let Some(ref v) = self.context.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.key);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.context.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.key.is_empty() {
            os.write_bytes(2, &self.key)?;
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

impl ::protobuf::MessageStatic for RawGetRequest {
    fn new() -> RawGetRequest {
        RawGetRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<RawGetRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Context>>(
                    "context",
                    RawGetRequest::get_context_for_reflect,
                    RawGetRequest::mut_context_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheBytes>(
                    "key",
                    RawGetRequest::get_key_for_reflect,
                    RawGetRequest::mut_key_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RawGetRequest>(
                    "RawGetRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RawGetRequest {
    fn clear(&mut self) {
        self.clear_context();
        self.clear_key();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RawGetRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RawGetRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RawGetResponse {
    // message fields
    region_error: ::protobuf::SingularPtrField<super::errorpb::Error>,
    pub error: ::protobuf::chars::Chars,
    pub value: ::bytes::Bytes,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RawGetResponse {}

impl RawGetResponse {
    pub fn new() -> RawGetResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RawGetResponse {
        static mut instance: ::protobuf::lazy::Lazy<RawGetResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RawGetResponse,
        };
        unsafe {
            instance.get(RawGetResponse::new)
        }
    }

    // .errorpb.Error region_error = 1;

    pub fn clear_region_error(&mut self) {
        self.region_error.clear();
    }

    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error.set_default();
        }
        self.region_error.as_mut().unwrap()
    }

    // Take field
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error.take().unwrap_or_else(|| super::errorpb::Error::new())
    }

    pub fn get_region_error(&self) -> &super::errorpb::Error {
        self.region_error.as_ref().unwrap_or_else(|| super::errorpb::Error::default_instance())
    }

    fn get_region_error_for_reflect(&self) -> &::protobuf::SingularPtrField<super::errorpb::Error> {
        &self.region_error
    }

    fn mut_region_error_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::errorpb::Error> {
        &mut self.region_error
    }

    // string error = 2;

    pub fn clear_error(&mut self) {
        ::protobuf::Clear::clear(&mut self.error);
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ::protobuf::chars::Chars) {
        self.error = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut ::protobuf::chars::Chars {
        &mut self.error
    }

    // Take field
    pub fn take_error(&mut self) -> ::protobuf::chars::Chars {
        ::std::mem::replace(&mut self.error, ::protobuf::chars::Chars::new())
    }

    pub fn get_error(&self) -> &str {
        &self.error
    }

    fn get_error_for_reflect(&self) -> &::protobuf::chars::Chars {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::protobuf::chars::Chars {
        &mut self.error
    }

    // bytes value = 3;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::bytes::Bytes) {
        self.value = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::bytes::Bytes {
        &mut self.value
    }

    // Take field
    pub fn take_value(&mut self) -> ::bytes::Bytes {
        ::std::mem::replace(&mut self.value, ::bytes::Bytes::new())
    }

    pub fn get_value(&self) -> &[u8] {
        &self.value
    }

    fn get_value_for_reflect(&self) -> &::bytes::Bytes {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::bytes::Bytes {
        &mut self.value
    }
}

impl ::protobuf::Message for RawGetResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.region_error {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.region_error)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_carllerche_string_into(wire_type, is, &mut self.error)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_carllerche_bytes_into(wire_type, is, &mut self.value)?;
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
        if let Some(ref v) = self.region_error.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.error.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.error);
        }
        if !self.value.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.value);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.region_error.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.error.is_empty() {
            os.write_string(2, &self.error)?;
        }
        if !self.value.is_empty() {
            os.write_bytes(3, &self.value)?;
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

impl ::protobuf::MessageStatic for RawGetResponse {
    fn new() -> RawGetResponse {
        RawGetResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<RawGetResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::errorpb::Error>>(
                    "region_error",
                    RawGetResponse::get_region_error_for_reflect,
                    RawGetResponse::mut_region_error_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheChars>(
                    "error",
                    RawGetResponse::get_error_for_reflect,
                    RawGetResponse::mut_error_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheBytes>(
                    "value",
                    RawGetResponse::get_value_for_reflect,
                    RawGetResponse::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RawGetResponse>(
                    "RawGetResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RawGetResponse {
    fn clear(&mut self) {
        self.clear_region_error();
        self.clear_error();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RawGetResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RawGetResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RawPutRequest {
    // message fields
    context: ::protobuf::SingularPtrField<Context>,
    pub key: ::bytes::Bytes,
    pub value: ::bytes::Bytes,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RawPutRequest {}

impl RawPutRequest {
    pub fn new() -> RawPutRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RawPutRequest {
        static mut instance: ::protobuf::lazy::Lazy<RawPutRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RawPutRequest,
        };
        unsafe {
            instance.get(RawPutRequest::new)
        }
    }

    // .kvrpcpb.Context context = 1;

    pub fn clear_context(&mut self) {
        self.context.clear();
    }

    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }

    // Param is passed by value, moved
    pub fn set_context(&mut self, v: Context) {
        self.context = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context.set_default();
        }
        self.context.as_mut().unwrap()
    }

    // Take field
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(|| Context::new())
    }

    pub fn get_context(&self) -> &Context {
        self.context.as_ref().unwrap_or_else(|| Context::default_instance())
    }

    fn get_context_for_reflect(&self) -> &::protobuf::SingularPtrField<Context> {
        &self.context
    }

    fn mut_context_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Context> {
        &mut self.context
    }

    // bytes key = 2;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::bytes::Bytes) {
        self.key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::bytes::Bytes {
        &mut self.key
    }

    // Take field
    pub fn take_key(&mut self) -> ::bytes::Bytes {
        ::std::mem::replace(&mut self.key, ::bytes::Bytes::new())
    }

    pub fn get_key(&self) -> &[u8] {
        &self.key
    }

    fn get_key_for_reflect(&self) -> &::bytes::Bytes {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::bytes::Bytes {
        &mut self.key
    }

    // bytes value = 3;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::bytes::Bytes) {
        self.value = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::bytes::Bytes {
        &mut self.value
    }

    // Take field
    pub fn take_value(&mut self) -> ::bytes::Bytes {
        ::std::mem::replace(&mut self.value, ::bytes::Bytes::new())
    }

    pub fn get_value(&self) -> &[u8] {
        &self.value
    }

    fn get_value_for_reflect(&self) -> &::bytes::Bytes {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::bytes::Bytes {
        &mut self.value
    }
}

impl ::protobuf::Message for RawPutRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.context {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.context)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_carllerche_bytes_into(wire_type, is, &mut self.key)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_carllerche_bytes_into(wire_type, is, &mut self.value)?;
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
        if let Some(ref v) = self.context.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.key);
        }
        if !self.value.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.value);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.context.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.key.is_empty() {
            os.write_bytes(2, &self.key)?;
        }
        if !self.value.is_empty() {
            os.write_bytes(3, &self.value)?;
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

impl ::protobuf::MessageStatic for RawPutRequest {
    fn new() -> RawPutRequest {
        RawPutRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<RawPutRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Context>>(
                    "context",
                    RawPutRequest::get_context_for_reflect,
                    RawPutRequest::mut_context_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheBytes>(
                    "key",
                    RawPutRequest::get_key_for_reflect,
                    RawPutRequest::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheBytes>(
                    "value",
                    RawPutRequest::get_value_for_reflect,
                    RawPutRequest::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RawPutRequest>(
                    "RawPutRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RawPutRequest {
    fn clear(&mut self) {
        self.clear_context();
        self.clear_key();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RawPutRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RawPutRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RawPutResponse {
    // message fields
    region_error: ::protobuf::SingularPtrField<super::errorpb::Error>,
    pub error: ::protobuf::chars::Chars,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RawPutResponse {}

impl RawPutResponse {
    pub fn new() -> RawPutResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RawPutResponse {
        static mut instance: ::protobuf::lazy::Lazy<RawPutResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RawPutResponse,
        };
        unsafe {
            instance.get(RawPutResponse::new)
        }
    }

    // .errorpb.Error region_error = 1;

    pub fn clear_region_error(&mut self) {
        self.region_error.clear();
    }

    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error.set_default();
        }
        self.region_error.as_mut().unwrap()
    }

    // Take field
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error.take().unwrap_or_else(|| super::errorpb::Error::new())
    }

    pub fn get_region_error(&self) -> &super::errorpb::Error {
        self.region_error.as_ref().unwrap_or_else(|| super::errorpb::Error::default_instance())
    }

    fn get_region_error_for_reflect(&self) -> &::protobuf::SingularPtrField<super::errorpb::Error> {
        &self.region_error
    }

    fn mut_region_error_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::errorpb::Error> {
        &mut self.region_error
    }

    // string error = 2;

    pub fn clear_error(&mut self) {
        ::protobuf::Clear::clear(&mut self.error);
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ::protobuf::chars::Chars) {
        self.error = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut ::protobuf::chars::Chars {
        &mut self.error
    }

    // Take field
    pub fn take_error(&mut self) -> ::protobuf::chars::Chars {
        ::std::mem::replace(&mut self.error, ::protobuf::chars::Chars::new())
    }

    pub fn get_error(&self) -> &str {
        &self.error
    }

    fn get_error_for_reflect(&self) -> &::protobuf::chars::Chars {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::protobuf::chars::Chars {
        &mut self.error
    }
}

impl ::protobuf::Message for RawPutResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.region_error {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.region_error)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_carllerche_string_into(wire_type, is, &mut self.error)?;
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
        if let Some(ref v) = self.region_error.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.error.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.error);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.region_error.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.error.is_empty() {
            os.write_string(2, &self.error)?;
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

impl ::protobuf::MessageStatic for RawPutResponse {
    fn new() -> RawPutResponse {
        RawPutResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<RawPutResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::errorpb::Error>>(
                    "region_error",
                    RawPutResponse::get_region_error_for_reflect,
                    RawPutResponse::mut_region_error_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheChars>(
                    "error",
                    RawPutResponse::get_error_for_reflect,
                    RawPutResponse::mut_error_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RawPutResponse>(
                    "RawPutResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RawPutResponse {
    fn clear(&mut self) {
        self.clear_region_error();
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RawPutResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RawPutResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RawDeleteRequest {
    // message fields
    context: ::protobuf::SingularPtrField<Context>,
    pub key: ::bytes::Bytes,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RawDeleteRequest {}

impl RawDeleteRequest {
    pub fn new() -> RawDeleteRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RawDeleteRequest {
        static mut instance: ::protobuf::lazy::Lazy<RawDeleteRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RawDeleteRequest,
        };
        unsafe {
            instance.get(RawDeleteRequest::new)
        }
    }

    // .kvrpcpb.Context context = 1;

    pub fn clear_context(&mut self) {
        self.context.clear();
    }

    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }

    // Param is passed by value, moved
    pub fn set_context(&mut self, v: Context) {
        self.context = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context.set_default();
        }
        self.context.as_mut().unwrap()
    }

    // Take field
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(|| Context::new())
    }

    pub fn get_context(&self) -> &Context {
        self.context.as_ref().unwrap_or_else(|| Context::default_instance())
    }

    fn get_context_for_reflect(&self) -> &::protobuf::SingularPtrField<Context> {
        &self.context
    }

    fn mut_context_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Context> {
        &mut self.context
    }

    // bytes key = 2;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::bytes::Bytes) {
        self.key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::bytes::Bytes {
        &mut self.key
    }

    // Take field
    pub fn take_key(&mut self) -> ::bytes::Bytes {
        ::std::mem::replace(&mut self.key, ::bytes::Bytes::new())
    }

    pub fn get_key(&self) -> &[u8] {
        &self.key
    }

    fn get_key_for_reflect(&self) -> &::bytes::Bytes {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::bytes::Bytes {
        &mut self.key
    }
}

impl ::protobuf::Message for RawDeleteRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.context {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.context)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_carllerche_bytes_into(wire_type, is, &mut self.key)?;
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
        if let Some(ref v) = self.context.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.key);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.context.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.key.is_empty() {
            os.write_bytes(2, &self.key)?;
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

impl ::protobuf::MessageStatic for RawDeleteRequest {
    fn new() -> RawDeleteRequest {
        RawDeleteRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<RawDeleteRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Context>>(
                    "context",
                    RawDeleteRequest::get_context_for_reflect,
                    RawDeleteRequest::mut_context_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheBytes>(
                    "key",
                    RawDeleteRequest::get_key_for_reflect,
                    RawDeleteRequest::mut_key_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RawDeleteRequest>(
                    "RawDeleteRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RawDeleteRequest {
    fn clear(&mut self) {
        self.clear_context();
        self.clear_key();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RawDeleteRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RawDeleteRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RawDeleteResponse {
    // message fields
    region_error: ::protobuf::SingularPtrField<super::errorpb::Error>,
    pub error: ::protobuf::chars::Chars,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RawDeleteResponse {}

impl RawDeleteResponse {
    pub fn new() -> RawDeleteResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RawDeleteResponse {
        static mut instance: ::protobuf::lazy::Lazy<RawDeleteResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RawDeleteResponse,
        };
        unsafe {
            instance.get(RawDeleteResponse::new)
        }
    }

    // .errorpb.Error region_error = 1;

    pub fn clear_region_error(&mut self) {
        self.region_error.clear();
    }

    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error.set_default();
        }
        self.region_error.as_mut().unwrap()
    }

    // Take field
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error.take().unwrap_or_else(|| super::errorpb::Error::new())
    }

    pub fn get_region_error(&self) -> &super::errorpb::Error {
        self.region_error.as_ref().unwrap_or_else(|| super::errorpb::Error::default_instance())
    }

    fn get_region_error_for_reflect(&self) -> &::protobuf::SingularPtrField<super::errorpb::Error> {
        &self.region_error
    }

    fn mut_region_error_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::errorpb::Error> {
        &mut self.region_error
    }

    // string error = 2;

    pub fn clear_error(&mut self) {
        ::protobuf::Clear::clear(&mut self.error);
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ::protobuf::chars::Chars) {
        self.error = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut ::protobuf::chars::Chars {
        &mut self.error
    }

    // Take field
    pub fn take_error(&mut self) -> ::protobuf::chars::Chars {
        ::std::mem::replace(&mut self.error, ::protobuf::chars::Chars::new())
    }

    pub fn get_error(&self) -> &str {
        &self.error
    }

    fn get_error_for_reflect(&self) -> &::protobuf::chars::Chars {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::protobuf::chars::Chars {
        &mut self.error
    }
}

impl ::protobuf::Message for RawDeleteResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.region_error {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.region_error)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_carllerche_string_into(wire_type, is, &mut self.error)?;
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
        if let Some(ref v) = self.region_error.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.error.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.error);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.region_error.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.error.is_empty() {
            os.write_string(2, &self.error)?;
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

impl ::protobuf::MessageStatic for RawDeleteResponse {
    fn new() -> RawDeleteResponse {
        RawDeleteResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<RawDeleteResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::errorpb::Error>>(
                    "region_error",
                    RawDeleteResponse::get_region_error_for_reflect,
                    RawDeleteResponse::mut_region_error_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheChars>(
                    "error",
                    RawDeleteResponse::get_error_for_reflect,
                    RawDeleteResponse::mut_error_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RawDeleteResponse>(
                    "RawDeleteResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RawDeleteResponse {
    fn clear(&mut self) {
        self.clear_region_error();
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RawDeleteResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RawDeleteResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RawRequest {
    // message fields
    pub msg_id: u64,
    raw_get: ::protobuf::SingularPtrField<RawGetRequest>,
    raw_put: ::protobuf::SingularPtrField<RawPutRequest>,
    raw_delete: ::protobuf::SingularPtrField<RawDeleteRequest>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RawRequest {}

impl RawRequest {
    pub fn new() -> RawRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RawRequest {
        static mut instance: ::protobuf::lazy::Lazy<RawRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RawRequest,
        };
        unsafe {
            instance.get(RawRequest::new)
        }
    }

    // uint64 msg_id = 1;

    pub fn clear_msg_id(&mut self) {
        self.msg_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_msg_id(&mut self, v: u64) {
        self.msg_id = v;
    }

    pub fn get_msg_id(&self) -> u64 {
        self.msg_id
    }

    fn get_msg_id_for_reflect(&self) -> &u64 {
        &self.msg_id
    }

    fn mut_msg_id_for_reflect(&mut self) -> &mut u64 {
        &mut self.msg_id
    }

    // .kvrpcpb.RawGetRequest raw_get = 2;

    pub fn clear_raw_get(&mut self) {
        self.raw_get.clear();
    }

    pub fn has_raw_get(&self) -> bool {
        self.raw_get.is_some()
    }

    // Param is passed by value, moved
    pub fn set_raw_get(&mut self, v: RawGetRequest) {
        self.raw_get = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_raw_get(&mut self) -> &mut RawGetRequest {
        if self.raw_get.is_none() {
            self.raw_get.set_default();
        }
        self.raw_get.as_mut().unwrap()
    }

    // Take field
    pub fn take_raw_get(&mut self) -> RawGetRequest {
        self.raw_get.take().unwrap_or_else(|| RawGetRequest::new())
    }

    pub fn get_raw_get(&self) -> &RawGetRequest {
        self.raw_get.as_ref().unwrap_or_else(|| RawGetRequest::default_instance())
    }

    fn get_raw_get_for_reflect(&self) -> &::protobuf::SingularPtrField<RawGetRequest> {
        &self.raw_get
    }

    fn mut_raw_get_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<RawGetRequest> {
        &mut self.raw_get
    }

    // .kvrpcpb.RawPutRequest raw_put = 3;

    pub fn clear_raw_put(&mut self) {
        self.raw_put.clear();
    }

    pub fn has_raw_put(&self) -> bool {
        self.raw_put.is_some()
    }

    // Param is passed by value, moved
    pub fn set_raw_put(&mut self, v: RawPutRequest) {
        self.raw_put = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_raw_put(&mut self) -> &mut RawPutRequest {
        if self.raw_put.is_none() {
            self.raw_put.set_default();
        }
        self.raw_put.as_mut().unwrap()
    }

    // Take field
    pub fn take_raw_put(&mut self) -> RawPutRequest {
        self.raw_put.take().unwrap_or_else(|| RawPutRequest::new())
    }

    pub fn get_raw_put(&self) -> &RawPutRequest {
        self.raw_put.as_ref().unwrap_or_else(|| RawPutRequest::default_instance())
    }

    fn get_raw_put_for_reflect(&self) -> &::protobuf::SingularPtrField<RawPutRequest> {
        &self.raw_put
    }

    fn mut_raw_put_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<RawPutRequest> {
        &mut self.raw_put
    }

    // .kvrpcpb.RawDeleteRequest raw_delete = 4;

    pub fn clear_raw_delete(&mut self) {
        self.raw_delete.clear();
    }

    pub fn has_raw_delete(&self) -> bool {
        self.raw_delete.is_some()
    }

    // Param is passed by value, moved
    pub fn set_raw_delete(&mut self, v: RawDeleteRequest) {
        self.raw_delete = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_raw_delete(&mut self) -> &mut RawDeleteRequest {
        if self.raw_delete.is_none() {
            self.raw_delete.set_default();
        }
        self.raw_delete.as_mut().unwrap()
    }

    // Take field
    pub fn take_raw_delete(&mut self) -> RawDeleteRequest {
        self.raw_delete.take().unwrap_or_else(|| RawDeleteRequest::new())
    }

    pub fn get_raw_delete(&self) -> &RawDeleteRequest {
        self.raw_delete.as_ref().unwrap_or_else(|| RawDeleteRequest::default_instance())
    }

    fn get_raw_delete_for_reflect(&self) -> &::protobuf::SingularPtrField<RawDeleteRequest> {
        &self.raw_delete
    }

    fn mut_raw_delete_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<RawDeleteRequest> {
        &mut self.raw_delete
    }
}

impl ::protobuf::Message for RawRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.raw_get {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.raw_put {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.raw_delete {
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
                    let tmp = is.read_uint64()?;
                    self.msg_id = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.raw_get)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.raw_put)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.raw_delete)?;
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
        if self.msg_id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.msg_id, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.raw_get.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.raw_put.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.raw_delete.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.msg_id != 0 {
            os.write_uint64(1, self.msg_id)?;
        }
        if let Some(ref v) = self.raw_get.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.raw_put.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.raw_delete.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for RawRequest {
    fn new() -> RawRequest {
        RawRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<RawRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "msg_id",
                    RawRequest::get_msg_id_for_reflect,
                    RawRequest::mut_msg_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RawGetRequest>>(
                    "raw_get",
                    RawRequest::get_raw_get_for_reflect,
                    RawRequest::mut_raw_get_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RawPutRequest>>(
                    "raw_put",
                    RawRequest::get_raw_put_for_reflect,
                    RawRequest::mut_raw_put_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RawDeleteRequest>>(
                    "raw_delete",
                    RawRequest::get_raw_delete_for_reflect,
                    RawRequest::mut_raw_delete_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RawRequest>(
                    "RawRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RawRequest {
    fn clear(&mut self) {
        self.clear_msg_id();
        self.clear_raw_get();
        self.clear_raw_put();
        self.clear_raw_delete();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RawRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RawRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RawResponse {
    // message fields
    pub msg_id: u64,
    raw_get: ::protobuf::SingularPtrField<RawGetResponse>,
    raw_put: ::protobuf::SingularPtrField<RawPutResponse>,
    raw_delete: ::protobuf::SingularPtrField<RawDeleteResponse>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RawResponse {}

impl RawResponse {
    pub fn new() -> RawResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RawResponse {
        static mut instance: ::protobuf::lazy::Lazy<RawResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RawResponse,
        };
        unsafe {
            instance.get(RawResponse::new)
        }
    }

    // uint64 msg_id = 1;

    pub fn clear_msg_id(&mut self) {
        self.msg_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_msg_id(&mut self, v: u64) {
        self.msg_id = v;
    }

    pub fn get_msg_id(&self) -> u64 {
        self.msg_id
    }

    fn get_msg_id_for_reflect(&self) -> &u64 {
        &self.msg_id
    }

    fn mut_msg_id_for_reflect(&mut self) -> &mut u64 {
        &mut self.msg_id
    }

    // .kvrpcpb.RawGetResponse raw_get = 2;

    pub fn clear_raw_get(&mut self) {
        self.raw_get.clear();
    }

    pub fn has_raw_get(&self) -> bool {
        self.raw_get.is_some()
    }

    // Param is passed by value, moved
    pub fn set_raw_get(&mut self, v: RawGetResponse) {
        self.raw_get = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_raw_get(&mut self) -> &mut RawGetResponse {
        if self.raw_get.is_none() {
            self.raw_get.set_default();
        }
        self.raw_get.as_mut().unwrap()
    }

    // Take field
    pub fn take_raw_get(&mut self) -> RawGetResponse {
        self.raw_get.take().unwrap_or_else(|| RawGetResponse::new())
    }

    pub fn get_raw_get(&self) -> &RawGetResponse {
        self.raw_get.as_ref().unwrap_or_else(|| RawGetResponse::default_instance())
    }

    fn get_raw_get_for_reflect(&self) -> &::protobuf::SingularPtrField<RawGetResponse> {
        &self.raw_get
    }

    fn mut_raw_get_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<RawGetResponse> {
        &mut self.raw_get
    }

    // .kvrpcpb.RawPutResponse raw_put = 3;

    pub fn clear_raw_put(&mut self) {
        self.raw_put.clear();
    }

    pub fn has_raw_put(&self) -> bool {
        self.raw_put.is_some()
    }

    // Param is passed by value, moved
    pub fn set_raw_put(&mut self, v: RawPutResponse) {
        self.raw_put = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_raw_put(&mut self) -> &mut RawPutResponse {
        if self.raw_put.is_none() {
            self.raw_put.set_default();
        }
        self.raw_put.as_mut().unwrap()
    }

    // Take field
    pub fn take_raw_put(&mut self) -> RawPutResponse {
        self.raw_put.take().unwrap_or_else(|| RawPutResponse::new())
    }

    pub fn get_raw_put(&self) -> &RawPutResponse {
        self.raw_put.as_ref().unwrap_or_else(|| RawPutResponse::default_instance())
    }

    fn get_raw_put_for_reflect(&self) -> &::protobuf::SingularPtrField<RawPutResponse> {
        &self.raw_put
    }

    fn mut_raw_put_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<RawPutResponse> {
        &mut self.raw_put
    }

    // .kvrpcpb.RawDeleteResponse raw_delete = 4;

    pub fn clear_raw_delete(&mut self) {
        self.raw_delete.clear();
    }

    pub fn has_raw_delete(&self) -> bool {
        self.raw_delete.is_some()
    }

    // Param is passed by value, moved
    pub fn set_raw_delete(&mut self, v: RawDeleteResponse) {
        self.raw_delete = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_raw_delete(&mut self) -> &mut RawDeleteResponse {
        if self.raw_delete.is_none() {
            self.raw_delete.set_default();
        }
        self.raw_delete.as_mut().unwrap()
    }

    // Take field
    pub fn take_raw_delete(&mut self) -> RawDeleteResponse {
        self.raw_delete.take().unwrap_or_else(|| RawDeleteResponse::new())
    }

    pub fn get_raw_delete(&self) -> &RawDeleteResponse {
        self.raw_delete.as_ref().unwrap_or_else(|| RawDeleteResponse::default_instance())
    }

    fn get_raw_delete_for_reflect(&self) -> &::protobuf::SingularPtrField<RawDeleteResponse> {
        &self.raw_delete
    }

    fn mut_raw_delete_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<RawDeleteResponse> {
        &mut self.raw_delete
    }
}

impl ::protobuf::Message for RawResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.raw_get {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.raw_put {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.raw_delete {
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
                    let tmp = is.read_uint64()?;
                    self.msg_id = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.raw_get)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.raw_put)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.raw_delete)?;
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
        if self.msg_id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.msg_id, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.raw_get.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.raw_put.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.raw_delete.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.msg_id != 0 {
            os.write_uint64(1, self.msg_id)?;
        }
        if let Some(ref v) = self.raw_get.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.raw_put.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.raw_delete.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for RawResponse {
    fn new() -> RawResponse {
        RawResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<RawResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "msg_id",
                    RawResponse::get_msg_id_for_reflect,
                    RawResponse::mut_msg_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RawGetResponse>>(
                    "raw_get",
                    RawResponse::get_raw_get_for_reflect,
                    RawResponse::mut_raw_get_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RawPutResponse>>(
                    "raw_put",
                    RawResponse::get_raw_put_for_reflect,
                    RawResponse::mut_raw_put_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RawDeleteResponse>>(
                    "raw_delete",
                    RawResponse::get_raw_delete_for_reflect,
                    RawResponse::mut_raw_delete_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RawResponse>(
                    "RawResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RawResponse {
    fn clear(&mut self) {
        self.clear_msg_id();
        self.clear_raw_get();
        self.clear_raw_put();
        self.clear_raw_delete();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RawResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RawResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Op {
    Put = 0,
    Del = 1,
    Lock = 2,
}

impl ::protobuf::ProtobufEnum for Op {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Op> {
        match value {
            0 => ::std::option::Option::Some(Op::Put),
            1 => ::std::option::Option::Some(Op::Del),
            2 => ::std::option::Option::Some(Op::Lock),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Op] = &[
            Op::Put,
            Op::Del,
            Op::Lock,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Op>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Op", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Op {
}

impl ::std::default::Default for Op {
    fn default() -> Self {
        Op::Put
    }
}

impl ::protobuf::reflect::ProtobufValue for Op {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\rkvrpcpb.proto\x12\x07kvrpcpb\x1a\x0cmetapb.proto\x1a\rerrorpb.proto\
    \x1a\x14gogoproto/gogo.proto\x1a\x0frustproto.proto\"}\n\x08LockInfo\x12\
    !\n\x0cprimary_lock\x18\x01\x20\x01(\x0cR\x0bprimaryLock\x12!\n\x0clock_\
    version\x18\x02\x20\x01(\x04R\x0blockVersion\x12\x10\n\x03key\x18\x03\
    \x20\x01(\x0cR\x03key\x12\x19\n\x08lock_ttl\x18\x04\x20\x01(\x04R\x07loc\
    kTtl\"i\n\x08KeyError\x12)\n\x06locked\x18\x01\x20\x01(\x0b2\x11.kvrpcpb\
    .LockInfoR\x06locked\x12\x1c\n\tretryable\x18\x02\x20\x01(\tR\tretryable\
    \x12\x14\n\x05abort\x18\x03\x20\x01(\tR\x05abort\"\xb5\x01\n\x07Context\
    \x12\x1b\n\tregion_id\x18\x01\x20\x01(\x04R\x08regionId\x126\n\x0cregion\
    _epoch\x18\x02\x20\x01(\x0b2\x13.metapb.RegionEpochR\x0bregionEpoch\x12\
    \x20\n\x04peer\x18\x03\x20\x01(\x0b2\x0c.metapb.PeerR\x04peer\x12\x1f\n\
    \x0bread_quorum\x18\x04\x20\x01(\x08R\nreadQuorum\x12\x12\n\x04term\x18\
    \x05\x20\x01(\x04R\x04term\"d\n\nGetRequest\x12*\n\x07context\x18\x01\
    \x20\x01(\x0b2\x10.kvrpcpb.ContextR\x07context\x12\x10\n\x03key\x18\x02\
    \x20\x01(\x0cR\x03key\x12\x18\n\x07version\x18\x03\x20\x01(\x04R\x07vers\
    ion\"\x7f\n\x0bGetResponse\x121\n\x0cregion_error\x18\x01\x20\x01(\x0b2\
    \x0e.errorpb.ErrorR\x0bregionError\x12'\n\x05error\x18\x02\x20\x01(\x0b2\
    \x11.kvrpcpb.KeyErrorR\x05error\x12\x14\n\x05value\x18\x03\x20\x01(\x0cR\
    \x05value\"\xa1\x01\n\x0bScanRequest\x12*\n\x07context\x18\x01\x20\x01(\
    \x0b2\x10.kvrpcpb.ContextR\x07context\x12\x1b\n\tstart_key\x18\x02\x20\
    \x01(\x0cR\x08startKey\x12\x14\n\x05limit\x18\x03\x20\x01(\rR\x05limit\
    \x12\x18\n\x07version\x18\x04\x20\x01(\x04R\x07version\x12\x19\n\x08key_\
    only\x18\x05\x20\x01(\x08R\x07keyOnly\"Y\n\x06KvPair\x12'\n\x05error\x18\
    \x01\x20\x01(\x0b2\x11.kvrpcpb.KeyErrorR\x05error\x12\x10\n\x03key\x18\
    \x02\x20\x01(\x0cR\x03key\x12\x14\n\x05value\x18\x03\x20\x01(\x0cR\x05va\
    lue\"h\n\x0cScanResponse\x121\n\x0cregion_error\x18\x01\x20\x01(\x0b2\
    \x0e.errorpb.ErrorR\x0bregionError\x12%\n\x05pairs\x18\x02\x20\x03(\x0b2\
    \x0f.kvrpcpb.KvPairR\x05pairs\"O\n\x08Mutation\x12\x1b\n\x02op\x18\x01\
    \x20\x01(\x0e2\x0b.kvrpcpb.OpR\x02op\x12\x10\n\x03key\x18\x02\x20\x01(\
    \x0cR\x03key\x12\x14\n\x05value\x18\x03\x20\x01(\x0cR\x05value\"\x85\x02\
    \n\x0fPrewriteRequest\x12*\n\x07context\x18\x01\x20\x01(\x0b2\x10.kvrpcp\
    b.ContextR\x07context\x12/\n\tmutations\x18\x02\x20\x03(\x0b2\x11.kvrpcp\
    b.MutationR\tmutations\x12!\n\x0cprimary_lock\x18\x03\x20\x01(\x0cR\x0bp\
    rimaryLock\x12#\n\rstart_version\x18\x04\x20\x01(\x04R\x0cstartVersion\
    \x12\x19\n\x08lock_ttl\x18\x05\x20\x01(\x04R\x07lockTtl\x122\n\x15skip_c\
    onstraint_check\x18\x06\x20\x01(\x08R\x13skipConstraintCheck\"p\n\x10Pre\
    writeResponse\x121\n\x0cregion_error\x18\x01\x20\x01(\x0b2\x0e.errorpb.E\
    rrorR\x0bregionError\x12)\n\x06errors\x18\x02\x20\x03(\x0b2\x11.kvrpcpb.\
    KeyErrorR\x06errors\"\xb3\x01\n\rCommitRequest\x12*\n\x07context\x18\x01\
    \x20\x01(\x0b2\x10.kvrpcpb.ContextR\x07context\x12#\n\rstart_version\x18\
    \x02\x20\x01(\x04R\x0cstartVersion\x12\x12\n\x04keys\x18\x03\x20\x03(\
    \x0cR\x04keys\x12%\n\x0ecommit_version\x18\x04\x20\x01(\x04R\rcommitVers\
    ion\x12\x16\n\x06binlog\x18\x05\x20\x01(\x0cR\x06binlog\"l\n\x0eCommitRe\
    sponse\x121\n\x0cregion_error\x18\x01\x20\x01(\x0b2\x0e.errorpb.ErrorR\
    \x0bregionError\x12'\n\x05error\x18\x02\x20\x01(\x0b2\x11.kvrpcpb.KeyErr\
    orR\x05error\"g\n\rImportRequest\x12/\n\tmutations\x18\x01\x20\x03(\x0b2\
    \x11.kvrpcpb.MutationR\tmutations\x12%\n\x0ecommit_version\x18\x02\x20\
    \x01(\x04R\rcommitVersion\"Y\n\x0eImportResponse\x121\n\x0cregion_error\
    \x18\x01\x20\x01(\x0b2\x0e.errorpb.ErrorR\x0bregionError\x12\x14\n\x05er\
    ror\x18\x02\x20\x01(\tR\x05error\"{\n\x14BatchRollbackRequest\x12*\n\x07\
    context\x18\x01\x20\x01(\x0b2\x10.kvrpcpb.ContextR\x07context\x12#\n\rst\
    art_version\x18\x02\x20\x01(\x04R\x0cstartVersion\x12\x12\n\x04keys\x18\
    \x03\x20\x03(\x0cR\x04keys\"s\n\x15BatchRollbackResponse\x121\n\x0cregio\
    n_error\x18\x01\x20\x01(\x0b2\x0e.errorpb.ErrorR\x0bregionError\x12'\n\
    \x05error\x18\x02\x20\x01(\x0b2\x11.kvrpcpb.KeyErrorR\x05error\"s\n\x0eC\
    leanupRequest\x12*\n\x07context\x18\x01\x20\x01(\x0b2\x10.kvrpcpb.Contex\
    tR\x07context\x12\x10\n\x03key\x18\x02\x20\x01(\x0cR\x03key\x12#\n\rstar\
    t_version\x18\x03\x20\x01(\x04R\x0cstartVersion\"\x94\x01\n\x0fCleanupRe\
    sponse\x121\n\x0cregion_error\x18\x01\x20\x01(\x0b2\x0e.errorpb.ErrorR\
    \x0bregionError\x12'\n\x05error\x18\x02\x20\x01(\x0b2\x11.kvrpcpb.KeyErr\
    orR\x05error\x12%\n\x0ecommit_version\x18\x03\x20\x01(\x04R\rcommitVersi\
    on\"k\n\x0fBatchGetRequest\x12*\n\x07context\x18\x01\x20\x01(\x0b2\x10.k\
    vrpcpb.ContextR\x07context\x12\x12\n\x04keys\x18\x02\x20\x03(\x0cR\x04ke\
    ys\x12\x18\n\x07version\x18\x03\x20\x01(\x04R\x07version\"l\n\x10BatchGe\
    tResponse\x121\n\x0cregion_error\x18\x01\x20\x01(\x0b2\x0e.errorpb.Error\
    R\x0bregionError\x12%\n\x05pairs\x18\x02\x20\x03(\x0b2\x0f.kvrpcpb.KvPai\
    rR\x05pairs\"^\n\x0fScanLockRequest\x12*\n\x07context\x18\x01\x20\x01(\
    \x0b2\x10.kvrpcpb.ContextR\x07context\x12\x1f\n\x0bmax_version\x18\x02\
    \x20\x01(\x04R\nmaxVersion\"\x97\x01\n\x10ScanLockResponse\x121\n\x0creg\
    ion_error\x18\x01\x20\x01(\x0b2\x0e.errorpb.ErrorR\x0bregionError\x12'\n\
    \x05error\x18\x02\x20\x01(\x0b2\x11.kvrpcpb.KeyErrorR\x05error\x12'\n\
    \x05locks\x18\x03\x20\x03(\x0b2\x11.kvrpcpb.LockInfoR\x05locks\"\x8c\x01\
    \n\x12ResolveLockRequest\x12*\n\x07context\x18\x01\x20\x01(\x0b2\x10.kvr\
    pcpb.ContextR\x07context\x12#\n\rstart_version\x18\x02\x20\x01(\x04R\x0c\
    startVersion\x12%\n\x0ecommit_version\x18\x03\x20\x01(\x04R\rcommitVersi\
    on\"q\n\x13ResolveLockResponse\x121\n\x0cregion_error\x18\x01\x20\x01(\
    \x0b2\x0e.errorpb.ErrorR\x0bregionError\x12'\n\x05error\x18\x02\x20\x01(\
    \x0b2\x11.kvrpcpb.KeyErrorR\x05error\"V\n\tGCRequest\x12*\n\x07context\
    \x18\x01\x20\x01(\x0b2\x10.kvrpcpb.ContextR\x07context\x12\x1d\n\nsafe_p\
    oint\x18\x02\x20\x01(\x04R\tsafePoint\"h\n\nGCResponse\x121\n\x0cregion_\
    error\x18\x01\x20\x01(\x0b2\x0e.errorpb.ErrorR\x0bregionError\x12'\n\x05\
    error\x18\x02\x20\x01(\x0b2\x11.kvrpcpb.KeyErrorR\x05error\"M\n\rRawGetR\
    equest\x12*\n\x07context\x18\x01\x20\x01(\x0b2\x10.kvrpcpb.ContextR\x07c\
    ontext\x12\x10\n\x03key\x18\x02\x20\x01(\x0cR\x03key\"o\n\x0eRawGetRespo\
    nse\x121\n\x0cregion_error\x18\x01\x20\x01(\x0b2\x0e.errorpb.ErrorR\x0br\
    egionError\x12\x14\n\x05error\x18\x02\x20\x01(\tR\x05error\x12\x14\n\x05\
    value\x18\x03\x20\x01(\x0cR\x05value\"c\n\rRawPutRequest\x12*\n\x07conte\
    xt\x18\x01\x20\x01(\x0b2\x10.kvrpcpb.ContextR\x07context\x12\x10\n\x03ke\
    y\x18\x02\x20\x01(\x0cR\x03key\x12\x14\n\x05value\x18\x03\x20\x01(\x0cR\
    \x05value\"Y\n\x0eRawPutResponse\x121\n\x0cregion_error\x18\x01\x20\x01(\
    \x0b2\x0e.errorpb.ErrorR\x0bregionError\x12\x14\n\x05error\x18\x02\x20\
    \x01(\tR\x05error\"P\n\x10RawDeleteRequest\x12*\n\x07context\x18\x01\x20\
    \x01(\x0b2\x10.kvrpcpb.ContextR\x07context\x12\x10\n\x03key\x18\x02\x20\
    \x01(\x0cR\x03key\"\\\n\x11RawDeleteResponse\x121\n\x0cregion_error\x18\
    \x01\x20\x01(\x0b2\x0e.errorpb.ErrorR\x0bregionError\x12\x14\n\x05error\
    \x18\x02\x20\x01(\tR\x05error\"\xbf\x01\n\nRawRequest\x12\x15\n\x06msg_i\
    d\x18\x01\x20\x01(\x04R\x05msgId\x12/\n\x07raw_get\x18\x02\x20\x01(\x0b2\
    \x16.kvrpcpb.RawGetRequestR\x06rawGet\x12/\n\x07raw_put\x18\x03\x20\x01(\
    \x0b2\x16.kvrpcpb.RawPutRequestR\x06rawPut\x128\n\nraw_delete\x18\x04\
    \x20\x01(\x0b2\x19.kvrpcpb.RawDeleteRequestR\trawDelete\"\xc3\x01\n\x0bR\
    awResponse\x12\x15\n\x06msg_id\x18\x01\x20\x01(\x04R\x05msgId\x120\n\x07\
    raw_get\x18\x02\x20\x01(\x0b2\x17.kvrpcpb.RawGetResponseR\x06rawGet\x120\
    \n\x07raw_put\x18\x03\x20\x01(\x0b2\x17.kvrpcpb.RawPutResponseR\x06rawPu\
    t\x129\n\nraw_delete\x18\x04\x20\x01(\x0b2\x1a.kvrpcpb.RawDeleteResponse\
    R\trawDelete*\x20\n\x02Op\x12\x07\n\x03Put\x10\0\x12\x07\n\x03Del\x10\
    \x01\x12\x08\n\x04Lock\x10\x02B.\n\x18com.pingcap.tikv.kvproto\xa0\xa7\
    \x08\x01\x98\xa7\x08\x01\xc8\xe2\x1e\x01\xe0\xe2\x1e\x01\xd0\xe2\x1e\x01\
    J\xb2H\n\x07\x12\x05\0\0\xe6\x01\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\
    \x08\n\x01\x02\x12\x03\x01\x08\x0f\n\t\n\x02\x03\0\x12\x03\x03\x07\x15\n\
    \t\n\x02\x03\x01\x12\x03\x04\x07\x16\n\t\n\x02\x03\x02\x12\x03\x05\x07\
    \x1d\n\t\n\x02\x03\x03\x12\x03\x06\x07\x18\n\x08\n\x01\x08\x12\x03\x08\0\
    (\n\x0b\n\x04\x08\xe7\x07\0\x12\x03\x08\0(\n\x0c\n\x05\x08\xe7\x07\0\x02\
    \x12\x03\x08\x07\x20\n\r\n\x06\x08\xe7\x07\0\x02\0\x12\x03\x08\x07\x20\n\
    \x0e\n\x07\x08\xe7\x07\0\x02\0\x01\x12\x03\x08\x08\x1f\n\x0c\n\x05\x08\
    \xe7\x07\0\x03\x12\x03\x08#'\n\x08\n\x01\x08\x12\x03\t\0$\n\x0b\n\x04\
    \x08\xe7\x07\x01\x12\x03\t\0$\n\x0c\n\x05\x08\xe7\x07\x01\x02\x12\x03\t\
    \x07\x1c\n\r\n\x06\x08\xe7\x07\x01\x02\0\x12\x03\t\x07\x1c\n\x0e\n\x07\
    \x08\xe7\x07\x01\x02\0\x01\x12\x03\t\x08\x1b\n\x0c\n\x05\x08\xe7\x07\x01\
    \x03\x12\x03\t\x1f#\n\x08\n\x01\x08\x12\x03\n\0*\n\x0b\n\x04\x08\xe7\x07\
    \x02\x12\x03\n\0*\n\x0c\n\x05\x08\xe7\x07\x02\x02\x12\x03\n\x07\"\n\r\n\
    \x06\x08\xe7\x07\x02\x02\0\x12\x03\n\x07\"\n\x0e\n\x07\x08\xe7\x07\x02\
    \x02\0\x01\x12\x03\n\x08!\n\x0c\n\x05\x08\xe7\x07\x02\x03\x12\x03\n%)\n\
    \x08\n\x01\x08\x12\x03\x0c\09\n\x0b\n\x04\x08\xe7\x07\x03\x12\x03\x0c\09\
    \n\x0c\n\x05\x08\xe7\x07\x03\x02\x12\x03\x0c\x071\n\r\n\x06\x08\xe7\x07\
    \x03\x02\0\x12\x03\x0c\x071\n\x0e\n\x07\x08\xe7\x07\x03\x02\0\x01\x12\
    \x03\x0c\x080\n\x0c\n\x05\x08\xe7\x07\x03\x03\x12\x03\x0c48\n\x08\n\x01\
    \x08\x12\x03\r\0:\n\x0b\n\x04\x08\xe7\x07\x04\x12\x03\r\0:\n\x0c\n\x05\
    \x08\xe7\x07\x04\x02\x12\x03\r\x072\n\r\n\x06\x08\xe7\x07\x04\x02\0\x12\
    \x03\r\x072\n\x0e\n\x07\x08\xe7\x07\x04\x02\0\x01\x12\x03\r\x081\n\x0c\n\
    \x05\x08\xe7\x07\x04\x03\x12\x03\r59\n\x08\n\x01\x08\x12\x03\x0f\01\n\
    \x0b\n\x04\x08\xe7\x07\x05\x12\x03\x0f\01\n\x0c\n\x05\x08\xe7\x07\x05\
    \x02\x12\x03\x0f\x07\x13\n\r\n\x06\x08\xe7\x07\x05\x02\0\x12\x03\x0f\x07\
    \x13\n\x0e\n\x07\x08\xe7\x07\x05\x02\0\x01\x12\x03\x0f\x07\x13\n\x0c\n\
    \x05\x08\xe7\x07\x05\x07\x12\x03\x0f\x160\n\n\n\x02\x04\0\x12\x04\x11\0\
    \x16\x01\n\n\n\x03\x04\0\x01\x12\x03\x11\x08\x10\n\x0b\n\x04\x04\0\x02\0\
    \x12\x03\x12\x04\x1b\n\r\n\x05\x04\0\x02\0\x04\x12\x04\x12\x04\x11\x12\n\
    \x0c\n\x05\x04\0\x02\0\x05\x12\x03\x12\x04\t\n\x0c\n\x05\x04\0\x02\0\x01\
    \x12\x03\x12\n\x16\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x12\x19\x1a\n\x0b\
    \n\x04\x04\0\x02\x01\x12\x03\x13\x04\x1c\n\r\n\x05\x04\0\x02\x01\x04\x12\
    \x04\x13\x04\x12\x1b\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x13\x04\n\n\
    \x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x13\x0b\x17\n\x0c\n\x05\x04\0\x02\
    \x01\x03\x12\x03\x13\x1a\x1b\n\x0b\n\x04\x04\0\x02\x02\x12\x03\x14\x04\
    \x12\n\r\n\x05\x04\0\x02\x02\x04\x12\x04\x14\x04\x13\x1c\n\x0c\n\x05\x04\
    \0\x02\x02\x05\x12\x03\x14\x04\t\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\
    \x14\n\r\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\x14\x10\x11\n\x0b\n\x04\
    \x04\0\x02\x03\x12\x03\x15\x04\x18\n\r\n\x05\x04\0\x02\x03\x04\x12\x04\
    \x15\x04\x14\x12\n\x0c\n\x05\x04\0\x02\x03\x05\x12\x03\x15\x04\n\n\x0c\n\
    \x05\x04\0\x02\x03\x01\x12\x03\x15\x0b\x13\n\x0c\n\x05\x04\0\x02\x03\x03\
    \x12\x03\x15\x16\x17\n\n\n\x02\x04\x01\x12\x04\x18\0\x1c\x01\n\n\n\x03\
    \x04\x01\x01\x12\x03\x18\x08\x10\nD\n\x04\x04\x01\x02\0\x12\x03\x19\x04\
    \x18\"7\x20Client\x20should\x20backoff\x20or\x20cleanup\x20the\x20lock\
    \x20then\x20retry.\n\n\r\n\x05\x04\x01\x02\0\x04\x12\x04\x19\x04\x18\x12\
    \n\x0c\n\x05\x04\x01\x02\0\x06\x12\x03\x19\x04\x0c\n\x0c\n\x05\x04\x01\
    \x02\0\x01\x12\x03\x19\r\x13\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x19\
    \x16\x17\n>\n\x04\x04\x01\x02\x01\x12\x03\x1a\x04\x19\"1\x20Client\x20ma\
    y\x20restart\x20the\x20txn.\x20e.g\x20write\x20conflict.\n\n\r\n\x05\x04\
    \x01\x02\x01\x04\x12\x04\x1a\x04\x19\x18\n\x0c\n\x05\x04\x01\x02\x01\x05\
    \x12\x03\x1a\x04\n\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03\x1a\x0b\x14\n\
    \x0c\n\x05\x04\x01\x02\x01\x03\x12\x03\x1a\x17\x18\n+\n\x04\x04\x01\x02\
    \x02\x12\x03\x1b\x04\x15\"\x1e\x20Client\x20should\x20abort\x20the\x20tx\
    n.\n\n\r\n\x05\x04\x01\x02\x02\x04\x12\x04\x1b\x04\x1a\x19\n\x0c\n\x05\
    \x04\x01\x02\x02\x05\x12\x03\x1b\x04\n\n\x0c\n\x05\x04\x01\x02\x02\x01\
    \x12\x03\x1b\x0b\x10\n\x0c\n\x05\x04\x01\x02\x02\x03\x12\x03\x1b\x13\x14\
    \n\n\n\x02\x04\x02\x12\x04\x1e\0$\x01\n\n\n\x03\x04\x02\x01\x12\x03\x1e\
    \x08\x0f\n\x0b\n\x04\x04\x02\x02\0\x12\x03\x1f\x04\x19\n\r\n\x05\x04\x02\
    \x02\0\x04\x12\x04\x1f\x04\x1e\x11\n\x0c\n\x05\x04\x02\x02\0\x05\x12\x03\
    \x1f\x04\n\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03\x1f\x0b\x14\n\x0c\n\x05\
    \x04\x02\x02\0\x03\x12\x03\x1f\x17\x18\n\x0b\n\x04\x04\x02\x02\x01\x12\
    \x03\x20\x04(\n\r\n\x05\x04\x02\x02\x01\x04\x12\x04\x20\x04\x1f\x19\n\
    \x0c\n\x05\x04\x02\x02\x01\x06\x12\x03\x20\x04\x16\n\x0c\n\x05\x04\x02\
    \x02\x01\x01\x12\x03\x20\x17#\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\x03\
    \x20&'\n\x0b\n\x04\x04\x02\x02\x02\x12\x03!\x04\x19\n\r\n\x05\x04\x02\
    \x02\x02\x04\x12\x04!\x04\x20(\n\x0c\n\x05\x04\x02\x02\x02\x06\x12\x03!\
    \x04\x0f\n\x0c\n\x05\x04\x02\x02\x02\x01\x12\x03!\x10\x14\n\x0c\n\x05\
    \x04\x02\x02\x02\x03\x12\x03!\x17\x18\n\x0b\n\x04\x04\x02\x02\x03\x12\
    \x03\"\x04\x1a\n\r\n\x05\x04\x02\x02\x03\x04\x12\x04\"\x04!\x19\n\x0c\n\
    \x05\x04\x02\x02\x03\x05\x12\x03\"\x04\x08\n\x0c\n\x05\x04\x02\x02\x03\
    \x01\x12\x03\"\t\x14\n\x0c\n\x05\x04\x02\x02\x03\x03\x12\x03\"\x18\x19\n\
    \x0b\n\x04\x04\x02\x02\x04\x12\x03#\x04\x14\n\r\n\x05\x04\x02\x02\x04\
    \x04\x12\x04#\x04\"\x1a\n\x0c\n\x05\x04\x02\x02\x04\x05\x12\x03#\x04\n\n\
    \x0c\n\x05\x04\x02\x02\x04\x01\x12\x03#\x0b\x0f\n\x0c\n\x05\x04\x02\x02\
    \x04\x03\x12\x03#\x12\x13\n\n\n\x02\x04\x03\x12\x04&\0*\x01\n\n\n\x03\
    \x04\x03\x01\x12\x03&\x08\x12\n\x0b\n\x04\x04\x03\x02\0\x12\x03'\x04\x18\
    \n\r\n\x05\x04\x03\x02\0\x04\x12\x04'\x04&\x14\n\x0c\n\x05\x04\x03\x02\0\
    \x06\x12\x03'\x04\x0b\n\x0c\n\x05\x04\x03\x02\0\x01\x12\x03'\x0c\x13\n\
    \x0c\n\x05\x04\x03\x02\0\x03\x12\x03'\x16\x17\n\x0b\n\x04\x04\x03\x02\
    \x01\x12\x03(\x04\x12\n\r\n\x05\x04\x03\x02\x01\x04\x12\x04(\x04'\x18\n\
    \x0c\n\x05\x04\x03\x02\x01\x05\x12\x03(\x04\t\n\x0c\n\x05\x04\x03\x02\
    \x01\x01\x12\x03(\n\r\n\x0c\n\x05\x04\x03\x02\x01\x03\x12\x03(\x10\x11\n\
    \x0b\n\x04\x04\x03\x02\x02\x12\x03)\x04\x17\n\r\n\x05\x04\x03\x02\x02\
    \x04\x12\x04)\x04(\x12\n\x0c\n\x05\x04\x03\x02\x02\x05\x12\x03)\x04\n\n\
    \x0c\n\x05\x04\x03\x02\x02\x01\x12\x03)\x0b\x12\n\x0c\n\x05\x04\x03\x02\
    \x02\x03\x12\x03)\x15\x16\n\n\n\x02\x04\x04\x12\x04,\00\x01\n\n\n\x03\
    \x04\x04\x01\x12\x03,\x08\x13\n\x0b\n\x04\x04\x04\x02\0\x12\x03-\x04#\n\
    \r\n\x05\x04\x04\x02\0\x04\x12\x04-\x04,\x15\n\x0c\n\x05\x04\x04\x02\0\
    \x06\x12\x03-\x04\x11\n\x0c\n\x05\x04\x04\x02\0\x01\x12\x03-\x12\x1e\n\
    \x0c\n\x05\x04\x04\x02\0\x03\x12\x03-!\"\n\x0b\n\x04\x04\x04\x02\x01\x12\
    \x03.\x04\x17\n\r\n\x05\x04\x04\x02\x01\x04\x12\x04.\x04-#\n\x0c\n\x05\
    \x04\x04\x02\x01\x06\x12\x03.\x04\x0c\n\x0c\n\x05\x04\x04\x02\x01\x01\
    \x12\x03.\r\x12\n\x0c\n\x05\x04\x04\x02\x01\x03\x12\x03.\x15\x16\n\x0b\n\
    \x04\x04\x04\x02\x02\x12\x03/\x04\x14\n\r\n\x05\x04\x04\x02\x02\x04\x12\
    \x04/\x04.\x17\n\x0c\n\x05\x04\x04\x02\x02\x05\x12\x03/\x04\t\n\x0c\n\
    \x05\x04\x04\x02\x02\x01\x12\x03/\n\x0f\n\x0c\n\x05\x04\x04\x02\x02\x03\
    \x12\x03/\x12\x13\n\n\n\x02\x04\x05\x12\x042\08\x01\n\n\n\x03\x04\x05\
    \x01\x12\x032\x08\x13\n\x0b\n\x04\x04\x05\x02\0\x12\x033\x04\x18\n\r\n\
    \x05\x04\x05\x02\0\x04\x12\x043\x042\x15\n\x0c\n\x05\x04\x05\x02\0\x06\
    \x12\x033\x04\x0b\n\x0c\n\x05\x04\x05\x02\0\x01\x12\x033\x0c\x13\n\x0c\n\
    \x05\x04\x05\x02\0\x03\x12\x033\x16\x17\n\x0b\n\x04\x04\x05\x02\x01\x12\
    \x034\x04\x18\n\r\n\x05\x04\x05\x02\x01\x04\x12\x044\x043\x18\n\x0c\n\
    \x05\x04\x05\x02\x01\x05\x12\x034\x04\t\n\x0c\n\x05\x04\x05\x02\x01\x01\
    \x12\x034\n\x13\n\x0c\n\x05\x04\x05\x02\x01\x03\x12\x034\x16\x17\n\x0b\n\
    \x04\x04\x05\x02\x02\x12\x035\x04\x15\n\r\n\x05\x04\x05\x02\x02\x04\x12\
    \x045\x044\x18\n\x0c\n\x05\x04\x05\x02\x02\x05\x12\x035\x04\n\n\x0c\n\
    \x05\x04\x05\x02\x02\x01\x12\x035\x0b\x10\n\x0c\n\x05\x04\x05\x02\x02\
    \x03\x12\x035\x13\x14\n\x0b\n\x04\x04\x05\x02\x03\x12\x036\x04\x17\n\r\n\
    \x05\x04\x05\x02\x03\x04\x12\x046\x045\x15\n\x0c\n\x05\x04\x05\x02\x03\
    \x05\x12\x036\x04\n\n\x0c\n\x05\x04\x05\x02\x03\x01\x12\x036\x0b\x12\n\
    \x0c\n\x05\x04\x05\x02\x03\x03\x12\x036\x15\x16\n\x0b\n\x04\x04\x05\x02\
    \x04\x12\x037\x04\x16\n\r\n\x05\x04\x05\x02\x04\x04\x12\x047\x046\x17\n\
    \x0c\n\x05\x04\x05\x02\x04\x05\x12\x037\x04\x08\n\x0c\n\x05\x04\x05\x02\
    \x04\x01\x12\x037\t\x11\n\x0c\n\x05\x04\x05\x02\x04\x03\x12\x037\x14\x15\
    \n\n\n\x02\x04\x06\x12\x04:\0>\x01\n\n\n\x03\x04\x06\x01\x12\x03:\x08\
    \x0e\n\x0b\n\x04\x04\x06\x02\0\x12\x03;\x04\x17\n\r\n\x05\x04\x06\x02\0\
    \x04\x12\x04;\x04:\x10\n\x0c\n\x05\x04\x06\x02\0\x06\x12\x03;\x04\x0c\n\
    \x0c\n\x05\x04\x06\x02\0\x01\x12\x03;\r\x12\n\x0c\n\x05\x04\x06\x02\0\
    \x03\x12\x03;\x15\x16\n\x0b\n\x04\x04\x06\x02\x01\x12\x03<\x04\x12\n\r\n\
    \x05\x04\x06\x02\x01\x04\x12\x04<\x04;\x17\n\x0c\n\x05\x04\x06\x02\x01\
    \x05\x12\x03<\x04\t\n\x0c\n\x05\x04\x06\x02\x01\x01\x12\x03<\n\r\n\x0c\n\
    \x05\x04\x06\x02\x01\x03\x12\x03<\x10\x11\n\x0b\n\x04\x04\x06\x02\x02\
    \x12\x03=\x04\x14\n\r\n\x05\x04\x06\x02\x02\x04\x12\x04=\x04<\x12\n\x0c\
    \n\x05\x04\x06\x02\x02\x05\x12\x03=\x04\t\n\x0c\n\x05\x04\x06\x02\x02\
    \x01\x12\x03=\n\x0f\n\x0c\n\x05\x04\x06\x02\x02\x03\x12\x03=\x12\x13\n\n\
    \n\x02\x04\x07\x12\x04@\0C\x01\n\n\n\x03\x04\x07\x01\x12\x03@\x08\x14\n\
    \x0b\n\x04\x04\x07\x02\0\x12\x03A\x04#\n\r\n\x05\x04\x07\x02\0\x04\x12\
    \x04A\x04@\x16\n\x0c\n\x05\x04\x07\x02\0\x06\x12\x03A\x04\x11\n\x0c\n\
    \x05\x04\x07\x02\0\x01\x12\x03A\x12\x1e\n\x0c\n\x05\x04\x07\x02\0\x03\
    \x12\x03A!\"\n\x0b\n\x04\x04\x07\x02\x01\x12\x03B\x04\x1e\n\x0c\n\x05\
    \x04\x07\x02\x01\x04\x12\x03B\x04\x0c\n\x0c\n\x05\x04\x07\x02\x01\x06\
    \x12\x03B\r\x13\n\x0c\n\x05\x04\x07\x02\x01\x01\x12\x03B\x14\x19\n\x0c\n\
    \x05\x04\x07\x02\x01\x03\x12\x03B\x1c\x1d\n\n\n\x02\x05\0\x12\x04E\0I\
    \x01\n\n\n\x03\x05\0\x01\x12\x03E\x05\x07\n\x0b\n\x04\x05\0\x02\0\x12\
    \x03F\x04\x0c\n\x0c\n\x05\x05\0\x02\0\x01\x12\x03F\x04\x07\n\x0c\n\x05\
    \x05\0\x02\0\x02\x12\x03F\n\x0b\n\x0b\n\x04\x05\0\x02\x01\x12\x03G\x04\
    \x0c\n\x0c\n\x05\x05\0\x02\x01\x01\x12\x03G\x04\x07\n\x0c\n\x05\x05\0\
    \x02\x01\x02\x12\x03G\n\x0b\n\x0b\n\x04\x05\0\x02\x02\x12\x03H\x04\r\n\
    \x0c\n\x05\x05\0\x02\x02\x01\x12\x03H\x04\x08\n\x0c\n\x05\x05\0\x02\x02\
    \x02\x12\x03H\x0b\x0c\n\n\n\x02\x04\x08\x12\x04K\0O\x01\n\n\n\x03\x04\
    \x08\x01\x12\x03K\x08\x10\n\x0b\n\x04\x04\x08\x02\0\x12\x03L\x04\x0e\n\r\
    \n\x05\x04\x08\x02\0\x04\x12\x04L\x04K\x12\n\x0c\n\x05\x04\x08\x02\0\x06\
    \x12\x03L\x04\x06\n\x0c\n\x05\x04\x08\x02\0\x01\x12\x03L\x07\t\n\x0c\n\
    \x05\x04\x08\x02\0\x03\x12\x03L\x0c\r\n\x0b\n\x04\x04\x08\x02\x01\x12\
    \x03M\x04\x12\n\r\n\x05\x04\x08\x02\x01\x04\x12\x04M\x04L\x0e\n\x0c\n\
    \x05\x04\x08\x02\x01\x05\x12\x03M\x04\t\n\x0c\n\x05\x04\x08\x02\x01\x01\
    \x12\x03M\n\r\n\x0c\n\x05\x04\x08\x02\x01\x03\x12\x03M\x10\x11\n\x0b\n\
    \x04\x04\x08\x02\x02\x12\x03N\x04\x14\n\r\n\x05\x04\x08\x02\x02\x04\x12\
    \x04N\x04M\x12\n\x0c\n\x05\x04\x08\x02\x02\x05\x12\x03N\x04\t\n\x0c\n\
    \x05\x04\x08\x02\x02\x01\x12\x03N\n\x0f\n\x0c\n\x05\x04\x08\x02\x02\x03\
    \x12\x03N\x12\x13\n\n\n\x02\x04\t\x12\x04Q\0Y\x01\n\n\n\x03\x04\t\x01\
    \x12\x03Q\x08\x17\n\x0b\n\x04\x04\t\x02\0\x12\x03R\x04\x18\n\r\n\x05\x04\
    \t\x02\0\x04\x12\x04R\x04Q\x19\n\x0c\n\x05\x04\t\x02\0\x06\x12\x03R\x04\
    \x0b\n\x0c\n\x05\x04\t\x02\0\x01\x12\x03R\x0c\x13\n\x0c\n\x05\x04\t\x02\
    \0\x03\x12\x03R\x16\x17\n\x0b\n\x04\x04\t\x02\x01\x12\x03S\x04$\n\x0c\n\
    \x05\x04\t\x02\x01\x04\x12\x03S\x04\x0c\n\x0c\n\x05\x04\t\x02\x01\x06\
    \x12\x03S\r\x15\n\x0c\n\x05\x04\t\x02\x01\x01\x12\x03S\x16\x1f\n\x0c\n\
    \x05\x04\t\x02\x01\x03\x12\x03S\"#\n\x1f\n\x04\x04\t\x02\x02\x12\x03U\
    \x04\x1b\x1a\x12\x20primary_lock_key\n\n\r\n\x05\x04\t\x02\x02\x04\x12\
    \x04U\x04S$\n\x0c\n\x05\x04\t\x02\x02\x05\x12\x03U\x04\t\n\x0c\n\x05\x04\
    \t\x02\x02\x01\x12\x03U\n\x16\n\x0c\n\x05\x04\t\x02\x02\x03\x12\x03U\x19\
    \x1a\n\x0b\n\x04\x04\t\x02\x03\x12\x03V\x04\x1d\n\r\n\x05\x04\t\x02\x03\
    \x04\x12\x04V\x04U\x1b\n\x0c\n\x05\x04\t\x02\x03\x05\x12\x03V\x04\n\n\
    \x0c\n\x05\x04\t\x02\x03\x01\x12\x03V\x0b\x18\n\x0c\n\x05\x04\t\x02\x03\
    \x03\x12\x03V\x1b\x1c\n\x0b\n\x04\x04\t\x02\x04\x12\x03W\x04\x18\n\r\n\
    \x05\x04\t\x02\x04\x04\x12\x04W\x04V\x1d\n\x0c\n\x05\x04\t\x02\x04\x05\
    \x12\x03W\x04\n\n\x0c\n\x05\x04\t\x02\x04\x01\x12\x03W\x0b\x13\n\x0c\n\
    \x05\x04\t\x02\x04\x03\x12\x03W\x16\x17\n\x0b\n\x04\x04\t\x02\x05\x12\
    \x03X\x04#\n\r\n\x05\x04\t\x02\x05\x04\x12\x04X\x04W\x18\n\x0c\n\x05\x04\
    \t\x02\x05\x05\x12\x03X\x04\x08\n\x0c\n\x05\x04\t\x02\x05\x01\x12\x03X\t\
    \x1e\n\x0c\n\x05\x04\t\x02\x05\x03\x12\x03X!\"\n\n\n\x02\x04\n\x12\x04[\
    \0^\x01\n\n\n\x03\x04\n\x01\x12\x03[\x08\x18\n\x0b\n\x04\x04\n\x02\0\x12\
    \x03\\\x04#\n\r\n\x05\x04\n\x02\0\x04\x12\x04\\\x04[\x1a\n\x0c\n\x05\x04\
    \n\x02\0\x06\x12\x03\\\x04\x11\n\x0c\n\x05\x04\n\x02\0\x01\x12\x03\\\x12\
    \x1e\n\x0c\n\x05\x04\n\x02\0\x03\x12\x03\\!\"\n\x0b\n\x04\x04\n\x02\x01\
    \x12\x03]\x04!\n\x0c\n\x05\x04\n\x02\x01\x04\x12\x03]\x04\x0c\n\x0c\n\
    \x05\x04\n\x02\x01\x06\x12\x03]\r\x15\n\x0c\n\x05\x04\n\x02\x01\x01\x12\
    \x03]\x16\x1c\n\x0c\n\x05\x04\n\x02\x01\x03\x12\x03]\x1f\x20\n\n\n\x02\
    \x04\x0b\x12\x04`\0f\x01\n\n\n\x03\x04\x0b\x01\x12\x03`\x08\x15\n\x0b\n\
    \x04\x04\x0b\x02\0\x12\x03a\x04\x18\n\r\n\x05\x04\x0b\x02\0\x04\x12\x04a\
    \x04`\x17\n\x0c\n\x05\x04\x0b\x02\0\x06\x12\x03a\x04\x0b\n\x0c\n\x05\x04\
    \x0b\x02\0\x01\x12\x03a\x0c\x13\n\x0c\n\x05\x04\x0b\x02\0\x03\x12\x03a\
    \x16\x17\n\x0b\n\x04\x04\x0b\x02\x01\x12\x03b\x04\x1d\n\r\n\x05\x04\x0b\
    \x02\x01\x04\x12\x04b\x04a\x18\n\x0c\n\x05\x04\x0b\x02\x01\x05\x12\x03b\
    \x04\n\n\x0c\n\x05\x04\x0b\x02\x01\x01\x12\x03b\x0b\x18\n\x0c\n\x05\x04\
    \x0b\x02\x01\x03\x12\x03b\x1b\x1c\n\x0b\n\x04\x04\x0b\x02\x02\x12\x03c\
    \x04\x1c\n\x0c\n\x05\x04\x0b\x02\x02\x04\x12\x03c\x04\x0c\n\x0c\n\x05\
    \x04\x0b\x02\x02\x05\x12\x03c\r\x12\n\x0c\n\x05\x04\x0b\x02\x02\x01\x12\
    \x03c\x13\x17\n\x0c\n\x05\x04\x0b\x02\x02\x03\x12\x03c\x1a\x1b\n\x0b\n\
    \x04\x04\x0b\x02\x03\x12\x03d\x04\x1e\n\r\n\x05\x04\x0b\x02\x03\x04\x12\
    \x04d\x04c\x1c\n\x0c\n\x05\x04\x0b\x02\x03\x05\x12\x03d\x04\n\n\x0c\n\
    \x05\x04\x0b\x02\x03\x01\x12\x03d\x0b\x19\n\x0c\n\x05\x04\x0b\x02\x03\
    \x03\x12\x03d\x1c\x1d\n\x0b\n\x04\x04\x0b\x02\x04\x12\x03e\x04\x16\n\r\n\
    \x05\x04\x0b\x02\x04\x04\x12\x04e\x04d\x1e\n\x0c\n\x05\x04\x0b\x02\x04\
    \x05\x12\x03e\x04\t\n\x0c\n\x05\x04\x0b\x02\x04\x01\x12\x03e\x0b\x11\n\
    \x0c\n\x05\x04\x0b\x02\x04\x03\x12\x03e\x14\x15\n\n\n\x02\x04\x0c\x12\
    \x04h\0k\x01\n\n\n\x03\x04\x0c\x01\x12\x03h\x08\x16\n\x0b\n\x04\x04\x0c\
    \x02\0\x12\x03i\x04#\n\r\n\x05\x04\x0c\x02\0\x04\x12\x04i\x04h\x18\n\x0c\
    \n\x05\x04\x0c\x02\0\x06\x12\x03i\x04\x11\n\x0c\n\x05\x04\x0c\x02\0\x01\
    \x12\x03i\x12\x1e\n\x0c\n\x05\x04\x0c\x02\0\x03\x12\x03i!\"\n\x0b\n\x04\
    \x04\x0c\x02\x01\x12\x03j\x04\x17\n\r\n\x05\x04\x0c\x02\x01\x04\x12\x04j\
    \x04i#\n\x0c\n\x05\x04\x0c\x02\x01\x06\x12\x03j\x04\x0c\n\x0c\n\x05\x04\
    \x0c\x02\x01\x01\x12\x03j\r\x12\n\x0c\n\x05\x04\x0c\x02\x01\x03\x12\x03j\
    \x15\x16\n\n\n\x02\x04\r\x12\x04m\0p\x01\n\n\n\x03\x04\r\x01\x12\x03m\
    \x08\x15\n\x0b\n\x04\x04\r\x02\0\x12\x03n\x04$\n\x0c\n\x05\x04\r\x02\0\
    \x04\x12\x03n\x04\x0c\n\x0c\n\x05\x04\r\x02\0\x06\x12\x03n\r\x15\n\x0c\n\
    \x05\x04\r\x02\0\x01\x12\x03n\x16\x1f\n\x0c\n\x05\x04\r\x02\0\x03\x12\
    \x03n\"#\n\x0b\n\x04\x04\r\x02\x01\x12\x03o\x04\x1e\n\r\n\x05\x04\r\x02\
    \x01\x04\x12\x04o\x04n$\n\x0c\n\x05\x04\r\x02\x01\x05\x12\x03o\x04\n\n\
    \x0c\n\x05\x04\r\x02\x01\x01\x12\x03o\x0b\x19\n\x0c\n\x05\x04\r\x02\x01\
    \x03\x12\x03o\x1c\x1d\n\n\n\x02\x04\x0e\x12\x04r\0u\x01\n\n\n\x03\x04\
    \x0e\x01\x12\x03r\x08\x16\n\x0b\n\x04\x04\x0e\x02\0\x12\x03s\x04#\n\r\n\
    \x05\x04\x0e\x02\0\x04\x12\x04s\x04r\x18\n\x0c\n\x05\x04\x0e\x02\0\x06\
    \x12\x03s\x04\x11\n\x0c\n\x05\x04\x0e\x02\0\x01\x12\x03s\x12\x1e\n\x0c\n\
    \x05\x04\x0e\x02\0\x03\x12\x03s!\"\n\x0b\n\x04\x04\x0e\x02\x01\x12\x03t\
    \x04\x15\n\r\n\x05\x04\x0e\x02\x01\x04\x12\x04t\x04s#\n\x0c\n\x05\x04\
    \x0e\x02\x01\x05\x12\x03t\x04\n\n\x0c\n\x05\x04\x0e\x02\x01\x01\x12\x03t\
    \x0b\x10\n\x0c\n\x05\x04\x0e\x02\x01\x03\x12\x03t\x13\x14\n\n\n\x02\x04\
    \x0f\x12\x04w\0{\x01\n\n\n\x03\x04\x0f\x01\x12\x03w\x08\x1c\n\x0b\n\x04\
    \x04\x0f\x02\0\x12\x03x\x04\x18\n\r\n\x05\x04\x0f\x02\0\x04\x12\x04x\x04\
    w\x1e\n\x0c\n\x05\x04\x0f\x02\0\x06\x12\x03x\x04\x0b\n\x0c\n\x05\x04\x0f\
    \x02\0\x01\x12\x03x\x0c\x13\n\x0c\n\x05\x04\x0f\x02\0\x03\x12\x03x\x16\
    \x17\n\x0b\n\x04\x04\x0f\x02\x01\x12\x03y\x04\x1d\n\r\n\x05\x04\x0f\x02\
    \x01\x04\x12\x04y\x04x\x18\n\x0c\n\x05\x04\x0f\x02\x01\x05\x12\x03y\x04\
    \n\n\x0c\n\x05\x04\x0f\x02\x01\x01\x12\x03y\x0b\x18\n\x0c\n\x05\x04\x0f\
    \x02\x01\x03\x12\x03y\x1b\x1c\n\x0b\n\x04\x04\x0f\x02\x02\x12\x03z\x04\
    \x1c\n\x0c\n\x05\x04\x0f\x02\x02\x04\x12\x03z\x04\x0c\n\x0c\n\x05\x04\
    \x0f\x02\x02\x05\x12\x03z\r\x12\n\x0c\n\x05\x04\x0f\x02\x02\x01\x12\x03z\
    \x13\x17\n\x0c\n\x05\x04\x0f\x02\x02\x03\x12\x03z\x1a\x1b\n\x0b\n\x02\
    \x04\x10\x12\x05}\0\x80\x01\x01\n\n\n\x03\x04\x10\x01\x12\x03}\x08\x1d\n\
    \x0b\n\x04\x04\x10\x02\0\x12\x03~\x04#\n\r\n\x05\x04\x10\x02\0\x04\x12\
    \x04~\x04}\x1f\n\x0c\n\x05\x04\x10\x02\0\x06\x12\x03~\x04\x11\n\x0c\n\
    \x05\x04\x10\x02\0\x01\x12\x03~\x12\x1e\n\x0c\n\x05\x04\x10\x02\0\x03\
    \x12\x03~!\"\n\x0b\n\x04\x04\x10\x02\x01\x12\x03\x7f\x04\x17\n\r\n\x05\
    \x04\x10\x02\x01\x04\x12\x04\x7f\x04~#\n\x0c\n\x05\x04\x10\x02\x01\x06\
    \x12\x03\x7f\x04\x0c\n\x0c\n\x05\x04\x10\x02\x01\x01\x12\x03\x7f\r\x12\n\
    \x0c\n\x05\x04\x10\x02\x01\x03\x12\x03\x7f\x15\x16\n\x0c\n\x02\x04\x11\
    \x12\x06\x82\x01\0\x86\x01\x01\n\x0b\n\x03\x04\x11\x01\x12\x04\x82\x01\
    \x08\x16\n\x0c\n\x04\x04\x11\x02\0\x12\x04\x83\x01\x04\x18\n\x0f\n\x05\
    \x04\x11\x02\0\x04\x12\x06\x83\x01\x04\x82\x01\x18\n\r\n\x05\x04\x11\x02\
    \0\x06\x12\x04\x83\x01\x04\x0b\n\r\n\x05\x04\x11\x02\0\x01\x12\x04\x83\
    \x01\x0c\x13\n\r\n\x05\x04\x11\x02\0\x03\x12\x04\x83\x01\x16\x17\n\x0c\n\
    \x04\x04\x11\x02\x01\x12\x04\x84\x01\x04\x12\n\x0f\n\x05\x04\x11\x02\x01\
    \x04\x12\x06\x84\x01\x04\x83\x01\x18\n\r\n\x05\x04\x11\x02\x01\x05\x12\
    \x04\x84\x01\x04\t\n\r\n\x05\x04\x11\x02\x01\x01\x12\x04\x84\x01\n\r\n\r\
    \n\x05\x04\x11\x02\x01\x03\x12\x04\x84\x01\x10\x11\n\x0c\n\x04\x04\x11\
    \x02\x02\x12\x04\x85\x01\x04\x1d\n\x0f\n\x05\x04\x11\x02\x02\x04\x12\x06\
    \x85\x01\x04\x84\x01\x12\n\r\n\x05\x04\x11\x02\x02\x05\x12\x04\x85\x01\
    \x04\n\n\r\n\x05\x04\x11\x02\x02\x01\x12\x04\x85\x01\x0b\x18\n\r\n\x05\
    \x04\x11\x02\x02\x03\x12\x04\x85\x01\x1b\x1c\n\x0c\n\x02\x04\x12\x12\x06\
    \x88\x01\0\x8c\x01\x01\n\x0b\n\x03\x04\x12\x01\x12\x04\x88\x01\x08\x17\n\
    \x0c\n\x04\x04\x12\x02\0\x12\x04\x89\x01\x04#\n\x0f\n\x05\x04\x12\x02\0\
    \x04\x12\x06\x89\x01\x04\x88\x01\x19\n\r\n\x05\x04\x12\x02\0\x06\x12\x04\
    \x89\x01\x04\x11\n\r\n\x05\x04\x12\x02\0\x01\x12\x04\x89\x01\x12\x1e\n\r\
    \n\x05\x04\x12\x02\0\x03\x12\x04\x89\x01!\"\n\x0c\n\x04\x04\x12\x02\x01\
    \x12\x04\x8a\x01\x04\x17\n\x0f\n\x05\x04\x12\x02\x01\x04\x12\x06\x8a\x01\
    \x04\x89\x01#\n\r\n\x05\x04\x12\x02\x01\x06\x12\x04\x8a\x01\x04\x0c\n\r\
    \n\x05\x04\x12\x02\x01\x01\x12\x04\x8a\x01\r\x12\n\r\n\x05\x04\x12\x02\
    \x01\x03\x12\x04\x8a\x01\x15\x16\n8\n\x04\x04\x12\x02\x02\x12\x04\x8b\
    \x01\x04\x1e\"*\x20set\x20this\x20if\x20the\x20key\x20is\x20already\x20c\
    ommitted\n\n\x0f\n\x05\x04\x12\x02\x02\x04\x12\x06\x8b\x01\x04\x8a\x01\
    \x17\n\r\n\x05\x04\x12\x02\x02\x05\x12\x04\x8b\x01\x04\n\n\r\n\x05\x04\
    \x12\x02\x02\x01\x12\x04\x8b\x01\x0b\x19\n\r\n\x05\x04\x12\x02\x02\x03\
    \x12\x04\x8b\x01\x1c\x1d\n\x0c\n\x02\x04\x13\x12\x06\x8e\x01\0\x92\x01\
    \x01\n\x0b\n\x03\x04\x13\x01\x12\x04\x8e\x01\x08\x17\n\x0c\n\x04\x04\x13\
    \x02\0\x12\x04\x8f\x01\x04\x18\n\x0f\n\x05\x04\x13\x02\0\x04\x12\x06\x8f\
    \x01\x04\x8e\x01\x19\n\r\n\x05\x04\x13\x02\0\x06\x12\x04\x8f\x01\x04\x0b\
    \n\r\n\x05\x04\x13\x02\0\x01\x12\x04\x8f\x01\x0c\x13\n\r\n\x05\x04\x13\
    \x02\0\x03\x12\x04\x8f\x01\x16\x17\n\x0c\n\x04\x04\x13\x02\x01\x12\x04\
    \x90\x01\x04\x1c\n\r\n\x05\x04\x13\x02\x01\x04\x12\x04\x90\x01\x04\x0c\n\
    \r\n\x05\x04\x13\x02\x01\x05\x12\x04\x90\x01\r\x12\n\r\n\x05\x04\x13\x02\
    \x01\x01\x12\x04\x90\x01\x13\x17\n\r\n\x05\x04\x13\x02\x01\x03\x12\x04\
    \x90\x01\x1a\x1b\n\x0c\n\x04\x04\x13\x02\x02\x12\x04\x91\x01\x04\x17\n\
    \x0f\n\x05\x04\x13\x02\x02\x04\x12\x06\x91\x01\x04\x90\x01\x1c\n\r\n\x05\
    \x04\x13\x02\x02\x05\x12\x04\x91\x01\x04\n\n\r\n\x05\x04\x13\x02\x02\x01\
    \x12\x04\x91\x01\x0b\x12\n\r\n\x05\x04\x13\x02\x02\x03\x12\x04\x91\x01\
    \x15\x16\n\x0c\n\x02\x04\x14\x12\x06\x94\x01\0\x97\x01\x01\n\x0b\n\x03\
    \x04\x14\x01\x12\x04\x94\x01\x08\x18\n\x0c\n\x04\x04\x14\x02\0\x12\x04\
    \x95\x01\x04#\n\x0f\n\x05\x04\x14\x02\0\x04\x12\x06\x95\x01\x04\x94\x01\
    \x1a\n\r\n\x05\x04\x14\x02\0\x06\x12\x04\x95\x01\x04\x11\n\r\n\x05\x04\
    \x14\x02\0\x01\x12\x04\x95\x01\x12\x1e\n\r\n\x05\x04\x14\x02\0\x03\x12\
    \x04\x95\x01!\"\n\x0c\n\x04\x04\x14\x02\x01\x12\x04\x96\x01\x04\x1e\n\r\
    \n\x05\x04\x14\x02\x01\x04\x12\x04\x96\x01\x04\x0c\n\r\n\x05\x04\x14\x02\
    \x01\x06\x12\x04\x96\x01\r\x13\n\r\n\x05\x04\x14\x02\x01\x01\x12\x04\x96\
    \x01\x14\x19\n\r\n\x05\x04\x14\x02\x01\x03\x12\x04\x96\x01\x1c\x1d\n\x0c\
    \n\x02\x04\x15\x12\x06\x99\x01\0\x9c\x01\x01\n\x0b\n\x03\x04\x15\x01\x12\
    \x04\x99\x01\x08\x17\n\x0c\n\x04\x04\x15\x02\0\x12\x04\x9a\x01\x04\x18\n\
    \x0f\n\x05\x04\x15\x02\0\x04\x12\x06\x9a\x01\x04\x99\x01\x19\n\r\n\x05\
    \x04\x15\x02\0\x06\x12\x04\x9a\x01\x04\x0b\n\r\n\x05\x04\x15\x02\0\x01\
    \x12\x04\x9a\x01\x0c\x13\n\r\n\x05\x04\x15\x02\0\x03\x12\x04\x9a\x01\x16\
    \x17\n\x0c\n\x04\x04\x15\x02\x01\x12\x04\x9b\x01\x04\x1b\n\x0f\n\x05\x04\
    \x15\x02\x01\x04\x12\x06\x9b\x01\x04\x9a\x01\x18\n\r\n\x05\x04\x15\x02\
    \x01\x05\x12\x04\x9b\x01\x04\n\n\r\n\x05\x04\x15\x02\x01\x01\x12\x04\x9b\
    \x01\x0b\x16\n\r\n\x05\x04\x15\x02\x01\x03\x12\x04\x9b\x01\x19\x1a\n\x0c\
    \n\x02\x04\x16\x12\x06\x9e\x01\0\xa2\x01\x01\n\x0b\n\x03\x04\x16\x01\x12\
    \x04\x9e\x01\x08\x18\n\x0c\n\x04\x04\x16\x02\0\x12\x04\x9f\x01\x04#\n\
    \x0f\n\x05\x04\x16\x02\0\x04\x12\x06\x9f\x01\x04\x9e\x01\x1a\n\r\n\x05\
    \x04\x16\x02\0\x06\x12\x04\x9f\x01\x04\x11\n\r\n\x05\x04\x16\x02\0\x01\
    \x12\x04\x9f\x01\x12\x1e\n\r\n\x05\x04\x16\x02\0\x03\x12\x04\x9f\x01!\"\
    \n\x0c\n\x04\x04\x16\x02\x01\x12\x04\xa0\x01\x04\x17\n\x0f\n\x05\x04\x16\
    \x02\x01\x04\x12\x06\xa0\x01\x04\x9f\x01#\n\r\n\x05\x04\x16\x02\x01\x06\
    \x12\x04\xa0\x01\x04\x0c\n\r\n\x05\x04\x16\x02\x01\x01\x12\x04\xa0\x01\r\
    \x12\n\r\n\x05\x04\x16\x02\x01\x03\x12\x04\xa0\x01\x15\x16\n\x0c\n\x04\
    \x04\x16\x02\x02\x12\x04\xa1\x01\x04\x20\n\r\n\x05\x04\x16\x02\x02\x04\
    \x12\x04\xa1\x01\x04\x0c\n\r\n\x05\x04\x16\x02\x02\x06\x12\x04\xa1\x01\r\
    \x15\n\r\n\x05\x04\x16\x02\x02\x01\x12\x04\xa1\x01\x16\x1b\n\r\n\x05\x04\
    \x16\x02\x02\x03\x12\x04\xa1\x01\x1e\x1f\n\x0c\n\x02\x04\x17\x12\x06\xa4\
    \x01\0\xa9\x01\x01\n\x0b\n\x03\x04\x17\x01\x12\x04\xa4\x01\x08\x1a\n\x0c\
    \n\x04\x04\x17\x02\0\x12\x04\xa5\x01\x04\x18\n\x0f\n\x05\x04\x17\x02\0\
    \x04\x12\x06\xa5\x01\x04\xa4\x01\x1c\n\r\n\x05\x04\x17\x02\0\x06\x12\x04\
    \xa5\x01\x04\x0b\n\r\n\x05\x04\x17\x02\0\x01\x12\x04\xa5\x01\x0c\x13\n\r\
    \n\x05\x04\x17\x02\0\x03\x12\x04\xa5\x01\x16\x17\n\x0c\n\x04\x04\x17\x02\
    \x01\x12\x04\xa6\x01\x04\x1e\n\x0f\n\x05\x04\x17\x02\x01\x04\x12\x06\xa6\
    \x01\x04\xa5\x01\x18\n\r\n\x05\x04\x17\x02\x01\x05\x12\x04\xa6\x01\x04\n\
    \n\r\n\x05\x04\x17\x02\x01\x01\x12\x04\xa6\x01\x0b\x18\n\r\n\x05\x04\x17\
    \x02\x01\x03\x12\x04\xa6\x01\x1c\x1d\n9\n\x04\x04\x17\x02\x02\x12\x04\
    \xa8\x01\x04\x1e\x1a+\x20If\x20the\x20txn\x20is\x20rolled\x20back,\x20do\
    \x20not\x20set\x20it.\n\n\x0f\n\x05\x04\x17\x02\x02\x04\x12\x06\xa8\x01\
    \x04\xa6\x01\x1e\n\r\n\x05\x04\x17\x02\x02\x05\x12\x04\xa8\x01\x04\n\n\r\
    \n\x05\x04\x17\x02\x02\x01\x12\x04\xa8\x01\x0b\x19\n\r\n\x05\x04\x17\x02\
    \x02\x03\x12\x04\xa8\x01\x1c\x1d\n\x0c\n\x02\x04\x18\x12\x06\xab\x01\0\
    \xae\x01\x01\n\x0b\n\x03\x04\x18\x01\x12\x04\xab\x01\x08\x1b\n\x0c\n\x04\
    \x04\x18\x02\0\x12\x04\xac\x01\x04#\n\x0f\n\x05\x04\x18\x02\0\x04\x12\
    \x06\xac\x01\x04\xab\x01\x1d\n\r\n\x05\x04\x18\x02\0\x06\x12\x04\xac\x01\
    \x04\x11\n\r\n\x05\x04\x18\x02\0\x01\x12\x04\xac\x01\x12\x1e\n\r\n\x05\
    \x04\x18\x02\0\x03\x12\x04\xac\x01!\"\n\x0c\n\x04\x04\x18\x02\x01\x12\
    \x04\xad\x01\x04\x17\n\x0f\n\x05\x04\x18\x02\x01\x04\x12\x06\xad\x01\x04\
    \xac\x01#\n\r\n\x05\x04\x18\x02\x01\x06\x12\x04\xad\x01\x04\x0c\n\r\n\
    \x05\x04\x18\x02\x01\x01\x12\x04\xad\x01\r\x12\n\r\n\x05\x04\x18\x02\x01\
    \x03\x12\x04\xad\x01\x15\x16\n\x0c\n\x02\x04\x19\x12\x06\xb0\x01\0\xb3\
    \x01\x01\n\x0b\n\x03\x04\x19\x01\x12\x04\xb0\x01\x08\x11\n\x0c\n\x04\x04\
    \x19\x02\0\x12\x04\xb1\x01\x04\x18\n\x0f\n\x05\x04\x19\x02\0\x04\x12\x06\
    \xb1\x01\x04\xb0\x01\x13\n\r\n\x05\x04\x19\x02\0\x06\x12\x04\xb1\x01\x04\
    \x0b\n\r\n\x05\x04\x19\x02\0\x01\x12\x04\xb1\x01\x0c\x13\n\r\n\x05\x04\
    \x19\x02\0\x03\x12\x04\xb1\x01\x16\x17\n\x0c\n\x04\x04\x19\x02\x01\x12\
    \x04\xb2\x01\x04\x1a\n\x0f\n\x05\x04\x19\x02\x01\x04\x12\x06\xb2\x01\x04\
    \xb1\x01\x18\n\r\n\x05\x04\x19\x02\x01\x05\x12\x04\xb2\x01\x04\n\n\r\n\
    \x05\x04\x19\x02\x01\x01\x12\x04\xb2\x01\x0b\x15\n\r\n\x05\x04\x19\x02\
    \x01\x03\x12\x04\xb2\x01\x18\x19\n\x0c\n\x02\x04\x1a\x12\x06\xb5\x01\0\
    \xb8\x01\x01\n\x0b\n\x03\x04\x1a\x01\x12\x04\xb5\x01\x08\x12\n\x0c\n\x04\
    \x04\x1a\x02\0\x12\x04\xb6\x01\x04#\n\x0f\n\x05\x04\x1a\x02\0\x04\x12\
    \x06\xb6\x01\x04\xb5\x01\x14\n\r\n\x05\x04\x1a\x02\0\x06\x12\x04\xb6\x01\
    \x04\x11\n\r\n\x05\x04\x1a\x02\0\x01\x12\x04\xb6\x01\x12\x1e\n\r\n\x05\
    \x04\x1a\x02\0\x03\x12\x04\xb6\x01!\"\n\x0c\n\x04\x04\x1a\x02\x01\x12\
    \x04\xb7\x01\x04\x17\n\x0f\n\x05\x04\x1a\x02\x01\x04\x12\x06\xb7\x01\x04\
    \xb6\x01#\n\r\n\x05\x04\x1a\x02\x01\x06\x12\x04\xb7\x01\x04\x0c\n\r\n\
    \x05\x04\x1a\x02\x01\x01\x12\x04\xb7\x01\r\x12\n\r\n\x05\x04\x1a\x02\x01\
    \x03\x12\x04\xb7\x01\x15\x16\n\x0c\n\x02\x04\x1b\x12\x06\xba\x01\0\xbd\
    \x01\x01\n\x0b\n\x03\x04\x1b\x01\x12\x04\xba\x01\x08\x15\n\x0c\n\x04\x04\
    \x1b\x02\0\x12\x04\xbb\x01\x04\x18\n\x0f\n\x05\x04\x1b\x02\0\x04\x12\x06\
    \xbb\x01\x04\xba\x01\x17\n\r\n\x05\x04\x1b\x02\0\x06\x12\x04\xbb\x01\x04\
    \x0b\n\r\n\x05\x04\x1b\x02\0\x01\x12\x04\xbb\x01\x0c\x13\n\r\n\x05\x04\
    \x1b\x02\0\x03\x12\x04\xbb\x01\x16\x17\n\x0c\n\x04\x04\x1b\x02\x01\x12\
    \x04\xbc\x01\x04\x12\n\x0f\n\x05\x04\x1b\x02\x01\x04\x12\x06\xbc\x01\x04\
    \xbb\x01\x18\n\r\n\x05\x04\x1b\x02\x01\x05\x12\x04\xbc\x01\x04\t\n\r\n\
    \x05\x04\x1b\x02\x01\x01\x12\x04\xbc\x01\n\r\n\r\n\x05\x04\x1b\x02\x01\
    \x03\x12\x04\xbc\x01\x10\x11\n\x0c\n\x02\x04\x1c\x12\x06\xbf\x01\0\xc3\
    \x01\x01\n\x0b\n\x03\x04\x1c\x01\x12\x04\xbf\x01\x08\x16\n\x0c\n\x04\x04\
    \x1c\x02\0\x12\x04\xc0\x01\x04#\n\x0f\n\x05\x04\x1c\x02\0\x04\x12\x06\
    \xc0\x01\x04\xbf\x01\x18\n\r\n\x05\x04\x1c\x02\0\x06\x12\x04\xc0\x01\x04\
    \x11\n\r\n\x05\x04\x1c\x02\0\x01\x12\x04\xc0\x01\x12\x1e\n\r\n\x05\x04\
    \x1c\x02\0\x03\x12\x04\xc0\x01!\"\n\x0c\n\x04\x04\x1c\x02\x01\x12\x04\
    \xc1\x01\x04\x15\n\x0f\n\x05\x04\x1c\x02\x01\x04\x12\x06\xc1\x01\x04\xc0\
    \x01#\n\r\n\x05\x04\x1c\x02\x01\x05\x12\x04\xc1\x01\x04\n\n\r\n\x05\x04\
    \x1c\x02\x01\x01\x12\x04\xc1\x01\x0b\x10\n\r\n\x05\x04\x1c\x02\x01\x03\
    \x12\x04\xc1\x01\x13\x14\n\x0c\n\x04\x04\x1c\x02\x02\x12\x04\xc2\x01\x04\
    \x14\n\x0f\n\x05\x04\x1c\x02\x02\x04\x12\x06\xc2\x01\x04\xc1\x01\x15\n\r\
    \n\x05\x04\x1c\x02\x02\x05\x12\x04\xc2\x01\x04\t\n\r\n\x05\x04\x1c\x02\
    \x02\x01\x12\x04\xc2\x01\n\x0f\n\r\n\x05\x04\x1c\x02\x02\x03\x12\x04\xc2\
    \x01\x12\x13\n\x0c\n\x02\x04\x1d\x12\x06\xc5\x01\0\xc9\x01\x01\n\x0b\n\
    \x03\x04\x1d\x01\x12\x04\xc5\x01\x08\x15\n\x0c\n\x04\x04\x1d\x02\0\x12\
    \x04\xc6\x01\x04\x18\n\x0f\n\x05\x04\x1d\x02\0\x04\x12\x06\xc6\x01\x04\
    \xc5\x01\x17\n\r\n\x05\x04\x1d\x02\0\x06\x12\x04\xc6\x01\x04\x0b\n\r\n\
    \x05\x04\x1d\x02\0\x01\x12\x04\xc6\x01\x0c\x13\n\r\n\x05\x04\x1d\x02\0\
    \x03\x12\x04\xc6\x01\x16\x17\n\x0c\n\x04\x04\x1d\x02\x01\x12\x04\xc7\x01\
    \x04\x12\n\x0f\n\x05\x04\x1d\x02\x01\x04\x12\x06\xc7\x01\x04\xc6\x01\x18\
    \n\r\n\x05\x04\x1d\x02\x01\x05\x12\x04\xc7\x01\x04\t\n\r\n\x05\x04\x1d\
    \x02\x01\x01\x12\x04\xc7\x01\n\r\n\r\n\x05\x04\x1d\x02\x01\x03\x12\x04\
    \xc7\x01\x10\x11\n\x0c\n\x04\x04\x1d\x02\x02\x12\x04\xc8\x01\x04\x14\n\
    \x0f\n\x05\x04\x1d\x02\x02\x04\x12\x06\xc8\x01\x04\xc7\x01\x12\n\r\n\x05\
    \x04\x1d\x02\x02\x05\x12\x04\xc8\x01\x04\t\n\r\n\x05\x04\x1d\x02\x02\x01\
    \x12\x04\xc8\x01\n\x0f\n\r\n\x05\x04\x1d\x02\x02\x03\x12\x04\xc8\x01\x12\
    \x13\n\x0c\n\x02\x04\x1e\x12\x06\xcb\x01\0\xce\x01\x01\n\x0b\n\x03\x04\
    \x1e\x01\x12\x04\xcb\x01\x08\x16\n\x0c\n\x04\x04\x1e\x02\0\x12\x04\xcc\
    \x01\x04#\n\x0f\n\x05\x04\x1e\x02\0\x04\x12\x06\xcc\x01\x04\xcb\x01\x18\
    \n\r\n\x05\x04\x1e\x02\0\x06\x12\x04\xcc\x01\x04\x11\n\r\n\x05\x04\x1e\
    \x02\0\x01\x12\x04\xcc\x01\x12\x1e\n\r\n\x05\x04\x1e\x02\0\x03\x12\x04\
    \xcc\x01!\"\n\x0c\n\x04\x04\x1e\x02\x01\x12\x04\xcd\x01\x04\x15\n\x0f\n\
    \x05\x04\x1e\x02\x01\x04\x12\x06\xcd\x01\x04\xcc\x01#\n\r\n\x05\x04\x1e\
    \x02\x01\x05\x12\x04\xcd\x01\x04\n\n\r\n\x05\x04\x1e\x02\x01\x01\x12\x04\
    \xcd\x01\x0b\x10\n\r\n\x05\x04\x1e\x02\x01\x03\x12\x04\xcd\x01\x13\x14\n\
    \x0c\n\x02\x04\x1f\x12\x06\xd0\x01\0\xd3\x01\x01\n\x0b\n\x03\x04\x1f\x01\
    \x12\x04\xd0\x01\x08\x18\n\x0c\n\x04\x04\x1f\x02\0\x12\x04\xd1\x01\x04\
    \x18\n\x0f\n\x05\x04\x1f\x02\0\x04\x12\x06\xd1\x01\x04\xd0\x01\x1a\n\r\n\
    \x05\x04\x1f\x02\0\x06\x12\x04\xd1\x01\x04\x0b\n\r\n\x05\x04\x1f\x02\0\
    \x01\x12\x04\xd1\x01\x0c\x13\n\r\n\x05\x04\x1f\x02\0\x03\x12\x04\xd1\x01\
    \x16\x17\n\x0c\n\x04\x04\x1f\x02\x01\x12\x04\xd2\x01\x04\x12\n\x0f\n\x05\
    \x04\x1f\x02\x01\x04\x12\x06\xd2\x01\x04\xd1\x01\x18\n\r\n\x05\x04\x1f\
    \x02\x01\x05\x12\x04\xd2\x01\x04\t\n\r\n\x05\x04\x1f\x02\x01\x01\x12\x04\
    \xd2\x01\n\r\n\r\n\x05\x04\x1f\x02\x01\x03\x12\x04\xd2\x01\x10\x11\n\x0c\
    \n\x02\x04\x20\x12\x06\xd5\x01\0\xd8\x01\x01\n\x0b\n\x03\x04\x20\x01\x12\
    \x04\xd5\x01\x08\x19\n\x0c\n\x04\x04\x20\x02\0\x12\x04\xd6\x01\x04#\n\
    \x0f\n\x05\x04\x20\x02\0\x04\x12\x06\xd6\x01\x04\xd5\x01\x1b\n\r\n\x05\
    \x04\x20\x02\0\x06\x12\x04\xd6\x01\x04\x11\n\r\n\x05\x04\x20\x02\0\x01\
    \x12\x04\xd6\x01\x12\x1e\n\r\n\x05\x04\x20\x02\0\x03\x12\x04\xd6\x01!\"\
    \n\x0c\n\x04\x04\x20\x02\x01\x12\x04\xd7\x01\x04\x15\n\x0f\n\x05\x04\x20\
    \x02\x01\x04\x12\x06\xd7\x01\x04\xd6\x01#\n\r\n\x05\x04\x20\x02\x01\x05\
    \x12\x04\xd7\x01\x04\n\n\r\n\x05\x04\x20\x02\x01\x01\x12\x04\xd7\x01\x0b\
    \x10\n\r\n\x05\x04\x20\x02\x01\x03\x12\x04\xd7\x01\x13\x14\n\x0c\n\x02\
    \x04!\x12\x06\xda\x01\0\xdf\x01\x01\n\x0b\n\x03\x04!\x01\x12\x04\xda\x01\
    \x08\x12\n\x0c\n\x04\x04!\x02\0\x12\x04\xdb\x01\x04\x16\n\x0f\n\x05\x04!\
    \x02\0\x04\x12\x06\xdb\x01\x04\xda\x01\x14\n\r\n\x05\x04!\x02\0\x05\x12\
    \x04\xdb\x01\x04\n\n\r\n\x05\x04!\x02\0\x01\x12\x04\xdb\x01\x0b\x11\n\r\
    \n\x05\x04!\x02\0\x03\x12\x04\xdb\x01\x14\x15\n\x0c\n\x04\x04!\x02\x01\
    \x12\x04\xdc\x01\x04\x1e\n\x0f\n\x05\x04!\x02\x01\x04\x12\x06\xdc\x01\
    \x04\xdb\x01\x16\n\r\n\x05\x04!\x02\x01\x06\x12\x04\xdc\x01\x04\x11\n\r\
    \n\x05\x04!\x02\x01\x01\x12\x04\xdc\x01\x12\x19\n\r\n\x05\x04!\x02\x01\
    \x03\x12\x04\xdc\x01\x1c\x1d\n\x0c\n\x04\x04!\x02\x02\x12\x04\xdd\x01\
    \x04\x1e\n\x0f\n\x05\x04!\x02\x02\x04\x12\x06\xdd\x01\x04\xdc\x01\x1e\n\
    \r\n\x05\x04!\x02\x02\x06\x12\x04\xdd\x01\x04\x11\n\r\n\x05\x04!\x02\x02\
    \x01\x12\x04\xdd\x01\x12\x19\n\r\n\x05\x04!\x02\x02\x03\x12\x04\xdd\x01\
    \x1c\x1d\n\x0c\n\x04\x04!\x02\x03\x12\x04\xde\x01\x04$\n\x0f\n\x05\x04!\
    \x02\x03\x04\x12\x06\xde\x01\x04\xdd\x01\x1e\n\r\n\x05\x04!\x02\x03\x06\
    \x12\x04\xde\x01\x04\x14\n\r\n\x05\x04!\x02\x03\x01\x12\x04\xde\x01\x15\
    \x1f\n\r\n\x05\x04!\x02\x03\x03\x12\x04\xde\x01\"#\n\x0c\n\x02\x04\"\x12\
    \x06\xe1\x01\0\xe6\x01\x01\n\x0b\n\x03\x04\"\x01\x12\x04\xe1\x01\x08\x13\
    \n\x0c\n\x04\x04\"\x02\0\x12\x04\xe2\x01\x04\x16\n\x0f\n\x05\x04\"\x02\0\
    \x04\x12\x06\xe2\x01\x04\xe1\x01\x15\n\r\n\x05\x04\"\x02\0\x05\x12\x04\
    \xe2\x01\x04\n\n\r\n\x05\x04\"\x02\0\x01\x12\x04\xe2\x01\x0b\x11\n\r\n\
    \x05\x04\"\x02\0\x03\x12\x04\xe2\x01\x14\x15\n\x0c\n\x04\x04\"\x02\x01\
    \x12\x04\xe3\x01\x04\x1f\n\x0f\n\x05\x04\"\x02\x01\x04\x12\x06\xe3\x01\
    \x04\xe2\x01\x16\n\r\n\x05\x04\"\x02\x01\x06\x12\x04\xe3\x01\x04\x12\n\r\
    \n\x05\x04\"\x02\x01\x01\x12\x04\xe3\x01\x13\x1a\n\r\n\x05\x04\"\x02\x01\
    \x03\x12\x04\xe3\x01\x1d\x1e\n\x0c\n\x04\x04\"\x02\x02\x12\x04\xe4\x01\
    \x04\x1f\n\x0f\n\x05\x04\"\x02\x02\x04\x12\x06\xe4\x01\x04\xe3\x01\x1f\n\
    \r\n\x05\x04\"\x02\x02\x06\x12\x04\xe4\x01\x04\x12\n\r\n\x05\x04\"\x02\
    \x02\x01\x12\x04\xe4\x01\x13\x1a\n\r\n\x05\x04\"\x02\x02\x03\x12\x04\xe4\
    \x01\x1d\x1e\n\x0c\n\x04\x04\"\x02\x03\x12\x04\xe5\x01\x04%\n\x0f\n\x05\
    \x04\"\x02\x03\x04\x12\x06\xe5\x01\x04\xe4\x01\x1f\n\r\n\x05\x04\"\x02\
    \x03\x06\x12\x04\xe5\x01\x04\x15\n\r\n\x05\x04\"\x02\x03\x01\x12\x04\xe5\
    \x01\x16\x20\n\r\n\x05\x04\"\x02\x03\x03\x12\x04\xe5\x01#$b\x06proto3\
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
