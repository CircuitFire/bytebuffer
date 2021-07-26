//! # ByteBuffer
//! Allows for easy conversion of data to and from u8 iters.
//! 
//! ## Structs
//! - ByteBuffer<T>
//! 
//! ## Enums
//! - ByteErr
//! 
//! ## Traits
//! - Bytes
//! - WriteBytes

use std::mem::size_of;
use std::convert::TryInto;
use std::io::{Write, Error};
use std::fs::File;

/// The collection of errors.
#[derive(Debug)]
pub enum ByteErr {
    NeedMoreBytes,
    InvalidBytes,
    IoErr(Error),
}

impl From<Error> for ByteErr {
    fn from(err: Error) -> Self {
        ByteErr::IoErr(err)
    }
}

/// The Bytes trait allows for the easy conversion of data to and from u8 iters.
pub trait Bytes{
    /// Creates an iter over the bytes of self.
    fn into_bytes(&self) -> Box<dyn Iterator<Item = u8>>;

    /// Tries to create the indicated type out of an io byte iter.
    fn from_io_bytes<T: Iterator<Item = Result<u8, Error>>>(bytes: &mut T) -> Result<Self, ByteErr> where Self: Sized;

    /// Tries to create the indicated type out of a byte iter.
    fn from_bytes<T: Iterator<Item = u8>>(bytes: &mut T) -> Result<Self, ByteErr> where Self: Sized;
}

/// The iter over the bytes for all of the basic types.
pub struct ByteBuffer<T>{
    buffer: T,
    current: usize,
}

trait MakeByteBuffer<T> {
    fn new(buffer: T) -> Box<ByteBuffer<T>>;
}

macro_rules! impl_byte_buffer {
    ($type:ty) => {
        impl MakeByteBuffer<$type> for ByteBuffer<$type> {
            fn new(buffer: $type) -> Box<ByteBuffer<$type>> {
                Box::new(ByteBuffer{
                    buffer: buffer,
                    current: 0,
                })
            }
        }
        
        impl Iterator for ByteBuffer<$type> {
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
    };
}

impl_byte_buffer!([u8; 1]);
impl_byte_buffer!([u8; 2]);
impl_byte_buffer!([u8; 4]);
impl_byte_buffer!([u8; 8]);
impl_byte_buffer!([u8; 16]);

/// Extension of the Write trait writing all bytes from a byte iter.
pub trait WriteBytes: Write {
    fn write_bytes<T: Iterator<Item = u8>>(&mut self, byte_iter: &mut T, buffer_size: usize) -> Result<(), Error>{
        let mut buffer: Vec<u8>;

        loop {
            buffer = byte_iter.take(buffer_size).collect();
            if buffer.is_empty() {break}

            self.write_all(&buffer[..])?;
        }

        Ok(())
    }
}

impl WriteBytes for File {}

impl Bytes for bool{

    fn into_bytes(&self) -> Box<dyn Iterator<Item = u8>>{
        ByteBuffer::new([*self as u8])
    }

    fn from_io_bytes<T: Iterator<Item = Result<u8, Error>>>(bytes: &mut T) -> Result<Self, ByteErr> {
        if let Some(byte) = bytes.next() {
            Ok(byte? != 0)
        }
        else{
            Err(ByteErr::NeedMoreBytes)
        }
    }

    fn from_bytes<T: Iterator<Item = u8>>(bytes: &mut T) -> Result<Self, ByteErr> {
        if let Some(byte) = bytes.next(){
            Ok(byte != 0)
        }
        else{
            Err(ByteErr::NeedMoreBytes)
        }
    }
}

impl Bytes for char{

    fn into_bytes(&self) -> Box<dyn Iterator<Item = u8>>{
        let temp = *self as u32;
        ByteBuffer::new(temp.to_le_bytes())
    }

    fn from_io_bytes<T: Iterator<Item = Result<u8, Error>>>(bytes: &mut T) -> Result<Self, ByteErr> {
        let buffer = bytes.take(size_of::<Self>()).collect::<Result<Vec<_>, _>>()?;

        if buffer.len() != size_of::<Self>() {
            // not enough bytes
            return Err(ByteErr::NeedMoreBytes)
        }

        let temp = u32::from_le_bytes(buffer.try_into().unwrap());

        if let Some(character) = std::char::from_u32(temp) {
            return Ok(character)
        }
        else {
            // invalid character
            Err(ByteErr::InvalidBytes)
        }
    }

    fn from_bytes<T: Iterator<Item = u8>>(bytes: &mut T) -> Result<Self, ByteErr> {
        let buffer = bytes.take(size_of::<Self>()).collect::<Vec<u8>>();

        if buffer.len() != size_of::<Self>() {
            // not enough bytes
            return Err(ByteErr::NeedMoreBytes)
        }

        let temp = u32::from_le_bytes(buffer.try_into().unwrap());

        if let Some(character) = std::char::from_u32(temp) {
            return Ok(character)
        }
        else {
            // invalid character
            Err(ByteErr::InvalidBytes)
        }
    }
}

impl Bytes for u8{

    fn into_bytes(&self) -> Box<dyn Iterator<Item = u8>>{
        ByteBuffer::new([*self])
    }

    fn from_io_bytes<T: Iterator<Item = Result<u8, Error>>>(bytes: &mut T) -> Result<Self, ByteErr> {
        if let Some(byte) = bytes.next(){
            Ok(byte?)
        }
        else{
            Err(ByteErr::NeedMoreBytes)
        }
    }
    fn from_bytes<T: Iterator<Item = u8>>(bytes: &mut T) -> Result<Self, ByteErr> {
        if let Some(byte) = bytes.next(){
            Ok(byte)
        }
        else{
            Err(ByteErr::NeedMoreBytes)
        }
    }
}

macro_rules! impl_buffer {
    ($type:ty) => {
        impl Bytes for $type{

            fn into_bytes(&self) -> Box<dyn Iterator<Item = u8>>{
                ByteBuffer::new(self.to_le_bytes())
            }
        
            fn from_io_bytes<T: Iterator<Item = Result<u8, Error>>>(bytes: &mut T) -> Result<Self, ByteErr> {
                let buffer = bytes.take(size_of::<Self>()).collect::<Result<Vec<_>, _>>()?;
        
                if buffer.len() != size_of::<Self>() {
                    // not enough bytes
                    return Err(ByteErr::NeedMoreBytes)
                }
        
                Ok(<$type>::from_le_bytes(buffer.try_into().unwrap()))
            }
        
            fn from_bytes<T: Iterator<Item = u8>>(bytes: &mut T) -> Result<Self, ByteErr> {
                let buffer = bytes.take(size_of::<Self>()).collect::<Vec<u8>>();
        
                if buffer.len() != size_of::<Self>() {
                    // not enough bytes
                    return Err(ByteErr::NeedMoreBytes)
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
    use std::{fs, fs::File};
    use std::io::Read;

    macro_rules! number_test {
        ($type:ty) => {
            let test = <$type>::MIN;
            println!("test min: {}", test);
            assert_eq!(test, <$type>::from_bytes(&mut test.into_bytes()).unwrap());
            let test = 75;
            println!("test middle value: {}", test);
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

    #[test]
    fn read_write() -> Result<(), ByteErr> {
        let mut file = File::create("test.txt")?;
        let value: char = 'G'; 
        file.write_bytes(&mut value.into_bytes(), 1024)?;
        let file = File::open("test.txt")?;
        assert_eq!(value, char::from_io_bytes(&mut file.bytes())?);
        fs::remove_file("test.txt")?;

        Ok(())
    }
}
