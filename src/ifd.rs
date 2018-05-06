
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
        }
    }
}

tags!{
    // must ?
    ImageWidth 256;
    ImageLength 257;
    BitsPerSample 258;
    Compression 259;
    PhotometricInterpretation 262;
    StripOffsets 273;
    RowsPerStrip 278;
    StripByteCounts 279;
    XResolusion 282;
    YResolusion 283;
    ResolutionUnit 296;
    ColorMap 320;
    
    // options
    NewSubfileType 254;
    Thresholding 263;
    CellWidth 264;
    CellLength 265;
    FillOrder 266;
    ImageDescription 270;
    Make 271;
    Model 272;
    Orientation 274;
    SamplesPerPixel 277;
    MinSampleValue 280;
    MaxSampleValue 281;
    PlanarConfiguration 284;
    GrayResponseUnit 290;
    GrayResponseCurve 291;
    Software 305;
    DateTime 306;
    HostComputer 316;
    ExtraSamples 338;
    Copyright 33432;
}

#[derive(Debug, Clone, Copy)]
pub enum DataType {
    Byte,
    Ascii,
    Short,
    Long,
    Rational,
    Unknown(u16),
}

impl DataType {
    pub fn from_u16(n: u16) -> DataType {
        match n {
            1 => DataType::Byte,
            2 => DataType::Ascii,
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
}

pub type IFD = HashMap<Tag, Entry>;
