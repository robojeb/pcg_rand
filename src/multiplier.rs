pub trait Multiplier<Itype> {
    fn multiplier(&self) -> Itype;
}


pub struct DefaultMultiplier;

macro_rules! make_default_mul {
    ( $( $t:ty => $e:expr);* ) => {
        $(impl Multiplier<$t> for DefaultMultiplier {
            fn multiplier(&self) -> $t {
                $e
            }
        })*
    }
}


make_default_mul!(
    u8 => 141u8;
    u16 => 12829u16;
    u32 => 747796405u32;
    u64 => 6364136223846793005u64
);
