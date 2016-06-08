pub trait ExtSize {
    fn ext_size() -> usize;
    fn ext_bits() -> usize;
}

macro_rules! make_ext_size {
    ($($i:ident, $size:expr, $bits:expr);*) => {
        pub struct $i;

        impl ExtSize for $i {
            fn ext_size() -> usize {
                $size
            }

            fn ext_bits() -> usize {
                $bits
            }
        }
    }
}

make_ext_size!(
    Ext2, 2, 1;
    Ext4, 4, 2;
    Ext8, 8, 3;
    Ext16, 16, 4;
    Ext32, 32, 5;
    Ext64, 64, 6;
    Ext128, 128, 7;
    Ext256, 256, 8;
    Ext512, 512, 9;
    Ext1024, 1024, 10
)