
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::collections::HashMap;

macro_rules! tags {
    {$($tag:ident $val:expr;)*} => {
        #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
        pub enum Tag {
            $($tag,)*
            Unknown(u16),
        }

        impl Tag {
            pub fn from_u16(n: u16) -> Tag {
                $(if n == $val {
                    Tag::$tag
                } else)* {
                    Tag::Unknown(n)
                }
            }

            pub fn all() -> Vec<Tag> {
                vec![
                    $(Tag::$tag,)*
                ]
            }
        }
    }
}

tags!{
    ImageWidth 256;
    ImageLength 257;
    BitsPerSample 258;
    Compression 259;
    PhotometricInterpretation 262;
    StripOffsets 273;
    SamplesPerPixel 277;
    RowsPerStrip 278;
    StripByteCounts 279;
    XResolusion 282;
    YResolusion 283;
    ResolutionUnit 296;
}

#[derive(Debug, Clone)]
pub enum DataType {
    Byte,
    Short,
    Long,
    Rational,
    Unknown(u16),
}

impl DataType {
    pub fn from_u16(n: u16) -> DataType {
        match n {
            1 => DataType::Byte,
            3 => DataType::Short,
            4 => DataType::Long,
            5 => DataType::Rational,
            n => DataType::Unknown(n),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Entry {
    datatype: DataType,
    count: u32,
    offset: [u8; 4],
}

impl Entry {
    pub fn new(datatype: DataType, count: u32, offset: [u8; 4]) -> Entry {
        Entry {
            datatype: datatype,
            count: count,
            offset: offset,
        }
    }

    pub fn datatype(&self) -> &DataType {
        &self.datatype
    }

    pub fn count(&self) -> u32 {
        self.count
    }

    pub fn offset(&self) -> &[u8] {
        &self.offset
    }
}

#[derive(Debug, Clone)]
pub struct IFD(HashMap<Tag, Entry>);

impl IFD {
    pub fn new() -> IFD {
        IFD(HashMap::new())
    }

    pub fn insert(&mut self, k: Tag, v: Entry) -> Option<Entry> {
        self.0.insert(k, v)
    }
    
    #[inline]
    pub fn get(&self, k: &Tag) -> Option<&Entry> {
        self.0.get(k)
    }
}

