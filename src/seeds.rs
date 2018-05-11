use std::mem::size_of;
use std::marker::PhantomData;

use byteorder::{LE, ByteOrder};

pub trait ReadByteOrder {
    fn read(src: &[u8]) -> Self;
}

impl ReadByteOrder for u8 {
    fn read(src: &[u8]) -> Self {
        src[0]
    }
}

impl ReadByteOrder for u16 {
    fn read(src: &[u8]) -> Self {
        LE::read_u16(src)
    }
}

impl ReadByteOrder for u32 {
    fn read(src: &[u8]) -> Self {
        LE::read_u32(src)
    }
}

impl ReadByteOrder for u64 {
    fn read(src: &[u8]) -> Self {
        LE::read_u64(src)
    }
}

impl ReadByteOrder for u128 {
    fn read(src: &[u8]) -> Self {
        let top = LE::read_u64(src) as u128;
        let bottom = LE::read_u64(&src[size_of::<u64>()..]) as u128;

        (top << 64) | bottom
    }
}

pub struct PCGSeeder<'a, T>{
    data: &'a [u8],
    at_pos: usize,
    _type: PhantomData<T>
}

impl<'a, T: Sized + ReadByteOrder> PCGSeeder<'a, T> {
    pub fn new(data: &'a [u8]) -> PCGSeeder<'a, T> {
        PCGSeeder {
            data: data,
            at_pos: 0,
            _type: PhantomData,
        }
    }

    pub fn get(&mut self) -> T {
        //For now we panic if there aren't enough bytes
        if size_of::<T>() > (self.data.len() - self.at_pos) {
            panic!("Not enough bytes left in the seed");
        }

        let out = T::read(&self.data[self.at_pos..]);
        self.at_pos += size_of::<T>();
        out
    }
}


