use super::{
    did::Did,
    fact::Fact,
};
use codec::{
    Decode,
    Encode,
};
use frame_support::dispatch::Vec;

#[derive(
    Encode, Decode, Default, PartialOrd, Ord, PartialEq, Eq, Clone, core::fmt::Debug,
)]
pub struct Claim {
    /// unique id
    pub id: i64,
    /// Creation timestamp
    pub created: i64,
    /// Claim is only valid until expiry
    pub expiry: i64,
    /// Claim consumer creates a claim
    pub created_by: Option<Did>,
    /// Claim verifier attests a claim
    pub attested_by: Option<Did>,
    /// A claim description
    pub description: Vec<u8>,
    /// Statements contained in this claim
    pub statements: Vec<Statement>,
}

#[derive(
    Encode, Decode, Default, PartialOrd, Ord, PartialEq, Eq, Clone, core::fmt::Debug,
)]
pub struct Statement {
    /// Name of the property
    pub name: Vec<u8>,
    /// Fact in question
    pub fact: Fact,
}
