// This file is generated by rust-protobuf 3.7.2. Do not edit
// .proto file is parsed by protoc 28.2
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `msg_initiate_token_withdrawal.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_2;

// @@protoc_insertion_point(message:opinit.opchild.v1.Coin)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct Coin {
    // message fields
    // @@protoc_insertion_point(field:opinit.opchild.v1.Coin.denom)
    pub denom: ::std::string::String,
    // @@protoc_insertion_point(field:opinit.opchild.v1.Coin.amount)
    pub amount: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:opinit.opchild.v1.Coin.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Coin {
    fn default() -> &'a Coin {
        <Coin as ::protobuf::Message>::default_instance()
    }
}

impl Coin {
    pub fn new() -> Coin {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "denom",
            |m: &Coin| { &m.denom },
            |m: &mut Coin| { &mut m.denom },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "amount",
            |m: &Coin| { &m.amount },
            |m: &mut Coin| { &mut m.amount },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Coin>(
            "Coin",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Coin {
    const NAME: &'static str = "Coin";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.denom = is.read_string()?;
                },
                18 => {
                    self.amount = is.read_string()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if !self.denom.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.denom);
        }
        if !self.amount.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.amount);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.denom.is_empty() {
            os.write_string(1, &self.denom)?;
        }
        if !self.amount.is_empty() {
            os.write_string(2, &self.amount)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> Coin {
        Coin::new()
    }

    fn clear(&mut self) {
        self.denom.clear();
        self.amount.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Coin {
        static instance: Coin = Coin {
            denom: ::std::string::String::new(),
            amount: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Coin {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Coin").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Coin {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Coin {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:opinit.opchild.v1.MsgInitiateTokenWithdrawal)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct MsgInitiateTokenWithdrawal {
    // message fields
    // @@protoc_insertion_point(field:opinit.opchild.v1.MsgInitiateTokenWithdrawal.sender)
    pub sender: ::std::string::String,
    // @@protoc_insertion_point(field:opinit.opchild.v1.MsgInitiateTokenWithdrawal.to)
    pub to: ::std::string::String,
    // @@protoc_insertion_point(field:opinit.opchild.v1.MsgInitiateTokenWithdrawal.amount)
    pub amount: ::protobuf::MessageField<Coin>,
    // special fields
    // @@protoc_insertion_point(special_field:opinit.opchild.v1.MsgInitiateTokenWithdrawal.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MsgInitiateTokenWithdrawal {
    fn default() -> &'a MsgInitiateTokenWithdrawal {
        <MsgInitiateTokenWithdrawal as ::protobuf::Message>::default_instance()
    }
}

impl MsgInitiateTokenWithdrawal {
    pub fn new() -> MsgInitiateTokenWithdrawal {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "sender",
            |m: &MsgInitiateTokenWithdrawal| { &m.sender },
            |m: &mut MsgInitiateTokenWithdrawal| { &mut m.sender },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "to",
            |m: &MsgInitiateTokenWithdrawal| { &m.to },
            |m: &mut MsgInitiateTokenWithdrawal| { &mut m.to },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, Coin>(
            "amount",
            |m: &MsgInitiateTokenWithdrawal| { &m.amount },
            |m: &mut MsgInitiateTokenWithdrawal| { &mut m.amount },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MsgInitiateTokenWithdrawal>(
            "MsgInitiateTokenWithdrawal",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MsgInitiateTokenWithdrawal {
    const NAME: &'static str = "MsgInitiateTokenWithdrawal";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.sender = is.read_string()?;
                },
                18 => {
                    self.to = is.read_string()?;
                },
                34 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.amount)?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if !self.sender.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.sender);
        }
        if !self.to.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.to);
        }
        if let Some(v) = self.amount.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.sender.is_empty() {
            os.write_string(1, &self.sender)?;
        }
        if !self.to.is_empty() {
            os.write_string(2, &self.to)?;
        }
        if let Some(v) = self.amount.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> MsgInitiateTokenWithdrawal {
        MsgInitiateTokenWithdrawal::new()
    }

    fn clear(&mut self) {
        self.sender.clear();
        self.to.clear();
        self.amount.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MsgInitiateTokenWithdrawal {
        static instance: MsgInitiateTokenWithdrawal = MsgInitiateTokenWithdrawal {
            sender: ::std::string::String::new(),
            to: ::std::string::String::new(),
            amount: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MsgInitiateTokenWithdrawal {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MsgInitiateTokenWithdrawal").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MsgInitiateTokenWithdrawal {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MsgInitiateTokenWithdrawal {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Extension fields
pub mod exts {

    pub const goproto_getters_all: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FileOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(63001, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const scalar: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FieldOptions, ::std::string::String> = ::protobuf::ext::ExtFieldOptional::new(93002, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_STRING);

    pub const nullable: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FieldOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(65001, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const customtype: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FieldOptions, ::std::string::String> = ::protobuf::ext::ExtFieldOptional::new(65003, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_STRING);

    pub const moretags: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FieldOptions, ::std::string::String> = ::protobuf::ext::ExtFieldOptional::new(65006, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_STRING);

    pub const dont_omitempty: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FieldOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(11110005, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const equal: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::MessageOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(64013, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const name: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::MessageOptions, ::std::string::String> = ::protobuf::ext::ExtFieldOptional::new(11110001, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_STRING);

    pub const signer: ::protobuf::ext::ExtFieldRepeated<::protobuf::descriptor::MessageOptions, ::std::string::String> = ::protobuf::ext::ExtFieldRepeated::new(11110000, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_STRING);
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n#msg_initiate_token_withdrawal.proto\x12\x11opinit.opchild.v1\x1a\x20g\
    oogle/protobuf/descriptor.proto\"l\n\x04Coin\x12\x14\n\x05denom\x18\x01\
    \x20\x01(\tR\x05denom\x12H\n\x06amount\x18\x02\x20\x01(\tR\x06amountB0\
    \xd2\xb4-\ncosmos.Int\xc8\xde\x1f\0\xda\xde\x1f\x15cosmossdk.io/math.Int\
    \xa8\xe7\xb0*\x01:\x04\xe8\xa0\x1f\x01\"\xf9\x01\n\x1aMsgInitiateTokenWi\
    thdrawal\x120\n\x06sender\x18\x01\x20\x01(\tR\x06senderB\x18\xd2\xb4-\
    \x14cosmos.AddressString\x12(\n\x02to\x18\x02\x20\x01(\tR\x02toB\x18\xd2\
    \xb4-\x14cosmos.AddressString\x12K\n\x06amount\x18\x04\x20\x01(\x0b2\x17\
    .opinit.opchild.v1.CoinR\x06amountB\x1a\xf2\xde\x1f\ryaml:\"amount\"\xc8\
    \xde\x1f\0\xa8\xe7\xb0*\x01:2\x8a\xe7\xb0*\"opchild/MsgInitiateTokenWith\
    drawal\x82\xe7\xb0*\x06sender:Q\n\x13goproto_getters_all\x18\x99\xec\x03\
    \x20\x01(\x08\x12\x1c.google.protobuf.FileOptionsR\x11goprotoGettersAll\
    \x88\x01\x01:7\n\x06scalar\x18\xca\xd6\x05\x20\x01(\t\x12\x1d.google.pro\
    tobuf.FieldOptionsR\x06scalar:>\n\x08nullable\x18\xe9\xfb\x03\x20\x01(\
    \x08\x12\x1d.google.protobuf.FieldOptionsR\x08nullable\x88\x01\x01:B\n\n\
    customtype\x18\xeb\xfb\x03\x20\x01(\t\x12\x1d.google.protobuf.FieldOptio\
    nsR\ncustomtype\x88\x01\x01:>\n\x08moretags\x18\xee\xfb\x03\x20\x01(\t\
    \x12\x1d.google.protobuf.FieldOptionsR\x08moretags\x88\x01\x01:G\n\x0edo\
    nt_omitempty\x18\xf5\x8c\xa6\x05\x20\x01(\x08\x12\x1d.google.protobuf.Fi\
    eldOptionsR\rdontOmitempty::\n\x05equal\x18\x8d\xf4\x03\x20\x01(\x08\x12\
    \x1f.google.protobuf.MessageOptionsR\x05equal\x88\x01\x01:6\n\x04name\
    \x18\xf1\x8c\xa6\x05\x20\x01(\t\x12\x1f.google.protobuf.MessageOptionsR\
    \x04name::\n\x06signer\x18\xf0\x8c\xa6\x05\x20\x03(\t\x12\x1f.google.pro\
    tobuf.MessageOptionsR\x06signerB3Z-github.com/initia-labs/OPinit/x/opchi\
    ld/types\xc8\xe1\x1e\0b\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(::protobuf::descriptor::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(2);
            messages.push(Coin::generated_message_descriptor_data());
            messages.push(MsgInitiateTokenWithdrawal::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
