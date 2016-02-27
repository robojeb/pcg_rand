use ::Pcg32;

use serde;
use serde::{Serializer, Deserializer};
use ::stream::Stream;


struct Pcg32MapVisitor {
    state  : u64,
    stream : u64,
    count  : u8,
}

impl serde::ser::MapVisitor for Pcg32MapVisitor {
    fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
        where S: serde::Serializer
    {
        match self.count {
            0 => {
                self.count += 1;
                Ok(Some(try!(serializer.serialize_struct_elt("state", &self.state))))
            }
            1 => {
                self.count += 1;
                Ok(Some(try!(serializer.serialize_struct_elt("stream", &self.stream))))
            }
            _ => {
                Ok(None)
            }
        }
    }
}

impl serde::ser::Serialize for Pcg32 {
    fn serialize<S: Serializer>(&self, ser: &mut S) -> Result<(), S::Error> {
        ser.serialize_struct("Pcg32", Pcg32MapVisitor {
            state  : self.state,
            stream : self.stream_mix.get_stream(),
            count  : 0,
        })
    }
}

enum Pcg32Field {
    STATE,
    STREAM,
}

impl serde::Deserialize for Pcg32Field {
    fn deserialize<D>(deserializer: &mut D) -> Result<Pcg32Field, D::Error>
        where D: serde::de::Deserializer
    {
        struct Pcg32FieldVisitor;

        impl serde::de::Visitor for Pcg32FieldVisitor {
            type Value = Pcg32Field;

            fn visit_str<E>(&mut self, value: &str) -> Result<Pcg32Field, E>
                where E: serde::de::Error
            {
                match value {
                    "state" => Ok(Pcg32Field::STATE),
                    "stream" => Ok(Pcg32Field::STREAM),
                    _ => Err(serde::de::Error::custom("expected state or stream")),
                }
            }
        }

        deserializer.deserialize(Pcg32FieldVisitor)
    }
}

impl serde::de::Deserialize for Pcg32 {
    fn deserialize<D: Deserializer>(des: &mut D) -> Result<Self, D::Error> where Self: Sized {
        static FIELDS: &'static [&'static str] = &["state", "stream"];
        des.deserialize_struct("Pcg32", FIELDS, Pcg32Visitor)
    }
}

struct Pcg32Visitor;

impl serde::de::Visitor for Pcg32Visitor {
    type Value = Pcg32;

    fn visit_map<V>(&mut self, mut visitor: V) -> Result<Pcg32, V::Error>
        where V: serde::de::MapVisitor
    {
        let mut state = None;
        let mut stream = None;

        loop {
            match try!(visitor.visit_key()) {
                Some(Pcg32Field::STATE) => { state = Some(try!(visitor.visit_value())); }
                Some(Pcg32Field::STREAM) => { stream = Some(try!(visitor.visit_value())); }
                None => { break; }
            }
        }

        let state = match state {
            Some(state) => state,
            None => try!(visitor.missing_field("state")),
        };

        let stream = match stream {
            Some(stream) => stream,
            None => try!(visitor.missing_field("stream")),
        };

        try!(visitor.end());

        Ok(Pcg32::new_from_state([state, stream]))
    }
}
