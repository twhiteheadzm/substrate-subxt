use super::fact::Fact;

use codec::{
    Decode,
    Encode,
};
use frame_support::dispatch::Vec;

#[derive(
    Encode, Decode, Default, PartialOrd, Ord, PartialEq, Eq, Clone, core::fmt::Debug,
)]
pub struct DidProperty {
    pub created: i64,
    pub name: Vec<u8>,
    pub fact: Fact,
}
