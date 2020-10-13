/*
 * PCG Random Number Generation for Rust
 *
 * Copyright 2015 John Brooks <jeb@robojeb.dev>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 * This work is derived from the implementation PCG RNG for C++ by
 * Melissa O'Neill.
 *
 * For additional information about the PCG random number generation scheme,
 * including its license and other licensing options, visit
 *
 *     http://www.pcg-random.org
 */

use crate::multiplier::Multiplier;
use crate::numops::BitSize;
use crate::outputmix::OutputMixin;
use crate::stream::{SpecificSeqStream, Stream};
use crate::PcgEngine;
use num_traits::Zero;
#[cfg(not(feature = "no_deserialize_verify"))]
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

impl<'de, Itype, Xtype, MulMix, OutMix> Deserialize<'de>
    for PcgEngine<Itype, Xtype, SpecificSeqStream<Itype>, MulMix, OutMix>
where
    Itype: Copy + Eq + Zero + BitSize + Deserialize<'de>,
    SpecificSeqStream<Itype>: Stream<Itype>,
    Xtype: BitSize,
    MulMix: Multiplier<Itype>,
    OutMix: OutputMixin<Itype, Xtype>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let state = crate::PCGStateInfo::deserialize(deserializer)?;

        #[cfg(feature = "no_deserialize_verify")]
        {
            Ok(Self::restore_state_with_no_verification(state))
        }
        #[cfg(not(feature = "no_deserialize_verify"))]
        {
            Self::restore_state(state).map_err(|e| Error::custom(e))
        }
    }
}

impl<Itype, Xtype, StreamMix, MulMix, OutMix> Serialize
    for PcgEngine<Itype, Xtype, StreamMix, MulMix, OutMix>
where
    Itype: Copy + BitSize + Serialize,
    Xtype: BitSize,
    StreamMix: Stream<Itype>,
    MulMix: Multiplier<Itype>,
    OutMix: OutputMixin<Itype, Xtype>,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let state = self.get_state();

        state.serialize(serializer)
    }
}
