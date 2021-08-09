use super::{IntoBytes, IntoBytesStatic, FromBytes, ByteErr};
use std::io::Error;

struct SliceByteIter<'a, T> {
    vec_iter: std::slice::Iter<'a, T>,
    data_iter: Box<dyn std::iter::Iterator<Item = u8> + 'a>,
}

impl<'a, T> SliceByteIter<'a, T> {
    fn new(vec: &'a [T]) -> Self {
        let len = vec.len() as u32;

        SliceByteIter {
            vec_iter: vec.iter(),
            data_iter: Box::new(len.into_bytes_static()),
        }
    }
}

impl<'a, T: IntoBytes<'a>> Iterator for SliceByteIter<'a, T> {
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

impl<'a, T: IntoBytes<'a>> IntoBytes<'a> for [T]{
    fn into_bytes(&'a self) -> Box<dyn Iterator<Item = u8> + 'a>{
        Box::new(SliceByteIter::new(self))
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