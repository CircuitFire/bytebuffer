use std::mem;
use std::convert::TryInto;

pub trait Buffer{
    const STATIC_SIZE: bool;
    
    fn into_buffer(&self, buffer: &mut [u8]);

    fn as_buffer(&self) -> Box<[u8]>;

    fn buffer_len(&self) -> usize;

    fn from_buffer(buffer: &[u8]) -> Self;
}

macro_rules! impl_buffer {
    ($type:ty) => {
        impl Buffer for $type{
            const STATIC_SIZE: bool = true;

            fn into_buffer(&self, buffer: &mut [u8]){
                buffer.clone_from_slice(&self.to_le_bytes());
            }

            fn as_buffer(&self) -> Box<[u8]>{
                Box::new(self.to_le_bytes())
            }

            fn buffer_len(&self) -> usize{
                mem::size_of::<Self>()
            }

            fn from_buffer(buffer: &[u8]) -> Self {
                Self::from_le_bytes(buffer.try_into().unwrap())
            }
        }
    };
}

impl Buffer for bool{
    const STATIC_SIZE: bool = true;

    fn into_buffer(&self, buffer: &mut [u8]){
        buffer[0] = *self as u8;
    }

    fn as_buffer(&self) -> Box<[u8]>{
        Box::new([*self as u8])
    }

    fn buffer_len(&self) -> usize{
        mem::size_of::<Self>()
    }

    fn from_buffer(buffer: &[u8]) -> Self {
        buffer[0] != 0
    }
}

impl Buffer for u8{
    const STATIC_SIZE: bool = true;

    fn into_buffer(&self, buffer: &mut [u8]){
        buffer[0] = *self;
    }

    fn as_buffer(&self) -> Box<[u8]>{
        Box::new([*self])
    }

    fn buffer_len(&self) -> usize{
        mem::size_of::<Self>()
    }

    fn from_buffer(buffer: &[u8]) -> Self {
        buffer[0]
    }
}

impl_buffer!(u16);
impl_buffer!(u32);
impl_buffer!(u64);
impl_buffer!(u128);

impl_buffer!(i8);
impl_buffer!(i16);
impl_buffer!(i32);
impl_buffer!(i64);
impl_buffer!(i128);

impl Buffer for char{
    const STATIC_SIZE: bool = true;

    fn into_buffer(&self, buffer: &mut [u8]){
        let temp = *self as u32;
        buffer.clone_from_slice(&temp.to_le_bytes());
    }

    fn as_buffer(&self) -> Box<[u8]>{
        let temp = *self as u32;
        Box::new(temp.to_le_bytes())
    }

    fn buffer_len(&self) -> usize{
        mem::size_of::<Self>()
    }

    fn from_buffer(buffer: &[u8]) -> Self {
        let temp = u32::from_le_bytes(buffer.try_into().unwrap());
        std::char::from_u32(temp).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! number_test {
        ($type:ty) => {
            let test = <$type>::MIN;
            println!("test min: {}", test);
            assert_eq!(test, <$type>::from_buffer(&test.as_buffer()));
            let test = <$type>::MAX;
            println!("test max: {}", test);
            assert_eq!(test, <$type>::from_buffer(&test.as_buffer()));
        }
    }

    #[test]
    fn bool_buff() {
        let test = true;
        assert_eq!(test, bool::from_buffer(&test.as_buffer()));
        let test = false;
        assert_eq!(test, bool::from_buffer(&test.as_buffer()));
    }

    #[test]
    fn u8_buff() {number_test!(u8);}

    #[test]
    fn u16_buff() {number_test!(u16);}

    #[test]
    fn u32_buff() {number_test!(u32);}

    #[test]
    fn u64_buff() {number_test!(u64);}

    #[test]
    fn u128_buff() {number_test!(u128);}

    #[test]
    fn i8_buff() {number_test!(i8);}

    #[test]
    fn i16_buff() {number_test!(i16);}

    #[test]
    fn i32_buff() {number_test!(i32);}

    #[test]
    fn i64_buff() {number_test!(i64);}

    #[test]
    fn i128_buff() {number_test!(i128);}

    #[test]
    fn char_buff() {
        let test = 't';
        assert_eq!(test, char::from_buffer(&test.as_buffer()));
    }
}