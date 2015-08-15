//! Helper traits and types.

use super::types::Tag;
use super::traits::ToNbt;

/// Index trait for index operations where a result may not be available.
pub trait IndexOpt<Idx> {
    type Output;

    fn index_opt<'a>(&'a self, i: Idx) -> Option<&'a Self::Output>;
}

/// Index trait for mutable index operations where a result may not be
/// available.
pub trait IndexOptMut<Idx>: IndexOpt<Idx> {
    fn index_opt_mut<'a>(&'a mut self, i: Idx) -> Option<&'a mut Self::Output>;
}


/// Wrapper type for generating byte arrays
pub struct ByteArrayWrapper<'a> {
    data: &'a [u8]
}

impl<'a> ByteArrayWrapper<'a> {
    pub fn new(d: &'a [u8]) -> ByteArrayWrapper<'a> {
        ByteArrayWrapper {
            data: d
        }
    }
}

impl<'a> ToNbt for ByteArrayWrapper<'a> {
    fn to_nbt(&self) -> Tag {
        let mut v = Vec::new();

        v.extend(self.data.iter());

        Tag::ByteArray(v)
    }
}


/// Wrapper type for generating int arrays
pub struct IntArrayWrapper<'a> {
    data: &'a [i32]
}

impl<'a> IntArrayWrapper<'a> {
    pub fn new(d: &'a [i32]) -> IntArrayWrapper<'a> {
        IntArrayWrapper {
            data: d
        }
    }
}

impl<'a> ToNbt for IntArrayWrapper<'a> {
    fn to_nbt(&self) -> Tag {
        let mut v = Vec::new();

        v.extend(self.data.iter());

        Tag::IntArray(v)
    }
}
