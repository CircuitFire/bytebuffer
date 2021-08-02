//! # ByteBuffer
//! Allows for easy conversion of data to and from u8 iters.
//! 
//! ## Structs
//! - ByteBuffer
//! 
//! ## Enums
//! - ByteErr
//! 
//! ## Traits
//! - IntoBytes
//! - IntoBytesStatic
//! - FromBytes
//! - WriteBytes


use std::io::{Write, Error};

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

/// The IntoBytesStatic trait is the same as Into bytes except that is iter is 'static.
pub trait IntoBytesStatic{
    fn into_bytes_static(&self) -> Box<dyn Iterator<Item = u8>>;
}

impl<'a, T: IntoBytesStatic> IntoBytes<'a> for T {
    fn into_bytes(&'a self) -> Box<dyn Iterator<Item = u8> + 'a>{
        self.into_bytes_static()
    }
}

/// The IntoBytes trait allows for the easy conversion of data to u8 iters.
pub trait IntoBytes<'a>{
    /// Creates an iter over the bytes of self.
    fn into_bytes(&'a self) -> Box<dyn Iterator<Item = u8> + 'a>;
}

/// The IntoBytes trait allows for the easy conversion of u8 iters into data.
pub trait FromBytes{
    /// Tries to create the indicated type out of an io byte iter.
    fn from_io_bytes<T: Iterator<Item = Result<u8, Error>>>(bytes: &mut T) -> Result<Self, ByteErr> where Self: Sized;

    /// Tries to create the indicated type out of a byte iter.
    fn from_bytes<T: Iterator<Item = u8>>(bytes: &mut T) -> Result<Self, ByteErr> where Self: Sized;
}

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

impl<T: Write> WriteBytes for T {}

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
impl_byte_buffer!(Vec<u8>);

mod prims;
pub use prims::*;

mod collections;
pub use collections::*;