#![allow(dead_code)]

pub trait CapnpWrite<'a> {
    type Builder;

    fn write_capnp(&self, builder: &mut Self::Builder);
}

pub trait CapnpRead<'a> {
    type Reader;

    fn read_capnp(reader: Self::Reader) -> Self;
}
