use super::{
    Attestation,
    Did,
    Fact,
};

use codec::{
    Decode,
    Encode,
};
use frame_support::dispatch::Vec;
use serde::{
    Deserialize,
    Serialize,
};
use sp_runtime::RuntimeDebug;

#[derive(Encode, Decode, Default, PartialOrd, Ord, PartialEq, Eq, Clone, RuntimeDebug)]
pub struct Claim<ClaimIndex, Timestamp> {
    /// Globally unique id
    pub id: ClaimIndex,
    /// A claim description
    pub description: Vec<u8>,
    /// Statements contained in this claim
    pub statements: Vec<Statement>,
    /// Claim consumer creates a claim
    pub created_by: Did,
    /// Attesttation by claim verifier
    pub attestation: Option<Attestation<Timestamp>>,
}

#[derive(
    Encode,
    Decode,
    Default,
    PartialOrd,
    Ord,
    PartialEq,
    Eq,
    Clone,
    RuntimeDebug,
    Serialize,
    Deserialize,
)]
pub struct Statement {
    /// Name of the property
    pub name: Vec<u8>,
    /// Fact in question
    pub fact: Fact,
    /// To be completed by verifier
    pub for_verifier: bool,
}
