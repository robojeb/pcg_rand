
use rand::Rng;
use std::marker::PhantomData;
use super::PcgEngine;
use super::numops::PcgOps;

pub trait ExtSize {
    fn ext_size() -> usize;
    fn ext_bits() -> usize;
}

struct Ext1024;

impl ExtSize for Ext1024 {
    fn ext_size() -> usize {
        1024
    }
    fn ext_bits() -> usize {
        10
    }
}

pub struct ExtPcg<Itype, Xtype, StreamMix, MulMix, OutMix, Size> {
    pcg : PcgEngine<Itype, Xtype, StreamMix, MulMix, OutMix>,
    ext : Vec<Xtype>,
    _size : PhantomData<Size>,
}

impl ExtPcg<Itype, Xtype, StreamMix, MulMix, OutMix, Size> {
    fn new_from_pcg<Itype, Xtype, StreamMix, MulMix, OutMix, Size>
        (pcg: PcgEngine<Itype, Xtype, StreamMix, MulMix, OutMix>) ->
        ExtPcg<Itype, Xtype, StreamMix, MulMix, OutMix, Size>
        where Size: ExtSize {
            let mut pcg = pcg;

            //Create the starting extension array
            let mut ext = Vec::new();
            for _ in 0..Size::ext_size() {
                ext.push_back(pcg.gen());
            }

            ExtPcg {
                pcg : pcg,
                ext : ext,
                _size : PhantomData::<Size>,
            }
    }            
}

impl<Itype, Xtype, StreamMix, MulMix, OutMix, Size> Rng for
    ExtPcg<Itype, Xtype, StreamMix, MulMix, OutMix, Size>
    where Itype: PcgOps + BitSize + AsSmaller<Xtype> + Clone, Xtype: PcgOps + BitSize, StreamMix: Stream<Itype>, MulMix: Multiplier<Itype>, OutMix: OutputMixin<Itype, Xtype> {

    pub fn next_u32(&mut self) -> u32 {
        let oldstate = self.state.clone();
        self.state = self.stream_mix.increment().add(oldstate.mul(MulMix::multiplier()));
        let shift = std::mem::sizeof(usize) - Size::ext_bits();
        let pick = (self.state.usize() << shift) >> shift;

        let ext_val = self.ext[pick];
        self.ext[pick] = self.ext[pick].add(Xtype::one());
        OutMix::output(oldstate).xor(ext_val)
    }
