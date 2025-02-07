// @generated by the capnpc-rust plugin to the Cap'n Proto schema compiler.
// DO NOT EDIT.
// source: mood.capnp


pub mod mood {
    #[derive(Copy, Clone)]
    pub struct Owned(());
    impl ::capnp::introspect::Introspect for Owned {
        fn introspect() -> ::capnp::introspect::Type {
            ::capnp::introspect::TypeVariant::Struct(::capnp::introspect::RawBrandedStructSchema {
                generic: &_private::RAW_SCHEMA,
                field_types: _private::get_field_types,
                annotation_types: _private::get_annotation_types,
            })
            .into()
        }
    }
    impl ::capnp::traits::Owned for Owned {
        type Builder<'a> = Builder<'a>;
        type Reader<'a> = Reader<'a>;
    }
    impl ::capnp::traits::OwnedStruct for Owned {
        type Builder<'a> = Builder<'a>;
        type Reader<'a> = Reader<'a>;
    }
    impl ::capnp::traits::Pipelined for Owned {
        type Pipeline = Pipeline;
    }

    pub struct Reader<'a> {
        reader: ::capnp::private::layout::StructReader<'a>,
    }
    impl ::core::marker::Copy for Reader<'_> {}
    impl ::core::clone::Clone for Reader<'_> {
        fn clone(&self) -> Self {
            *self
        }
    }

    impl ::capnp::traits::HasTypeId for Reader<'_> {
        const TYPE_ID: u64 = _private::TYPE_ID;
    }
    impl<'a> ::core::convert::From<::capnp::private::layout::StructReader<'a>> for Reader<'a> {
        fn from(reader: ::capnp::private::layout::StructReader<'a>) -> Self {
            Self { reader }
        }
    }

    impl<'a> ::core::convert::From<Reader<'a>> for ::capnp::dynamic_value::Reader<'a> {
        fn from(reader: Reader<'a>) -> Self {
            Self::Struct(::capnp::dynamic_struct::Reader::new(
                reader.reader,
                ::capnp::schema::StructSchema::new(::capnp::introspect::RawBrandedStructSchema {
                    generic: &_private::RAW_SCHEMA,
                    field_types: _private::get_field_types,
                    annotation_types: _private::get_annotation_types,
                }),
            ))
        }
    }

    impl ::core::fmt::Debug for Reader<'_> {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            core::fmt::Debug::fmt(
                &::core::convert::Into::<::capnp::dynamic_value::Reader<'_>>::into(*self),
                f,
            )
        }
    }

    impl<'a> ::capnp::traits::FromPointerReader<'a> for Reader<'a> {
        fn get_from_pointer(
            reader: &::capnp::private::layout::PointerReader<'a>,
            default: ::core::option::Option<&'a [::capnp::Word]>,
        ) -> ::capnp::Result<Self> {
            ::core::result::Result::Ok(reader.get_struct(default)?.into())
        }
    }

    impl<'a> ::capnp::traits::IntoInternalStructReader<'a> for Reader<'a> {
        fn into_internal_struct_reader(self) -> ::capnp::private::layout::StructReader<'a> {
            self.reader
        }
    }

    impl<'a> ::capnp::traits::Imbue<'a> for Reader<'a> {
        fn imbue(&mut self, cap_table: &'a ::capnp::private::layout::CapTable) {
            self.reader
                .imbue(::capnp::private::layout::CapTableReader::Plain(cap_table))
        }
    }

    impl Reader<'_> {
        pub fn reborrow(&self) -> Reader<'_> {
            Self { ..*self }
        }

        pub fn total_size(&self) -> ::capnp::Result<::capnp::MessageSize> {
            self.reader.total_size()
        }

        #[inline]
        pub fn get_valence(self) -> i32 {
            self.reader.get_data_field::<i32>(0)
        }

        #[inline]
        pub fn get_arousal(self) -> i32 {
            self.reader.get_data_field::<i32>(1)
        }

        #[inline]
        pub fn get_dominance(self) -> i32 {
            self.reader.get_data_field::<i32>(2)
        }
    }

    pub struct Builder<'a> {
        builder: ::capnp::private::layout::StructBuilder<'a>,
    }
    impl ::capnp::traits::HasStructSize for Builder<'_> {
        const STRUCT_SIZE: ::capnp::private::layout::StructSize =
            ::capnp::private::layout::StructSize {
                data: 2,
                pointers: 0,
            };
    }
    impl ::capnp::traits::HasTypeId for Builder<'_> {
        const TYPE_ID: u64 = _private::TYPE_ID;
    }
    impl<'a> ::core::convert::From<::capnp::private::layout::StructBuilder<'a>> for Builder<'a> {
        fn from(builder: ::capnp::private::layout::StructBuilder<'a>) -> Self {
            Self { builder }
        }
    }

    impl<'a> ::core::convert::From<Builder<'a>> for ::capnp::dynamic_value::Builder<'a> {
        fn from(builder: Builder<'a>) -> Self {
            Self::Struct(::capnp::dynamic_struct::Builder::new(
                builder.builder,
                ::capnp::schema::StructSchema::new(::capnp::introspect::RawBrandedStructSchema {
                    generic: &_private::RAW_SCHEMA,
                    field_types: _private::get_field_types,
                    annotation_types: _private::get_annotation_types,
                }),
            ))
        }
    }

    impl<'a> ::capnp::traits::ImbueMut<'a> for Builder<'a> {
        fn imbue_mut(&mut self, cap_table: &'a mut ::capnp::private::layout::CapTable) {
            self.builder
                .imbue(::capnp::private::layout::CapTableBuilder::Plain(cap_table))
        }
    }

    impl<'a> ::capnp::traits::FromPointerBuilder<'a> for Builder<'a> {
        fn init_pointer(builder: ::capnp::private::layout::PointerBuilder<'a>, _size: u32) -> Self {
            builder
                .init_struct(<Self as ::capnp::traits::HasStructSize>::STRUCT_SIZE)
                .into()
        }

        fn get_from_pointer(
            builder: ::capnp::private::layout::PointerBuilder<'a>,
            default: ::core::option::Option<&'a [::capnp::Word]>,
        ) -> ::capnp::Result<Self> {
            ::core::result::Result::Ok(
                builder
                    .get_struct(
                        <Self as ::capnp::traits::HasStructSize>::STRUCT_SIZE,
                        default,
                    )?
                    .into(),
            )
        }
    }

    impl ::capnp::traits::SetterInput<Owned> for Reader<'_> {
        fn set_pointer_builder(
            mut pointer: ::capnp::private::layout::PointerBuilder<'_>,
            value: Self,
            canonicalize: bool,
        ) -> ::capnp::Result<()> {
            pointer.set_struct(&value.reader, canonicalize)
        }
    }

    impl<'a> Builder<'a> {
        pub fn into_reader(self) -> Reader<'a> {
            self.builder.into_reader().into()
        }

        pub fn reborrow(&mut self) -> Builder<'_> {
            Builder {
                builder: self.builder.reborrow(),
            }
        }

        pub fn reborrow_as_reader(&self) -> Reader<'_> {
            self.builder.as_reader().into()
        }

        pub fn total_size(&self) -> ::capnp::Result<::capnp::MessageSize> {
            self.builder.as_reader().total_size()
        }

        #[inline]
        pub fn get_valence(self) -> i32 {
            self.builder.get_data_field::<i32>(0)
        }

        #[inline]
        pub fn set_valence(&mut self, value: i32) {
            self.builder.set_data_field::<i32>(0, value);
        }

        #[inline]
        pub fn get_arousal(self) -> i32 {
            self.builder.get_data_field::<i32>(1)
        }

        #[inline]
        pub fn set_arousal(&mut self, value: i32) {
            self.builder.set_data_field::<i32>(1, value);
        }

        #[inline]
        pub fn get_dominance(self) -> i32 {
            self.builder.get_data_field::<i32>(2)
        }

        #[inline]
        pub fn set_dominance(&mut self, value: i32) {
            self.builder.set_data_field::<i32>(2, value);
        }
    }

    pub struct Pipeline {
        _typeless: ::capnp::any_pointer::Pipeline,
    }
    impl ::capnp::capability::FromTypelessPipeline for Pipeline {
        fn new(typeless: ::capnp::any_pointer::Pipeline) -> Self {
            Self {
                _typeless: typeless,
            }
        }
    }
    impl Pipeline {}
    mod _private {
        pub static ENCODED_NODE: [::capnp::Word; 63] = [
            ::capnp::word(0, 0, 0, 0, 6, 0, 6, 0),
            ::capnp::word(245, 99, 52, 43, 202, 209, 146, 172),
            ::capnp::word(11, 0, 0, 0, 1, 0, 2, 0),
            ::capnp::word(162, 153, 182, 4, 91, 8, 223, 244),
            ::capnp::word(0, 0, 7, 0, 0, 0, 0, 0),
            ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
            ::capnp::word(22, 0, 0, 0, 108, 0, 0, 0),
            ::capnp::word(21, 0, 0, 0, 130, 0, 0, 0),
            ::capnp::word(25, 0, 0, 0, 7, 0, 0, 0),
            ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
            ::capnp::word(21, 0, 0, 0, 175, 0, 0, 0),
            ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
            ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
            ::capnp::word(109, 111, 111, 100, 46, 99, 97, 112),
            ::capnp::word(110, 112, 58, 77, 111, 111, 100, 0),
            ::capnp::word(0, 0, 0, 0, 1, 0, 1, 0),
            ::capnp::word(12, 0, 0, 0, 3, 0, 4, 0),
            ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
            ::capnp::word(0, 0, 1, 0, 0, 0, 0, 0),
            ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
            ::capnp::word(69, 0, 0, 0, 66, 0, 0, 0),
            ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
            ::capnp::word(64, 0, 0, 0, 3, 0, 1, 0),
            ::capnp::word(76, 0, 0, 0, 2, 0, 1, 0),
            ::capnp::word(1, 0, 0, 0, 1, 0, 0, 0),
            ::capnp::word(0, 0, 1, 0, 1, 0, 0, 0),
            ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
            ::capnp::word(73, 0, 0, 0, 66, 0, 0, 0),
            ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
            ::capnp::word(68, 0, 0, 0, 3, 0, 1, 0),
            ::capnp::word(80, 0, 0, 0, 2, 0, 1, 0),
            ::capnp::word(2, 0, 0, 0, 2, 0, 0, 0),
            ::capnp::word(0, 0, 1, 0, 2, 0, 0, 0),
            ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
            ::capnp::word(77, 0, 0, 0, 82, 0, 0, 0),
            ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
            ::capnp::word(76, 0, 0, 0, 3, 0, 1, 0),
            ::capnp::word(88, 0, 0, 0, 2, 0, 1, 0),
            ::capnp::word(118, 97, 108, 101, 110, 99, 101, 0),
            ::capnp::word(4, 0, 0, 0, 0, 0, 0, 0),
            ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
            ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
            ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
            ::capnp::word(4, 0, 0, 0, 0, 0, 0, 0),
            ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
            ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
            ::capnp::word(97, 114, 111, 117, 115, 97, 108, 0),
            ::capnp::word(4, 0, 0, 0, 0, 0, 0, 0),
            ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
            ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
            ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
            ::capnp::word(4, 0, 0, 0, 0, 0, 0, 0),
            ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
            ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
            ::capnp::word(100, 111, 109, 105, 110, 97, 110, 99),
            ::capnp::word(101, 0, 0, 0, 0, 0, 0, 0),
            ::capnp::word(4, 0, 0, 0, 0, 0, 0, 0),
            ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
            ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
            ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
            ::capnp::word(4, 0, 0, 0, 0, 0, 0, 0),
            ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
            ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
        ];
        pub fn get_field_types(index: u16) -> ::capnp::introspect::Type {
            match index {
                0 => <i32 as ::capnp::introspect::Introspect>::introspect(),
                1 => <i32 as ::capnp::introspect::Introspect>::introspect(),
                2 => <i32 as ::capnp::introspect::Introspect>::introspect(),
                _ => panic!("invalid field index {}", index),
            }
        }
        pub fn get_annotation_types(
            child_index: Option<u16>,
            index: u32,
        ) -> ::capnp::introspect::Type {
            panic!("invalid annotation indices ({:?}, {}) ", child_index, index)
        }
        pub static RAW_SCHEMA: ::capnp::introspect::RawStructSchema =
            ::capnp::introspect::RawStructSchema {
                encoded_node: &ENCODED_NODE,
                nonunion_members: NONUNION_MEMBERS,
                members_by_discriminant: MEMBERS_BY_DISCRIMINANT,
                members_by_name: MEMBERS_BY_NAME,
            };
        pub static NONUNION_MEMBERS: &[u16] = &[0, 1, 2];
        pub static MEMBERS_BY_DISCRIMINANT: &[u16] = &[];
        pub static MEMBERS_BY_NAME: &[u16] = &[1, 2, 0];
        pub const TYPE_ID: u64 = 0xac92_d1ca_2b34_63f5;
    }
}
