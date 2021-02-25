use sp_std::prelude::*;
use codec::{Decode, Encode};
use sp_runtime::RuntimeDebug;
use node_primitives::{RSymbol};

/// Rtoken Identifier
#[derive(Encode, Decode, Copy, Clone, Eq, PartialEq, RuntimeDebug)]
pub enum BondReason {
    /// Pass
    Pass,
	/// blockhash
	BlockhashUnmatch,
	/// txhash
    TxhashUnmatch,
    /// from not match
    PubkeyUnmatch,
    /// to not match
    PoolUnmatch,
    /// amount not match
    AmountUnmatch,
}

#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug)]
pub struct BondRecord<AccountId> {
    pub bonder: AccountId,
    pub symbol: RSymbol,
    pub pubkey: Vec<u8>,
    pub pool: Vec<u8>,
    pub blockhash: Vec<u8>,
    pub txhash: Vec<u8>,
    pub amount: u128,
}

impl<A: PartialEq> BondRecord<A> {
    pub fn new(bonder: A, symbol: RSymbol, pubkey: Vec<u8>, pool: Vec<u8>, blockhash: Vec<u8>, txhash: Vec<u8>, amount: u128) -> Self {
        Self {
            bonder: bonder,
            symbol: symbol,
            pubkey: pubkey,
            pool: pool,
            blockhash: blockhash,
            txhash: txhash,
            amount: amount,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug)]
pub struct BondKey<Hash> {
    pub symbol: RSymbol,
    pub bond_id: Hash,
}

impl<A: PartialEq> BondKey<A> {
    pub fn new(symbol: RSymbol, bond_id: A) -> Self {
        Self {
            symbol: symbol,
            bond_id: bond_id,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug)]
pub struct LinkChunk {
    /// Pool
	pub pool: Vec<u8>,
	/// Total bond amount
	#[codec(compact)]
	pub bond_value: u128,
	/// Total unbond amount
	#[codec(compact)]
	pub unbond_value: u128,
}

#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug)]
pub struct BondUnlockChunk {
	/// Amount of funds to be unlocked.
	#[codec(compact)]
	pub value: u128,
	/// Era number at which point it'll be unlocked.
	#[codec(compact)]
	pub era: u32,
}

#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug)]
pub struct TotalUnlockChunk {
	/// Total amount of funds to be unlocked.
	#[codec(compact)]
	pub value: u128,
	/// Pool address from which it'll be unlocked.
	pub pool: Vec<u8>,
}

#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug)]
pub struct WithdrawChunk<AccountId> {
    pub who: AccountId,
    /// Pool address from which it'll be withdrawn.
	pub pool: Vec<u8>,
	/// The recipient account.
	pub recipient: Vec<u8>,
	/// Amount of funds exposed.
	#[codec(compact)]
	pub value: u128,
}
