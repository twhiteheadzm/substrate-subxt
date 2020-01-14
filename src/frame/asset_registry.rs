//! Implements support for the AssetRegistry module.

use crate::frame::{
    system::System,
    Call,
};
use codec::Encode;
/// Module name
pub const MODULE: &str = "AssetRegistry";

pub trait AssetRegistry: System {}
mod calls {
    pub const CREATE_REGISTRY: &str = "create_registry";
    pub const ADD_KYC_PROVIDER: &str = "add_kycprovider";
    pub const CREATE_ASSET: &str = "create_asset";
    pub const CREATE_ASSET_PROPERTY: &str = "create_asset_property";
    pub const VERIFY_ASSET_PROPERTY: &str = "verify_asset_property";
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
    pub const TRANSFERED: &str = "Transfered";
}
#[derive(Debug, Encode, Clone, Eq, PartialEq)]
pub enum PropertyDataType {
    Bool,
    String,
    U8,
    U16,
    U32,
    U128,
}
impl Default for PropertyDataType {
    fn default() -> Self {
        PropertyDataType::String
    }
}
#[derive(Debug, Encode, Clone, Eq, PartialEq, Default)]
pub struct PropertyDataValue {
    pub data_type: PropertyDataType,
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

/// call the extrinsic.
pub fn create_registry() -> Call<EmptyArgs> {
    Call::new(MODULE, calls::CREATE_REGISTRY, EmptyArgs {})
}
/// Arguments for creating a registry
#[derive(Encode)]
pub struct AddKycproviderArgs<T: AssetRegistry> {
    registry_id: <T as System>::Hash,
    kycprovider: <T as System>::AccountId,
}

/// call the extrinsic.
pub fn add_kycprovider<T: AssetRegistry>(
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
pub struct CreateDidArgs<T: AssetRegistry> {
    registry_id: <T as System>::Hash,
    owner: <T as System>::AccountId,
}

/// call the extrinsic.
pub fn create_asset<T: AssetRegistry>(
    registry_id: <T as System>::Hash,
    owner: <T as System>::AccountId,
) -> Call<CreateDidArgs<T>> {
    Call::new(
        MODULE,
        calls::CREATE_ASSET,
        CreateDidArgs { registry_id, owner },
    )
}
/// Arguments for creating a registry
#[derive(Encode)]
pub struct CreatePropertyArgs<T: AssetRegistry> {
    asset_id: <T as System>::Hash,
    issued: i64,
    expiry: i64,
    assertion: Vec<u8>,
    data_type: PropertyDataType,
    value: PropertyDataValue,
}

/// call the extrinsic.
pub fn create_asset_property<T: AssetRegistry>(
    asset_id: <T as System>::Hash,
    issued: i64,
    expiry: i64,
    assertion: Vec<u8>,
    data_type: PropertyDataType,
    value: PropertyDataValue,
) -> Call<CreatePropertyArgs<T>> {
    Call::new(
        MODULE,
        calls::CREATE_ASSET_PROPERTY,
        CreatePropertyArgs {
            asset_id,
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
pub struct VerifyPropertyArgs<T: AssetRegistry> {
    asset_id: <T as System>::Hash,
    property_id: <T as System>::Hash,
}

/// call the extrinsic.
pub fn verify_asset_property<T: AssetRegistry>(
    asset_id: <T as System>::Hash,
    property_id: <T as System>::Hash,
) -> Call<VerifyPropertyArgs<T>> {
    Call::new(
        MODULE,
        calls::VERIFY_ASSET_PROPERTY,
        VerifyPropertyArgs {
            asset_id,
            property_id,
        },
    )
}

pub fn remove_verification<T: AssetRegistry>(
    asset_id: <T as System>::Hash,
    property_id: <T as System>::Hash,
) -> Call<VerifyPropertyArgs<T>> {
    Call::new(
        MODULE,
        calls::REMOVE_VERIFICATION,
        VerifyPropertyArgs {
            asset_id,
            property_id,
        },
    )
}
