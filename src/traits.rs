//! Traits for working with NBT data.

use super::types::{Tag, TagType, CompoundData, ListData};

use std::collections::HashMap;

/// Trait implementable by types that can be converted to NBT tags.
pub trait ToNbt {
    /// Encode `self` as an NBT tag.
    fn to_nbt(&self) -> Tag;
}

macro_rules! tonbt_impl {
        ($t:ty, $e:path) => { impl ToNbt for $t {
            fn to_nbt(&self) -> Tag {
                $e(self.clone())
            }
        }
    }
}

tonbt_impl!(i8, Tag::Byte);
tonbt_impl!(i16, Tag::Short);
tonbt_impl!(i32, Tag::Int);
tonbt_impl!(i64, Tag::Long);
tonbt_impl!(f32, Tag::Float);
tonbt_impl!(f64, Tag::Double);
tonbt_impl!(String, Tag::String);

impl<T> ToNbt for [T]
    where T: ToNbt {

    fn to_nbt(&self) -> Tag {
        if self.len() > 0 {
            let mut iter = self.iter();
            let first = iter.next().unwrap();

            let mut list = ListData {
                element_type: first.to_nbt().get_type(),
                elements: vec![first.to_nbt()]
            };

            for e in iter {
                let d = e.to_nbt();

                if d.get_type() == list.element_type {
                    list.elements.push(e.to_nbt());
                }
            }

            Tag::List(list)
        } else {
            Tag::List(ListData {
                element_type: TagType::Byte,
                elements: Vec::new()
            })
        }
    }
}


impl<'a, T> ToNbt for HashMap<String, T>
    where T: ToNbt {

    fn to_nbt(&self) -> Tag {
        let mut cd = CompoundData { elements: HashMap::new() };

        for (name, tag) in self {
            cd.elements.insert(name.clone(), tag.to_nbt());
        }

        Tag::Compound(cd)
    }
}

#[test]
fn test_tonbt() {
    assert!(42_i8.to_nbt() == Tag::Byte(42));
    assert!("test".to_owned().to_nbt() == Tag::String("test".to_owned()));
}

/// Trait implementable by types that can be converted from NBT tags.
pub trait FromNbt {
    fn from_nbt(val: &Tag) -> Option<Self>;
}

macro_rules! fromnbt_impl {
        ($t:ty, $($p:path),+) => { impl FromNbt for $t {
            fn from_nbt(val: &Tag) -> Option<Self> {
                match *val {
                    $(
                        $p(ref x) => Some(x.clone() as $t)
                    ),+,
                    _         => None
                }
            }
        }
    }
}

// Allow smaller integer tags to be promoted to bigger types if need be.
// It's a bit repetetive, but it works.
fromnbt_impl!(i8, Tag::Byte);
fromnbt_impl!(i16, Tag::Byte, Tag::Short);
fromnbt_impl!(i32, Tag::Byte, Tag::Short, Tag::Int);
fromnbt_impl!(i64, Tag::Byte, Tag::Short, Tag::Int, Tag::Long);
fromnbt_impl!(f32, Tag::Float);
fromnbt_impl!(f64, Tag::Float, Tag::Double);
fromnbt_impl!(String, Tag::String);

#[test]
fn test_fromnbt() {
    assert!(FromNbt::from_nbt(&Tag::Short(12)) == Some(12_i32));
    assert!(FromNbt::from_nbt(&Tag::Byte(42_i8)) == Some(42_i8));
    assert!(<i8 as FromNbt>::from_nbt(&Tag::Int(42_i32)) == None);
}
