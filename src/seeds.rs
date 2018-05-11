use std::mem::size_of;
use std::marker::PhantomData;
use std::convert::AsMut;
use std::default::Default;
use num_traits::{Zero, One};

use byteorder::{LE, ByteOrder};

pub trait ReadByteOrder {
    fn read(src: &[u8]) -> Self;
    fn write(&self, dest: &mut [u8]);
}

impl ReadByteOrder for u8 {
    fn read(src: &[u8]) -> Self {
        src[0]
    }

    fn write(&self, dest: &mut [u8]) {
        dest[0] = *self;
    }
}

impl ReadByteOrder for u16 {
    fn read(src: &[u8]) -> Self {
        LE::read_u16(src)
    }

    fn write(&self, dest: &mut [u8]) {
        LE::write_u16(dest, *self);
    }
}

impl ReadByteOrder for u32 {
    fn read(src: &[u8]) -> Self {
        LE::read_u32(src)
    }

    fn write(&self, dest: &mut [u8]) {
        LE::write_u32(dest, *self);
    }
}

impl ReadByteOrder for u64 {
    fn read(src: &[u8]) -> Self {
        LE::read_u64(src)
    }

    fn write(&self, dest: &mut [u8]) {
        LE::write_u64(dest, *self);
    }
}

impl ReadByteOrder for u128 {
    fn read(src: &[u8]) -> Self {
        let top = LE::read_u64(src) as u128;
        let bottom = LE::read_u64(&src[size_of::<u64>()..]) as u128;

        (top << 64) | bottom
    }

    fn write(&self, dest: &mut [u8]) {
        let top = (*self >> 64) as u64;
        let bottom = *self as u64;
        LE::write_u64(dest, top);
        LE::write_u64(&mut dest[size_of::<u64>()..], bottom);
    }
}

pub struct PcgSeeder<T>{
    data: Vec<u8>,
    at_pos: usize,
    _type: PhantomData<T>
}

impl<T> AsMut<[u8]> for PcgSeeder<T> {
    fn as_mut(&mut self) -> &mut [u8] {
        self.data.as_mut()
    }
}

impl<T: Sized + ReadByteOrder + Zero + One> Default for PcgSeeder<T> {
    fn default() -> Self {
        PcgSeeder::seed_with_stream(T::zero(), T::one())
    }
}

impl<T: Sized + ReadByteOrder + Zero> PcgSeeder<T> {
    pub fn seed(seed: T) -> PcgSeeder<T> {
        PcgSeeder::seed_with_stream(seed, T::zero())
    }

    pub fn seed_with_stream(seed: T, stream: T) -> PcgSeeder<T> {
        let mut data = vec![0; size_of::<T>()*2];
        {
            let (seed_data, stream_data) = data.split_at_mut(size_of::<T>());
            seed.write(seed_data);
            stream.write(stream_data);
        }

        PcgSeeder {
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


