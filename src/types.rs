//! Core types used by the crate.

use std;

use std::ops::{Deref, DerefMut};
use std::collections::HashMap;

use util::{IndexOpt, IndexOptMut};

/// Compression flags
#[derive(Debug)]
pub enum Compression {
    /// Don't compress or uncompress.
    Uncompressed,

    /// Compress and uncompress using GZip.
    GZip
}

/// Things that can go wrong when reading or writing NBT tags.
#[derive(Debug)]
pub enum Error {
    /// Not a real error, used as a signal to stop compound reading. (TODO!)
    EndOfCompound,

    /// An invalid tag ID was encountered.
    Malformed,

    /// Tried to serialize an invalid NBT structure.
    Invalid,

    /// An IO error happened while decoding or encoding an NBT Tag.
    IOError(std::io::Error)
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error::IOError(e)
    }
}

/// Possible NBT tag types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TagType {
    /// End marker
    End,
    /// 8 bit signed integer.
    Byte,
    /// 16 bit signed integer.
    Short,
    /// 32 bit signed integer.
    Int,
    /// 64 bit signed integer.
    Long,
    /// IEEE-754 floating point.
    Float,
    /// IEEE-754 double precision floating point.
    Double,
    /// UTF-8 string.
    String,
    /// Vector of unsigned 8 bit integers.
    ByteArray,
    /// Vector of signed 32 bit integers.
    IntArray,
    /// Vector of NBT tags.
    List,
    /// Hash table of NBT tags indexed by UTF-8 strings.
    Compound
}

impl TagType {
    pub fn from_binary(t: u8) -> Option<TagType> {
        match t {
            0 => Some(TagType::End),
            1 => Some(TagType::Byte),
            2 => Some(TagType::Short),
            3 => Some(TagType::Int),
            4 => Some(TagType::Long),
            5 => Some(TagType::Float),
            6 => Some(TagType::Double),
            7 => Some(TagType::ByteArray),
            8 => Some(TagType::String),
            9 => Some(TagType::List),
            10 => Some(TagType::Compound),
            11 => Some(TagType::IntArray),
            _  => None
        }
    }

    pub fn to_binary(&self) -> u8 {
        match *self {
            TagType::End       => 0,
            TagType::Byte      => 1,
            TagType::Short     => 2,
            TagType::Int       => 3,
            TagType::Long      => 4,
            TagType::Float     => 5,
            TagType::Double    => 6,
            TagType::ByteArray => 7,
            TagType::String    => 8,
            TagType::List      => 9,
            TagType::Compound  => 10,
            TagType::IntArray  => 11
        }
    }
}


/// The internal representation of a list
#[derive(Debug, PartialEq)]
pub struct ListData {
    pub element_type: TagType,
    pub elements: Vec<Tag>
}

impl IndexOpt<usize> for ListData {
    type Output = Tag;

    fn index_opt<'a>(&'a self, i: usize) -> Option<&'a Tag> {
        if i >= self.elements.len() {
            None
        } else {
            Some(&self.elements[i])
        }
    }
}

impl IndexOptMut<usize> for ListData {
    fn index_opt_mut<'a>(&'a mut self, i: usize) -> Option<&'a mut Tag> {
        if i >= self.elements.len() {
            None
        } else {
            Some(&mut self.elements[i])
        }
    }
}


impl Deref for ListData {
    type Target = Vec<Tag>;

    fn deref<'a>(&'a self) -> &'a Self::Target {
        &self.elements
    }
}

impl DerefMut for ListData {
    fn deref_mut<'a>(&'a mut self) -> &'a mut Self::Target {
        &mut self.elements
    }
}


/// The internal representation of a compound
#[derive(Debug, PartialEq)]
pub struct CompoundData {
    pub elements: HashMap<String, Tag>
}

impl<'a> IndexOpt<&'a str> for CompoundData {
    type Output = Tag;

    fn index_opt<'b>(&'b self, i: &'a str) -> Option<&'b Self::Output> {
        self.elements.get(i)
    }
}

impl<'a> IndexOptMut<&'a str> for CompoundData {
    fn index_opt_mut<'b>(&'b mut self, i: &'a str) -> Option<&'b mut Self::Output> {
        self.elements.get_mut(i)
    }
}


impl Deref for CompoundData {
    type Target = HashMap<String, Tag>;

    fn deref<'a>(&'a self) -> &'a Self::Target {
        &self.elements
    }
}

impl DerefMut for CompoundData {
    fn deref_mut<'a>(&'a mut self) -> &'a mut Self::Target {
        &mut self.elements
    }
}


#[test]
fn test_aggregate() {
    let list = ListData {
        element_type: TagType::Short,
        elements: vec![Tag::Short(1), Tag::Short(2), Tag::Short(3)]
    };

    assert_eq!(list.index_opt(0), Some(&Tag::Short(1)));
    assert_eq!(list.index_opt(5), None);


    let mut comp = CompoundData {
        elements: HashMap::new()
    };

    comp.insert("Foo".to_owned(), Tag::String("Bar".to_owned()));
    comp.insert("Bar".to_owned(), Tag::Short(42));

    assert_eq!(comp.index_opt("Foo"), Some(&Tag::String("Bar".to_owned())));
}

/// An NBT value type.
#[derive(Debug, PartialEq)]
pub enum Tag {
    /// End marker.
    End,

    /// 8 bit signed integer.
    Byte(i8),
    /// 16 bit signed integer.
    Short(i16),
    /// 32 bit signed integer.
    Int(i32),
    /// 64 bit signed integer.
    Long(i64),
    /// IEEE-754 floating point.
    Float(f32),
    /// IEEE-754 double precision floating point.
    Double(f64),
    /// UTF-8 string.
    String(String),
    /// Vector of unsigned 8 bit integers.
    ByteArray(Vec<u8>),
    /// Vector of signed 32 bit integers.
    IntArray(Vec<i32>),
    /// Vector of NBT tags.
    List(ListData),
    /// Hash table of NBT tags indexed by UTF-8 strings.
    Compound(CompoundData)
}

impl Tag {
    /// Return the tag's type.
    pub fn get_type(&self) -> TagType {
        match *self {
            Tag::End          => TagType::End,
            Tag::Byte(_)      => TagType::Byte,
            Tag::Short(_)     => TagType::Short,
            Tag::Int(_)       => TagType::Int,
            Tag::Long(_)      => TagType::Long,
            Tag::Float(_)     => TagType::Float,
            Tag::Double(_)    => TagType::Double,
            Tag::String(_)    => TagType::String,
            Tag::ByteArray(_) => TagType::ByteArray,
            Tag::IntArray(_)  => TagType::IntArray,
            Tag::List(_)      => TagType::List,
            Tag::Compound(_)  => TagType::Compound
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

