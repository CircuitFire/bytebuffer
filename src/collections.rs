use super::{IntoBytes, IntoBytesStatic, FromBytes, ByteErr, ByteBuffer, MakeByteBuffer};
use std::io::Error;

struct VecByteIter<'a, T> {
    vec_iter: std::slice::Iter<'a, T>,
    data_iter: Box<dyn std::iter::Iterator<Item = u8> + 'a>,
}

impl<'a, T> VecByteIter<'a, T> {
    fn new(vec: &'a Vec<T>) -> Self {
        let len = vec.len() as u32;

        VecByteIter {
            vec_iter: vec.iter(),
            data_iter: Box::new(len.into_bytes_static()),
        }
    }
}

impl<'a, T: IntoBytes<'a>> Iterator for VecByteIter<'a, T> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item>{
        if let Some(value) = self.data_iter.next(){
            Some(value)
        }
        else {
            if let Some(node) = self.vec_iter.next(){
                self.data_iter = Box::new(node.into_bytes());
                self.data_iter.next()
            }
            else { None }
        }
    }
}

impl<'a, T: IntoBytes<'a>> IntoBytes<'a> for Vec<T>{
    fn into_bytes(&'a self) -> Box<dyn Iterator<Item = u8> + 'a>{
        Box::new(VecByteIter::new(self))
    }
}

impl<A: FromBytes> FromBytes for Vec<A>{
    fn from_bytes<T: Iterator<Item = u8>>(bytes: &mut T) -> Result<Self, ByteErr>{
        let len = u32::from_bytes(bytes)? as usize;
        let mut vec = Vec::with_capacity(len);

        for _ in 0..len{
            vec.push(A::from_bytes(bytes)?);
        }

        Ok(vec)
    }

    fn from_io_bytes<T: Iterator<Item = Result<u8, Error>>>(bytes: &mut T) -> Result<Self, ByteErr>{
        let len = u32::from_io_bytes(bytes)? as usize;
        let mut vec = Vec::with_capacity(len);

        for _ in 0..len{
            vec.push(A::from_io_bytes(bytes)?);
        }

        Ok(vec)
    }
}

impl<'a> IntoBytes<'a> for String{
    fn into_bytes(&'a self) -> Box<dyn Iterator<Item = u8> + 'a>{
        Box::new(self.bytes())
    }
}

impl FromBytes for String{
    fn from_bytes<T: Iterator<Item = u8>>(bytes: &mut T) -> Result<Self, ByteErr>{
        let vec = Vec::<u8>::from_bytes(bytes)?;

        Ok(String::from_utf8_lossy(&vec).to_string())
    }

    fn from_io_bytes<T: Iterator<Item = Result<u8, Error>>>(bytes: &mut T) -> Result<Self, ByteErr>{
        let vec = Vec::<u8>::from_io_bytes(bytes)?;

        Ok(String::from_utf8_lossy(&vec).to_string())
    }
}