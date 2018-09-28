use num_traits::Zero;
use std::convert::AsMut;
use std::default::Default;
use std::marker::PhantomData;
use std::mem::size_of;

use byteorder::{ByteOrder, LE};

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
        let top = u128::from(LE::read_u64(src));
        let bottom = u128::from(LE::read_u64(&src[size_of::<u64>()..]));

        (top << 64) | bottom
    }

    fn write(&self, dest: &mut [u8]) {
        let top = (*self >> 64) as u64;
        let bottom = *self as u64;
        LE::write_u64(dest, top);
        LE::write_u64(&mut dest[size_of::<u64>()..], bottom);
    }
}

#[derive(Clone)]
pub struct PcgSeeder<T> {
    data: Vec<u8>,
    at_pos: usize,
    _type: PhantomData<T>,
}

impl<T> AsMut<[u8]> for PcgSeeder<T> {
    fn as_mut(&mut self) -> &mut [u8] {
        self.data.as_mut()
    }
}

/*
 * The default seeds will control what seeds are used by `new_unseeded` calls.
 * These values were chosen at random.
 */
impl Default for PcgSeeder<u128> {
    fn default() -> Self {
        PcgSeeder::seed_with_stream(
            0xECC1_C32B_E531_D51A_93DC_E189_F916_29F4,
            0xF1CB_2035_E14F_F74B_46EF_3505_C538_6547,
        )
    }
}

impl Default for PcgSeeder<u64> {
    fn default() -> Self {
        PcgSeeder::seed_with_stream(0x1801_3CAD_3A48_3F72, 0x51DB_FCDA_0D6B_21D4)
    }
}

impl Default for PcgSeeder<u32> {
    fn default() -> Self {
        PcgSeeder::seed_with_stream(0x308A_20A0, 0xD133_51F1)
    }
}

impl Default for PcgSeeder<u16> {
    fn default() -> Self {
        PcgSeeder::seed_with_stream(0xAA19, 0x4FD8)
    }
}

impl Default for PcgSeeder<u8> {
    fn default() -> Self {
        PcgSeeder::seed_with_stream(0xE1, 0xB3)
    }
}

impl<T: Sized + ReadByteOrder + Zero> PcgSeeder<T> {
    pub fn seed(seed: T) -> PcgSeeder<T> {
        PcgSeeder::seed_with_stream(seed, T::zero())
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn seed_with_stream(seed: T, stream: T) -> PcgSeeder<T> {
        let mut data = vec![0; size_of::<T>() * 2];
        {
            let (seed_data, stream_data) = data.split_at_mut(size_of::<T>());
            seed.write(seed_data);
            stream.write(stream_data);
        }

        PcgSeeder {
            data,
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
