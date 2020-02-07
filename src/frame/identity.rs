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
    pub const CREATE_CATALOG: &str = "create_catalog";
    pub const CREATE_DID: &str = "create_did";
    pub const ADD_DID: &str = "add_did";
    pub const CREATE_PROPERTY: &str = "create_property";
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
pub enum DataType {
    Bool,
    String,
    U8,
    U16,
    U32,
    U128,
}
impl Default for DataType {
    fn default() -> Self {
        DataType::String
    }
}
#[derive(Debug, Encode, Clone, Eq, PartialEq, Default)]
pub struct DataValue {
    pub data_type: DataType,
    pub bool_value: Option<bool>,
    pub string_value: Option<Vec<u8>>,
    pub u8_value: Option<u8>,
    pub u16_value: Option<u16>,
    pub u32_value: Option<u32>,
    pub u128_value: Option<u128>,
}

#[derive(Encode)]
pub struct EmptyArgs {}

pub fn test_extrinsic() -> Call<EmptyArgs> {
    Call::new(MODULE, calls::TEST_EXTRINSIC, EmptyArgs {})
}

pub fn create_catalog() -> Call<EmptyArgs> {
    Call::new(MODULE, calls::CREATE_CATALOG, EmptyArgs {})
}

#[derive(Encode)]
pub struct CreateDidArgs<T: Identity> {
    catalog_id: Option<<T as System>::Hash>,
    name: Vec<u8>,
    subject: <T as System>::AccountId,
}

pub fn create_did<T: Identity>(
    catalog_id: Option<<T as System>::Hash>,
    name: Vec<u8>,
    subject: <T as System>::AccountId,
) -> Call<CreateDidArgs<T>> {
    Call::new(
        MODULE,
        calls::CREATE_DID,
        CreateDidArgs {
            catalog_id,
            name,
            subject,
        },
    )
}

#[derive(Encode)]
pub struct AddDidArgs<T: Identity> {
    catalog_id: <T as System>::Hash,
    did: <T as System>::Hash,
    name: Vec<u8>,
}

pub fn add_did<T: Identity>(
    catalog_id: <T as System>::Hash,
    did: <T as System>::Hash,
    name: Vec<u8>,
) -> Call<AddDidArgs<T>> {
    Call::new(
        MODULE,
        calls::ADD_DID,
        AddDidArgs {
            catalog_id,
            did,
            name,
        },
    )
}
#[derive(Encode)]
pub struct CreatePropertyArgs<T: Identity> {
    did_id: <T as System>::Hash,
    created: i64,
    name: Vec<u8>,
    data_type: DataType,
    value: DataValue,
}

pub fn create_property<T: Identity>(
    did_id: <T as System>::Hash,
    created: i64,
    name: Vec<u8>,
    data_type: DataType,
    value: DataValue,
) -> Call<CreatePropertyArgs<T>> {
    Call::new(
        MODULE,
        calls::CREATE_PROPERTY,
        CreatePropertyArgs {
            did_id,
            created,
            name,
            data_type,
            value,
        },
    )
}

#[derive(Encode)]
pub struct CreateClaimArgs<T: Identity> {
    did_id: <T as System>::Hash,
    issued: i64,
    expiry: i64,
    assertion: Vec<u8>,
    data_type: DataType,
    value: DataValue,
}

pub fn create_claim<T: Identity>(
    did_id: <T as System>::Hash,
    issued: i64,
    expiry: i64,
    assertion: Vec<u8>,
    data_type: DataType,
    value: DataValue,
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

#[derive(Encode)]
pub struct VerifyClaimArgs<T: Identity> {
    did_id: <T as System>::Hash,
    claim_id: <T as System>::Hash,
    issued: i64,
    expiry: i64,
}

pub fn verify_claim<T: Identity>(
    did_id: <T as System>::Hash,
    claim_id: <T as System>::Hash,
    issued: i64,
    expiry: i64,
) -> Call<VerifyClaimArgs<T>> {
    Call::new(
        MODULE,
        calls::VERIFY_CLAIM,
        VerifyClaimArgs {
            did_id,
            claim_id,
            issued,
            expiry,
        },
    )
}

#[derive(Encode)]
pub struct RemoveVerificationArgs<T: Identity> {
    did_id: <T as System>::Hash,
    claim_id: <T as System>::Hash,
}

pub fn remove_verification<T: Identity>(
    did_id: <T as System>::Hash,
    claim_id: <T as System>::Hash,
) -> Call<RemoveVerificationArgs<T>> {
    Call::new(
        MODULE,
        calls::REMOVE_VERIFICATION,
        RemoveVerificationArgs { did_id, claim_id },
    )
}
