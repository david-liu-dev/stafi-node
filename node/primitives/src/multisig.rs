use parity_codec::{Encode, Decode};
#[cfg(feature = "std")]
use serde::{Serialize, Deserialize};
use rstd::prelude::*;

#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
#[derive(Encode, Decode, Copy, Clone, Eq, PartialEq)]
pub enum ChainType {
	TEZOS,
	COSMOS,
}

#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
#[derive(Encode, Decode, Clone, Eq, PartialEq)]
pub struct MultisigAddr {
	pub chain_type: ChainType,
	pub multisig_addr: Vec<u8>,
}