use std::mem;
use std::convert::TryInto;

pub trait Buffer{
    fn into_buffer(&self, buffer: &mut [u8]);

    fn as_buffer(&self) -> Box<[u8]>;

    fn buffer_len(&self) -> usize;

    fn from_buffer(buffer: &[u8]) -> Self;
}

impl Buffer for bool{
    fn into_buffer(&self, buffer: &mut [u8]){
        buffer[0] = *self as u8;
    }

    fn as_buffer(&self) -> Box<[u8]>{
        Box::new([*self as u8])
    }

    fn buffer_len(&self) -> usize {
        mem::size_of::<Self>()
    }

    fn from_buffer(buffer: &[u8]) -> Self {
        let temp = [buffer[0]];
        temp[0] != 0
    }
}

impl Buffer for u8{
    fn into_buffer(&self, buffer: &mut [u8]){
        buffer[0] = *self;
    }

    fn as_buffer(&self) -> Box<[u8]>{
        Box::new([*self])
    }

    fn buffer_len(&self) -> usize {
        mem::size_of::<Self>()
    }

    fn from_buffer(buffer: &[u8]) -> Self {
        let temp = [buffer[0]];
        temp[0]
    }
}

impl Buffer for u16{
    fn into_buffer(&self, buffer: &mut [u8]){
        buffer.clone_from_slice(&self.to_le_bytes());
    }

    fn as_buffer(&self) -> Box<[u8]>{
        Box::new(self.to_le_bytes())
    }

    fn buffer_len(&self) -> usize {
        mem::size_of::<Self>()
    }

    fn from_buffer(buffer: &[u8]) -> Self {
        let temp = [buffer[0], buffer[1]];
        Self::from_le_bytes(temp.try_into().unwrap())
    }
}

impl Buffer for u32{
    fn into_buffer(&self, buffer: &mut [u8]){
        buffer.clone_from_slice(&self.to_le_bytes());
    }

    fn as_buffer(&self) -> Box<[u8]>{
        Box::new(self.to_le_bytes())
    }

    fn buffer_len(&self) -> usize {
        mem::size_of::<Self>()
    }

    fn from_buffer(buffer: &[u8]) -> Self {
        let temp = [buffer[0], buffer[1], buffer[2], buffer[3]];
        Self::from_le_bytes(temp.try_into().unwrap())
    }
}

impl Buffer for u64{
    fn into_buffer(&self, buffer: &mut [u8]){
        buffer.clone_from_slice(&self.to_le_bytes());
    }

    fn as_buffer(&self) -> Box<[u8]>{
        Box::new(self.to_le_bytes())
    }

    fn buffer_len(&self) -> usize {
        mem::size_of::<Self>()
    }

    fn from_buffer(buffer: &[u8]) -> Self {
        let temp = [buffer[0], buffer[1], buffer[2], buffer[3], buffer[4], buffer[5], buffer[6], buffer[7]];
        Self::from_le_bytes(temp.try_into().unwrap())
    }
}

impl Buffer for u128{
    fn into_buffer(&self, buffer: &mut [u8]){
        buffer.clone_from_slice(&self.to_le_bytes());
    }

    fn as_buffer(&self) -> Box<[u8]>{
        Box::new(self.to_le_bytes())
    }

    fn buffer_len(&self) -> usize {
        mem::size_of::<Self>()
    }

    fn from_buffer(buffer: &[u8]) -> Self {
        let temp = [
            buffer[0], buffer[1], buffer[2],  buffer[3],  buffer[4],  buffer[5],  buffer[6],  buffer[7],
            buffer[8], buffer[9], buffer[10], buffer[11], buffer[12], buffer[13], buffer[14], buffer[15]
        ];
        Self::from_le_bytes(temp.try_into().unwrap())
    }
}

impl Buffer for i8{
    fn into_buffer(&self, buffer: &mut [u8]){
        buffer.clone_from_slice(&self.to_le_bytes());
    }

    fn as_buffer(&self) -> Box<[u8]>{
        Box::new(self.to_le_bytes())
    }

    fn buffer_len(&self) -> usize {
        mem::size_of::<Self>()
    }

    fn from_buffer(buffer: &[u8]) -> Self {
        let temp = [buffer[0]];
        Self::from_le_bytes(temp.try_into().unwrap())
    }
}

impl Buffer for i16{
    fn into_buffer(&self, buffer: &mut [u8]){
        buffer.clone_from_slice(&self.to_le_bytes());
    }

    fn as_buffer(&self) -> Box<[u8]>{
        Box::new(self.to_le_bytes())
    }

    fn buffer_len(&self) -> usize {
        mem::size_of::<Self>()
    }

    fn from_buffer(buffer: &[u8]) -> Self {
        let temp = [buffer[0], buffer[1]];
        Self::from_le_bytes(temp.try_into().unwrap())
    }
}

impl Buffer for i32{
    fn into_buffer(&self, buffer: &mut [u8]){
        buffer.clone_from_slice(&self.to_le_bytes());
    }

    fn as_buffer(&self) -> Box<[u8]>{
        Box::new(self.to_le_bytes())
    }

    fn buffer_len(&self) -> usize {
        mem::size_of::<Self>()
    }

