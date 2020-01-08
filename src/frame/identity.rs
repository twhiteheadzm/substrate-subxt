//! Implements support for the identity module.

use crate::frame::{
    balances::Balances,
    system::System,
    Call,
};
use codec::Encode;

const MODULE: &str = "Identity";

mod calls {
    pub const CREATE_REGISTRY: &str = "create_registry";
}

#[allow(unused)]
mod events {
    pub const CREATED: &str = "Created";
}
/// Arguments for creating a registry
#[derive(Encode)]
pub struct CreateRegistryArgs {
    name: Vec<u8>,
}

pub fn create_registry(name: Vec<u8>) -> Call<CreateRegistryArgs> {
    Call::new(MODULE, calls::CREATE_REGISTRY, CreateRegistryArgs { name })
}
