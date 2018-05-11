use std::mem::size_of;
use std::marker::PhantomData;
use std::convert::AsMut;

pub(crate) struct PCGSeeder<'a>{
    data: &'a [u8],
    at_pos: usize,
}

impl<'a> PCGSeeder<'a> {
    pub fn new(data: &'a [u8]) -> PCGSeeder<'a> {
        
    }
}


