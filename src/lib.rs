
extern crate byteorder;
extern crate lzw;
#[macro_use] extern crate failure;

mod error;
mod byte;
mod decode;
mod ifd;
mod image;
mod tag;

pub use decode::{
    Decoder,
};

pub use ifd::{
    IFD,
    Entry,
};
pub use tag::{
    AnyTag,
};
pub use error::{
    DecodeError,
    DecodeErrorKind,
    DecodeResult,
};
