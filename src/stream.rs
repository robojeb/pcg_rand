pub trait Stream<Itype> {
    fn set_stream(&mut self, _stream_seq : Itype){
        panic!("Stream setting unimplemented for this stream type");
    }

    fn increment(&self) -> Itype;
}

//Definitions of the sequence types
pub struct OneSeqStream;

impl OneSeqStream {
    pub fn new() -> OneSeqStream {
        OneSeqStream
    }
}

macro_rules! make_one_seq {
    ( $( $t:ty => $e:expr);* ) => {
        $(impl Stream<$t> for OneSeqStream {
            fn increment(&self) -> $t {
                $e
            }
        })*
    }
}

pub struct SpecificSeqStream<Itype> {
    inc : Itype
}

macro_rules! specific_new {
    ( $($t:ty),*) => {
        $(impl SpecificSeqStream<$t> {
            pub fn new() -> SpecificSeqStream<$t> {
                SpecificSeqStream{inc: 0}
            }
        }

        )*
    }
}

specific_new!(u8, u16, u32, u64);



macro_rules! make_specific_seq {
    ( $($t:ty),* ) => {
        $(impl Stream<$t> for SpecificSeqStream<$t> {
            fn set_stream(&mut self, stream_seq : $t) {
                self.inc = (stream_seq << 1) | 1;
            }

            fn increment(&self) -> $t {
                self.inc
            }
        })*
    }
}


pub struct UniqueSeqStream;

impl UniqueSeqStream {
    pub fn new() -> UniqueSeqStream {
        UniqueSeqStream
    }
}

macro_rules! make_unique_seq {
    ( $($t:ty),* ) => {
        $(impl Stream<$t> for UniqueSeqStream {
            fn increment(&self) -> $t {
                let inc = self as *const UniqueSeqStream;
                inc as $t | 1
            }
        })*
    }
}


//For use with MCG
pub struct NoSeqStream;

impl NoSeqStream {
    pub fn new() -> NoSeqStream {
        NoSeqStream
    }
}

macro_rules! make_no_seq {
    ( $($t:ty),* ) => {
        $(impl Stream<$t> for NoSeqStream {
            fn increment(&self) -> $t {
                0
            }
        })*
    }
}

//Make the implementations for all the various sequence types
make_one_seq!(
    u8  => 77u8; //These are probably useless in rust
    u16 => 47989u16; // ^
    u32 => 2891336453u32;
    u64 => 1442695040888963407u64
);
make_specific_seq!(u8,u16,u32,u64);
make_unique_seq!(u8, u16, u32, u64);
make_no_seq!(u8, u16, u32, u64);
