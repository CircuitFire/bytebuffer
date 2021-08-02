use super::{IntoBytesStatic, FromBytes, ByteErr, ByteBuffer, MakeByteBuffer};
use std::{
    mem::size_of,
    convert::TryInto,
    io::Error,
};

impl IntoBytesStatic for bool{
    fn into_bytes_static(&self) -> Box<dyn Iterator<Item = u8>>{
        ByteBuffer::new([*self as u8])
    }
}

impl FromBytes for bool{
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

impl IntoBytesStatic for char{

    fn into_bytes_static(&self) -> Box<dyn Iterator<Item = u8>>{
        let temp = *self as u32;
        ByteBuffer::new(temp.to_le_bytes())
    }
}

impl FromBytes for char{
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

impl IntoBytesStatic for u8{
    fn into_bytes_static(&self) -> Box<dyn Iterator<Item = u8>>{
        ByteBuffer::new([*self])
    }
}

impl FromBytes for u8{
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
        impl IntoBytesStatic for $type{

            fn into_bytes_static(&self) -> Box<dyn Iterator<Item = u8>>{
                ByteBuffer::new(self.to_le_bytes())
            }
        }

        impl FromBytes for $type{

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
    use crate::*;
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