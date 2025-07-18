use alloy_primitives::B256;
use ream_bls::{BLSSignature, PublicKey};
use serde::{Deserialize, Serialize};
use ssz_derive::{Decode, Encode};
use tree_hash_derive::TreeHash;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Encode, Decode, TreeHash)]
pub struct DepositData {
    #[serde(rename = "pubkey")]
    pub public_key: PublicKey,
    pub withdrawal_credentials: B256,
    #[serde(with = "serde_utils::quoted_u64")]
    pub amount: u64,

    /// BLS aggregate signature
    pub signature: BLSSignature,
}
