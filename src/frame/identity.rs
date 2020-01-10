//! Implements support for the identity module.

use crate::frame::{
    system::System,
    Call,
};
use codec::Encode;
/// Module name
pub const MODULE: &str = "Identity";

pub trait Identity: System {}
mod calls {
    pub const TEST_EXTRINSIC: &str = "test_extrinsic";
    pub const CREATE_REGISTRY: &str = "create_registry";
    pub const ADD_KYC_PROVIDER: &str = "add_kycprovider";
    pub const CREATE_DID: &str = "create_did";
    pub const CREATE_CLAIM: &str = "create_claim";
    pub const VERIFY_CLAIM: &str = "verify_claim";
    pub const REMOVE_VERIFICATION: &str = "remove_verification";
}
/// events
#[allow(unused)]
pub mod events {
    pub const EXTRINSIC_TEST: &str = "ExtrinsicTest";
    pub const CREATED: &str = "Created";
    pub const VERIFIED: &str = "Verified";
    pub const ADDED: &str = "Added";
    pub const REMOVED: &str = "Removed";
}
#[derive(Debug, Encode, Clone, Eq, PartialEq)]
pub enum ClaimDataType {
    Bool,
    String,
    U8,
    U16,
    U32,
    U128,
}
impl Default for ClaimDataType {
    fn default() -> Self {
        ClaimDataType::String
    }
}
#[derive(Debug, Encode, Clone, Eq, PartialEq, Default)]
pub struct ClaimDataValue {
    pub data_type: ClaimDataType,
    pub bool_value: Option<bool>,
    pub string_value: Option<Vec<u8>>,
    pub u8_value: Option<u8>,
    pub u16_value: Option<u16>,
    pub u32_value: Option<u32>,
    pub u128_value: Option<u128>,
}

/// Arguments extrinsic that does not need args
#[derive(Encode)]
pub struct EmptyArgs {}

pub fn test_extrinsic() -> Call<EmptyArgs> {
    Call::new(MODULE, calls::TEST_EXTRINSIC, EmptyArgs {})
}

/// call the extrinsic.
pub fn create_registry() -> Call<EmptyArgs> {
    Call::new(MODULE, calls::CREATE_REGISTRY, EmptyArgs {})
}
/// Arguments for creating a registry
#[derive(Encode)]
pub struct AddKycproviderArgs<T: Identity> {
    registry_id: <T as System>::Hash,
    kycprovider: <T as System>::AccountId,
}

/// call the extrinsic.
pub fn add_kycprovider<T: Identity>(
    registry_id: <T as System>::Hash,
    kycprovider: <T as System>::AccountId,
) -> Call<AddKycproviderArgs<T>> {
    Call::new(
        MODULE,
        calls::ADD_KYC_PROVIDER,
        AddKycproviderArgs {
            registry_id,
            kycprovider,
        },
    )
}
/// Arguments for creating a registry
#[derive(Encode)]
pub struct CreateDidArgs<T: Identity> {
    registry_id: <T as System>::Hash,
    referent: <T as System>::AccountId,
}

/// call the extrinsic.
pub fn create_did<T: Identity>(
    registry_id: <T as System>::Hash,
    referent: <T as System>::AccountId,
) -> Call<CreateDidArgs<T>> {
    Call::new(
        MODULE,
        calls::CREATE_DID,
        CreateDidArgs {
            registry_id,
            referent,
        },
    )
}
/// Arguments for creating a registry
#[derive(Encode)]
pub struct CreateClaimArgs<T: Identity> {
    did_id: <T as System>::Hash,
    issued: i64,
    expiry: i64,
    assertion: Vec<u8>,
    data_type: ClaimDataType,
    value: ClaimDataValue,
}

/// call the extrinsic.
pub fn create_claim<T: Identity>(
    did_id: <T as System>::Hash,
    issued: i64,
    expiry: i64,
    assertion: Vec<u8>,
    data_type: ClaimDataType,
    value: ClaimDataValue,
) -> Call<CreateClaimArgs<T>> {
    Call::new(
        MODULE,
        calls::CREATE_CLAIM,
        CreateClaimArgs {
            did_id,
            issued,
            expiry,
            assertion,
            data_type,
            value,
        },
    )
}
/// Arguments for creating a registry
#[derive(Encode)]
pub struct VerifyClaimArgs<T: Identity> {
    did_id: <T as System>::Hash,
    claim_id: <T as System>::Hash,
}

/// call the extrinsic.
pub fn verify_claim<T: Identity>(
    did_id: <T as System>::Hash,
    claim_id: <T as System>::Hash,
) -> Call<VerifyClaimArgs<T>> {
    Call::new(
        MODULE,
        calls::VERIFY_CLAIM,
        VerifyClaimArgs { did_id, claim_id },
    )
}

pub fn remove_verification<T: Identity>(
    did_id: <T as System>::Hash,
    claim_id: <T as System>::Hash,
) -> Call<VerifyClaimArgs<T>> {
    Call::new(
        MODULE,
        calls::REMOVE_VERIFICATION,
        VerifyClaimArgs { did_id, claim_id },
    )
}
