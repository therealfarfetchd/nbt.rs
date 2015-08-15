#![feature(core_intrinsics)]
#![feature(convert)]

//! A a low level NBT decoding library that maps NBT structures onto
//! standard library containers.

extern crate flate2;

pub mod types;
pub mod decode;
pub mod encode;
pub mod util;
pub mod traits;

pub use types::*;

// Trait for encoding values to bytes
trait Encodable {
    fn to_bytes(&self) -> Vec<u8>;
}

macro_rules! make_encodable {
    // Encode an integral by shifting by expr, from left to right
    ($t:ty, $($n:expr),+) => {
        impl Encodable for $t {
            fn to_bytes(&self) -> Vec<u8> {
                vec![ $( (*self >> $n) as u8),+ ]
            }
        }
    };

    // Encode a value by transmuting it to another value and using that impl
    // i.e. transmute f32 to i32 -> i32.to_bytes()
    ($t:ty => $c:ty) => {
        impl Encodable for $t {
            fn to_bytes(&self) -> Vec<u8> {
                unsafe { std::mem::transmute::<$t, $c>(*self) }.to_bytes()
            }
        }
    };
}

make_encodable!(i8,                             0);
make_encodable!(i16,                         8, 0);
make_encodable!(i32,                 24, 16, 8, 0);
make_encodable!(i64, 56, 48, 40, 32, 24, 16, 8, 0);

make_encodable!(f32 => i32);
make_encodable!(f64 => i64);


macro_rules! make_decodable {
    ($t:ty, $s:expr, $($n:expr),+) => {
        impl Decodable for $t {
            fn from_bytes(d: &[u8]) -> Option<Self> {
                if d.len() != $s {
                    return None;
                }

                Some($((d[$n] as $t) << (8 * ($s - $n - 1)))|+)
            }
        }
    };

    ($t:ty => $c:ty) => {
        impl Decodable for $t {
            fn from_bytes(d: &[u8]) -> Option<Self> {
                match <$c as Decodable>::from_bytes(d) {
                    Some(x) => Some(unsafe { std::mem::transmute(x) }),
                    None    => None
                }
            }
        }
    };
}

// Trait for decoding values from bytes
trait Decodable {
    fn from_bytes(d: &[u8]) -> Option<Self>;
}

make_decodable!(i8,  1, 0);
make_decodable!(i16, 2, 0, 1);
make_decodable!(i32, 4, 0, 1, 2, 3);
make_decodable!(i64, 8, 0, 1, 2, 3, 4, 5, 6, 7);

make_decodable!(f32 => i32);
make_decodable!(f64 => i64);

#[test]
fn test_encode_decode() {
    assert_eq!(Some(0x1A2B_i16), i16::from_bytes(&0x1A2B_i16.to_bytes()));
    assert_eq!(Some(-3.14_f32), f32::from_bytes(&(-3.14_f32).to_bytes()));
}