    fn from_buffer(buffer: &[u8]) -> Self {
        let temp = [buffer[0], buffer[1], buffer[2], buffer[3]];
        Self::from_le_bytes(temp.try_into().unwrap())
    }
}

impl Buffer for i64{
    fn into_buffer(&self, buffer: &mut [u8]){
        buffer.clone_from_slice(&self.to_le_bytes());
    }

    fn as_buffer(&self) -> Box<[u8]>{
        Box::new(self.to_le_bytes())
    }

    fn buffer_len(&self) -> usize {
        mem::size_of::<Self>()
    }

    fn from_buffer(buffer: &[u8]) -> Self {
        let temp = [buffer[0], buffer[1], buffer[2], buffer[3], buffer[4], buffer[5], buffer[6], buffer[7]];
        Self::from_le_bytes(temp.try_into().unwrap())
    }
}

impl Buffer for i128{
    fn into_buffer(&self, buffer: &mut [u8]){
        buffer.clone_from_slice(&self.to_le_bytes());
    }

    fn as_buffer(&self) -> Box<[u8]>{
        Box::new(self.to_le_bytes())
    }

    fn buffer_len(&self) -> usize {
        mem::size_of::<Self>()
    }

    fn from_buffer(buffer: &[u8]) -> Self {
        let temp = [
            buffer[0], buffer[1], buffer[2],  buffer[3],  buffer[4],  buffer[5],  buffer[6],  buffer[7],
            buffer[8], buffer[9], buffer[10], buffer[11], buffer[12], buffer[13], buffer[14], buffer[15]
        ];
        Self::from_le_bytes(temp.try_into().unwrap())
    }
}

impl Buffer for char{
    fn into_buffer(&self, buffer: &mut [u8]){
        let temp = *self as u32;
        buffer.clone_from_slice(&temp.to_le_bytes());
    }

    fn as_buffer(&self) -> Box<[u8]>{
        let temp = *self as u32;
        Box::new(temp.to_le_bytes())
    }

    fn buffer_len(&self) -> usize {
        mem::size_of::<Self>()
    }

    fn from_buffer(buffer: &[u8]) -> Self {
        let temp = u32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
        std::char::from_u32(temp).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bool_buff() {
        let test = true;
        assert_eq!(test, bool::from_buffer(&test.as_buffer()));
        let test = false;
        assert_eq!(test, bool::from_buffer(&test.as_buffer()));
    }

    #[test]
    fn u8_buff() {
        let test = u8::MIN;
        assert_eq!(test, u8::from_buffer(&test.as_buffer()));
        let test = u8::MAX;
        assert_eq!(test, u8::from_buffer(&test.as_buffer()));
    }

    #[test]
    fn u16_buff() {
        let test = u16::MIN;
        assert_eq!(test, u16::from_buffer(&test.as_buffer()));
        let test = u16::MAX;
        assert_eq!(test, u16::from_buffer(&test.as_buffer()));
    }

    #[test]
    fn u32_buff() {
        let test = u32::MIN;
        assert_eq!(test, u32::from_buffer(&test.as_buffer()));
        let test = u32::MAX;
        assert_eq!(test, u32::from_buffer(&test.as_buffer()));
    }

    #[test]
    fn u64_buff() {
        let test = u64::MIN;
        assert_eq!(test, u64::from_buffer(&test.as_buffer()));
        let test = u64::MAX;
        assert_eq!(test, u64::from_buffer(&test.as_buffer()));
    }

    #[test]
    fn u128_buff() {
        let test = u128::MIN;
        assert_eq!(test, u128::from_buffer(&test.as_buffer()));
        let test = u128::MAX;
        assert_eq!(test, u128::from_buffer(&test.as_buffer()));
    }

    #[test]
    fn i8_buff() {
        let test = i8::MIN;
        assert_eq!(test, i8::from_buffer(&test.as_buffer()));
        let test = i8::MAX;
        assert_eq!(test, i8::from_buffer(&test.as_buffer()));
    }

    #[test]
    fn i16_buff() {
        let test = i16::MIN;
        assert_eq!(test, i16::from_buffer(&test.as_buffer()));
        let test = i16::MAX;
        assert_eq!(test, i16::from_buffer(&test.as_buffer()));
    }

    #[test]
    fn i32_buff() {
        let test = i32::MIN;
        assert_eq!(test, i32::from_buffer(&test.as_buffer()));
        let test = i32::MAX;
        assert_eq!(test, i32::from_buffer(&test.as_buffer()));
    }

    #[test]
    fn i64_buff() {
        let test = i64::MIN;
        assert_eq!(test, i64::from_buffer(&test.as_buffer()));
        let test = i64::MAX;
        assert_eq!(test, i64::from_buffer(&test.as_buffer()));
    }

    #[test]
    fn i128_buff() {
        let test = i128::MIN;
        assert_eq!(test, i128::from_buffer(&test.as_buffer()));
        let test = i128::MAX;
        assert_eq!(test, i128::from_buffer(&test.as_buffer()));
    }

    #[test]
    fn char_buff() {
        let test = 't';
        assert_eq!(test, char::from_buffer(&test.as_buffer()));
    }
}