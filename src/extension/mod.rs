pub mod extsizes;

use extsizes::*;

use extprim::u128::u128;
use rand::{Rng, Rand};
use std::marker::PhantomData;
use super::PcgEngine;
use super::numops::*;
use super::stream::*;
use super::outputmix::*;
use super::multiplier::*;


pub struct ExtPcg<Itype, Xtype, 
    StreamMix: Stream<Itype>, 
    MulMix: Multiplier<Itype>, 
    OutMix: OutputMixin<Itype, Xtype>, 
    Size: ExtSize> 
{
    pcg : PcgEngine<Itype, Xtype, StreamMix, MulMix, OutMix>,
    ext : Vec<Xtype>,
    _size : PhantomData<Size>,
}

impl<Itype, Xtype, StreamMix, MulMix, OutMix, Size> 
    ExtPcg<Itype, Xtype, StreamMix, MulMix, OutMix, Size> 
    where Itype: PcgOps + BitSize + AsSmaller<Xtype> + Clone, 
    Xtype: PcgOps + BitSize + Rand, 
    StreamMix: Stream<Itype>, 
    MulMix: Multiplier<Itype>, 
    OutMix: OutputMixin<Itype, Xtype>,
    Size: ExtSize {

    fn new_from_pcg(pcg: PcgEngine<Itype, Xtype, StreamMix, MulMix, OutMix>) ->
        ExtPcg<Itype, Xtype, StreamMix, MulMix, OutMix, Size> 
        where PcgEngine<Itype, Xtype, StreamMix, MulMix, OutMix>: Rng {
            let mut pcg = pcg;

            //Create the starting extension array
            let mut ext = Vec::new();
            for _ in 0..Size::ext_size() {
                ext.push(pcg.gen());
            }

            ExtPcg {
                pcg : pcg,
                ext : ext,
                _size : PhantomData::<Size>,
            }
    }

    fn new_unseeded() -> ExtPcg<Itype, Xtype, StreamMix, MulMix, OutMix, Size> 
    where PcgEngine<Itype, Xtype, StreamMix, MulMix, OutMix>: Rng
    {
        let pcg = PcgEngine::<Itype, Xtype, StreamMix, MulMix, OutMix>::new_unseeded();
        Self::new_from_pcg(pcg)
    }            
}

impl<Itype, StreamMix, MulMix, OutMix, Size> Rng for
    ExtPcg<Itype, u32, StreamMix, MulMix, OutMix, Size>
    where Itype: PcgOps + BitSize + AsSmaller<u32> + Clone, 
    StreamMix: Stream<Itype>, 
    MulMix: Multiplier<Itype>, 
    OutMix: OutputMixin<Itype, u32>,
    Size: ExtSize {

    #[inline]
    fn next_u32(&mut self) -> u32 {
        use std::mem::size_of;
        let oldstate = self.pcg.state.clone();
        self.pcg.state = self.pcg.stream_mix.increment().add(oldstate.mul(MulMix::multiplier()));
        let shift = size_of::<usize>() - Size::ext_bits();
        let pick = (self.pcg.state.usize() << shift) >> shift;

        let ext_val = self.ext[pick];
        self.ext[pick] = self.ext[pick] + 1;
        OutMix::output(oldstate) ^ ext_val
    }
}

impl<Itype, StreamMix, MulMix, OutMix, Size> Rng for
    ExtPcg<Itype, u64, StreamMix, MulMix, OutMix, Size>
    where Itype: PcgOps + BitSize + AsSmaller<u64> + Clone, 
    StreamMix: Stream<Itype>, 
    MulMix: Multiplier<Itype>, 
    OutMix: OutputMixin<Itype, u64>,
    Size: ExtSize {

    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        use std::mem::size_of;
        let oldstate = self.pcg.state.clone();
        self.pcg.state = self.pcg.stream_mix.increment().add(oldstate.mul(MulMix::multiplier()));
        let shift = size_of::<usize>() - Size::ext_bits();
        let pick = (self.pcg.state.usize() << shift) >> shift;

        let ext_val = self.ext[pick];
        self.ext[pick] = self.ext[pick] + 1;
        OutMix::output(oldstate) ^ ext_val
    }
}

pub type SetseqXshRr6432ext<Size> = ExtPcg<u64, u32, SpecificSeqStream<u64>, DefaultMultiplier, XshRrMixin, Size>;
pub type Pcg32Ext<Size> = SetseqXshRr6432ext<Size>;

pub type SetseqXshRr12832ext<Size> = ExtPcg<u128, u32, SpecificSeqStream<u128>, DefaultMultiplier, XshRrMixin, Size>;
pub type Pcg32LExt<Size> = SetseqXshRr12832ext<Size>;

pub type SetseqXshRr12864ext<Size> = ExtPcg<u128, u64, SpecificSeqStream<u128>, DefaultMultiplier, XshRrMixin, Size>;
pub type Pcg64Ext<Size> = SetseqXshRr12864ext<Size>;
