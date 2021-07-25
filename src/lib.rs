use std::mem::size_of;
use std::convert::TryInto;
use std::io::Write;

pub trait Bytes{
    const STATIC_SIZE: bool;

    fn into_bytes(&self) -> Box<dyn Iterator<Item = u8>>;

    fn bytes_len(&self) -> usize;

    fn from_bytes<T: Iterator<Item = u8>>(bytes: &mut T) -> Result<Self, ()> where Self: Sized;
}

pub struct ByteBuffer{
    buffer: Vec<u8>,
    current: usize,
}

impl ByteBuffer {
    fn new(buffer: Vec<u8>) -> Box<ByteBuffer> {
        Box::new(ByteBuffer{
            buffer: buffer,
            current: 0,
        })
    }
}

impl Iterator for ByteBuffer {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.buffer.len() {
            let result = Some(self.buffer[self.current]);
            self.current += 1;
            result
        }
        else{
            None
        }
    }
}

pub trait WriteBytes: Write {
    fn write_bytes<T: Iterator<Item = u8>>(&mut self, byte_iter: &mut T, buffer_size: usize) -> Result<(), std::io::Error>{
        let mut buffer: Vec<u8>;

        loop {
            buffer = byte_iter.take(buffer_size).collect();
            if buffer.is_empty() {break}

            self.write_all(&buffer[..])?;
        }

        Ok(())
    }
}

impl Bytes for bool{
    const STATIC_SIZE: bool = true;

    fn into_bytes(&self) -> Box<dyn Iterator<Item = u8>>{
        ByteBuffer::new(vec![*self as u8])
    }

    fn bytes_len(&self) -> usize{
        size_of::<Self>()
    }

    fn from_bytes<T: Iterator<Item = u8>>(bytes: &mut T) -> Result<Self, ()> {
        if let Some(byte) = bytes.next(){
            Ok(byte != 0)
        }
        else{
            Err(())
        }
    }
}

impl Bytes for char{
    const STATIC_SIZE: bool = true;

    fn into_bytes(&self) -> Box<dyn Iterator<Item = u8>>{
        let temp = *self as u32;
        ByteBuffer::new(temp.to_le_bytes().to_vec())
    }

    fn bytes_len(&self) -> usize{
        size_of::<Self>()
    }

    fn from_bytes<T: Iterator<Item = u8>>(bytes: &mut T) -> Result<Self, ()> {
        let buffer = bytes.take(size_of::<Self>()).collect::<Vec<u8>>();

        if buffer.len() != size_of::<Self>() {
            // not enough bytes
            return Err(())
        }

        let temp = u32::from_le_bytes(buffer.try_into().unwrap());

        if let Some(character) = std::char::from_u32(temp) {
            return Ok(character)
        }
        else {
            // invalid character
            Err(())
        }
    }
}

impl Bytes for u8{
    const STATIC_SIZE: bool = true;

    fn into_bytes(&self) -> Box<dyn Iterator<Item = u8>>{
        ByteBuffer::new(vec![*self])
    }

    fn bytes_len(&self) -> usize{
        size_of::<Self>()
    }

    fn from_bytes<T: Iterator<Item = u8>>(bytes: &mut T) -> Result<Self, ()> {
        if let Some(byte) = bytes.next(){
            Ok(byte)
        }
        else{
            Err(())
        }
    }
}

macro_rules! impl_buffer {
    ($type:ty) => {
        impl Bytes for $type{
            const STATIC_SIZE: bool = true;

            fn into_bytes(&self) -> Box<dyn Iterator<Item = u8>>{
                ByteBuffer::new(self.to_le_bytes().to_vec())
            }
        
            fn bytes_len(&self) -> usize{
                size_of::<Self>()
            }
        
            fn from_bytes<T: Iterator<Item = u8>>(bytes: &mut T) -> Result<Self, ()> {
                let buffer = bytes.take(size_of::<Self>()).collect::<Vec<u8>>();
        
                if buffer.len() != size_of::<Self>() {
                    // not enough bytes
                    return Err(())
                }
        
                Ok(<$type>::from_le_bytes(buffer.try_into().unwrap()))
            }
        }
    };
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

impl_buffer!(f32);
impl_buffer!(f64);

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! number_test {
        ($type:ty) => {
            let test = <$type>::MIN;
            println!("test min: {}", test);
            assert_eq!(test, <$type>::from_bytes(&mut test.into_bytes()).unwrap());
            let test = 75;
            println!("test max: {}", test);
            assert_eq!(test, <$type>::from_bytes(&mut test.into_bytes()).unwrap());
            let test = <$type>::MAX;
            println!("test max: {}", test);
            assert_eq!(test, <$type>::from_bytes(&mut test.into_bytes()).unwrap());
        }
    }

    #[test]
    fn bool_buff() {
        let test = true;
        assert_eq!(test, bool::from_bytes(&mut test.into_bytes()).unwrap());
        let test = false;
        assert_eq!(test, bool::from_bytes(&mut test.into_bytes()).unwrap());
    }

    #[test]
    fn char_buff() {
        let test = 't';
        assert_eq!(test, char::from_bytes(&mut test.into_bytes()).unwrap());
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
    fn f32_buff() {
        let test: f32 = 3.75;
        assert_eq!(test, f32::from_bytes(&mut test.into_bytes()).unwrap());
        let test: f32 = -3.75;
        assert_eq!(test, f32::from_bytes(&mut test.into_bytes()).unwrap());
    }

    #[test]
    fn f64_buff() {
        let test: f64 = 3.75;
        assert_eq!(test, f64::from_bytes(&mut test.into_bytes()).unwrap());
        let test: f64 = -3.75;
        assert_eq!(test, f64::from_bytes(&mut test.into_bytes()).unwrap());
    }
}
