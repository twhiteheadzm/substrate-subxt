//! Implements support for the identity module.
pub mod attestation;
pub mod claim;
pub mod did;
pub mod did_property;
pub mod fact;

use crate::frame::{
    system::System,
    Call,
};
use attestation::Attestation;
use claim::Statement;
use codec::Encode;
use did::Did;
use did_property::DidProperty;
use fact::Fact;
use std::convert::TryFrom;

/// Module name
pub const MODULE: &str = "Identity";

pub type CatalogId = u32;
pub struct AddaxTimestamp(u64);

pub trait Identity: System {}
mod calls {
    pub const REGISTER_DID: &str = "register_did";
    pub const REGISTER_DID_FOR: &str = "register_did_for";
    pub const REGISTER_DID_CATALOG: &str = "register_did_catalog";
    pub const ADD_DID_PROPERTIES: &str = "add_did_properties";
    pub const PATCH_DID_PROPERTIES: &str = "patch_did_properties";
    pub const UPDATE_DID_PROPERTIES: &str = "update_did_properties";
    pub const UPDATE_DID_CONTROLLERS: &str = "update_did_controllers";
    pub const AUTHORIZE_CLAIM_CONSUMER: &str = "authorize_claim_consumer";
    pub const AUTHORIZE_CLAIM_VERIFIER: &str = "authorize_claim_verifier";
    pub const MAKE_CLAIM: &str = "make_claim";
    pub const ATTEST_CLAIM: &str = "attest_claim";
    pub const REVOKE_CLAIM: &str = "revoke_claim";
    pub const CREATE_CATALOG: &str = "create_catalog";
    pub const DELETE_CATALOG: &str = "delete_catalog";
    pub const CATALOG_ADD_DID: &str = "catalog_add_did";
}
/// events
#[allow(unused)]
pub mod events {
    pub const REGISTERED: &str = "Registered";
    pub const PROPERTIES_ADDED: &str = "PropertiesAdded";
    pub const PROPERTIES_UPDATED: &str = "PropertiesUpdated";
    pub const PROPERTIES_REPLACED: &str = "PropertiesReplaced";
    pub const DID_CONTROLLERS_UPDATED: &str = "ControllersUpdated";
    pub const CLAIM_CONSUMERS_ADDED: &str = "ClaimConsumersAdded";
    pub const CLAIM_ISSUERS_ADDED: &str = "ClaimIssuersAdded";
    pub const CLAIM_CONSUMERS_REVOKED: &str = "ClaimConsumersRevoked";
    pub const CLAIM_ISSUERS_REVOKED: &str = "ClaimIssuersRevoked";
    pub const CLAIM_MADE: &str = "ClaimMade";
    pub const CLAIM_ATTESTED: &str = "ClaimAttested";
    pub const ATTESTATION_REVOKED: &str = "AttestationRevoked";
    pub const CREATED_CATALOG: &str = "CreatedCatalog";
    pub const DELETED_CATALOG: &str = "DeletedCatalog";
    pub const CATALOG_ADDED_DIDS: &str = "CatalogAddedDids";
}

#[derive(Encode)]
pub struct EmptyArgs {}

pub fn register_did() -> Call<EmptyArgs> {
    Call::new(MODULE, calls::REGISTER_DID, EmptyArgs {})
}

#[derive(Encode)]
pub struct RegisterDidForArgs<T: Identity> {
    subject: <T as System>::AccountId,
}

pub fn register_did_for<T: Identity>(
    subject: <T as System>::AccountId,
) -> Call<RegisterDidForArgs<T>> {
    Call::new(
        MODULE,
        calls::REGISTER_DID_FOR,
        RegisterDidForArgs { subject },
    )
}
#[derive(Encode)]
pub struct RegisterDidCatalogArgs<T: Identity> {
    subject: <T as System>::AccountId,
    catalog_id: CatalogId,
    name: Vec<u8>,
}

pub fn register_did_catalog<T: Identity>(
    subject: <T as System>::AccountId,
    catalog_id: CatalogId,
    name: Vec<u8>,
) -> Call<RegisterDidCatalogArgs<T>> {
    Call::new(
        MODULE,
        calls::REGISTER_DID_CATALOG,
        RegisterDidCatalogArgs {
            subject,
            catalog_id,
            name,
        },
    )
}

#[derive(Encode)]
pub struct AddDidPropertiesArgs {
    did: Did,
    properties: Vec<DidProperty>,
}

pub fn add_did_properties(
    did: Did,
    properties: Vec<DidProperty>,
) -> Call<AddDidPropertiesArgs> {
    Call::new(
        MODULE,
        calls::ADD_DID_PROPERTIES,
        AddDidPropertiesArgs { did, properties },
    )
}
#[derive(Encode)]
pub struct PatchDidPropertiesArgs {
    did: Did,
    properties: Vec<DidProperty>,
}

pub fn patch_did_properties(
    did: Did,
    properties: Vec<DidProperty>,
) -> Call<PatchDidPropertiesArgs> {
    Call::new(
        MODULE,
        calls::PATCH_DID_PROPERTIES,
        PatchDidPropertiesArgs { did, properties },
    )
}
#[derive(Encode)]
pub struct UpdateDidPropertiesArgs {
    did: Did,
    add: Vec<DidProperty>,
    remove: Vec<DidProperty>,
}

pub fn update_did_properties(
    did: Did,
    add: Vec<DidProperty>,
    remove: Vec<DidProperty>,
) -> Call<UpdateDidPropertiesArgs> {
    Call::new(
        MODULE,
        calls::UPDATE_DID_PROPERTIES,
        UpdateDidPropertiesArgs { did, add, remove },
    )
}

#[derive(Encode)]
pub struct UpdateDidControllersArgs {
    did: Did,
    add: Option<Vec<Vec<u8>>>,
    remove: Option<Vec<Vec<u8>>>,
}

pub fn update_did_controllers(
    did: Did,
    add: Option<Vec<Vec<u8>>>,
    remove: Option<Vec<Vec<u8>>>,
) -> Call<UpdateDidControllersArgs> {
    Call::new(
        MODULE,
        calls::UPDATE_DID_CONTROLLERS,
        UpdateDidControllersArgs { did, add, remove },
    )
}
#[derive(Encode)]
pub struct AuthorizeClaimConsumerArgs {
    target_did: Did,
    claim_consumer: Did,
}

pub fn authorize_claim_consumer(
    target_did: Did,
    claim_consumer: Did,
) -> Call<AuthorizeClaimConsumerArgs> {
    Call::new(
        MODULE,
        calls::AUTHORIZE_CLAIM_CONSUMER,
        AuthorizeClaimConsumerArgs {
            target_did,
            claim_consumer,
        },
    )
}

#[derive(Encode)]
pub struct AuthorizeClaimVerifierArgs {
    target_did: Did,
    claim_verifier: Did,
}

pub fn authorize_claim_verifier(
    target_did: Did,
    claim_verifier: Did,
) -> Call<AuthorizeClaimVerifierArgs> {
    Call::new(
        MODULE,
        calls::AUTHORIZE_CLAIM_VERIFIER,
        AuthorizeClaimVerifierArgs {
            target_did,
            claim_verifier,
        },
    )
}

#[derive(Encode)]
pub struct MakeClaimArgs {
    claim_consumer: Did,
    target_did: Did,
    description: Vec<u8>,
    statements: Vec<Statement>,
}

pub fn make_claim(
    claim_consumer: Did,
    target_did: Did,
    description: Vec<u8>,
    statements: Vec<Statement>,
) -> Call<MakeClaimArgs> {
    Call::new(
        MODULE,
        calls::MAKE_CLAIM,
        MakeClaimArgs {
            claim_consumer,
            target_did,
            description,
            statements,
        },
    )
}

#[derive(Encode)]
pub struct AttestClaimArgs {
    claim_verifier: Did,
    target_did: Did,
    claim_id: u64,
    statements: Vec<Statement>,
    valid_until: u64,
}

pub fn attest_claim(
    claim_verifier: Did,
    target_did: Did,
    claim_id: u64,
    statements: Vec<Statement>,
    valid_until: AddaxTimestamp,
) -> Call<AttestClaimArgs> {
    Call::new(
        MODULE,
        calls::ATTEST_CLAIM,
        AttestClaimArgs {
            claim_verifier,
            target_did,
            claim_id,
            statements,
            valid_until: valid_until.0,
        },
    )
}

#[derive(Encode)]
pub struct RevokeClaimArgs {
    claim_verifier: Did,
    claim_id: u64,
}

pub fn revoke_claim(claim_verifier: Did, claim_id: u64) -> Call<RevokeClaimArgs> {
    Call::new(
        MODULE,
        calls::REVOKE_CLAIM,
        RevokeClaimArgs {
            claim_verifier,
            claim_id,
        },
    )
}

#[derive(Encode)]
pub struct CreateCatalogArgs {
    owner: Did,
}

pub fn create_catalog(owner: Did) -> Call<CreateCatalogArgs> {
    Call::new(MODULE, calls::CREATE_CATALOG, CreateCatalogArgs { owner })
}

#[derive(Encode)]
pub struct DeleteCatalogArgs {
    catalog_id: CatalogId,
}

pub fn delete_catalog(catalog_id: CatalogId) -> Call<DeleteCatalogArgs> {
    Call::new(
        MODULE,
        calls::DELETE_CATALOG,
        DeleteCatalogArgs { catalog_id },
    )
}

#[derive(Encode)]
pub struct CatalogAddDidArgs {
    catalog_id: CatalogId,
    did: Did,
    name: Vec<u8>,
}

pub fn catalog_add_did(
    catalog_id: CatalogId,
    did: Did,
    name: Vec<u8>,
) -> Call<CatalogAddDidArgs> {
    Call::new(
        MODULE,
        calls::CATALOG_ADD_DID,
        CatalogAddDidArgs {
            catalog_id,
            did,
            name,
        },
    )
}

impl From<i64> for AddaxTimestamp {
    fn from(timestamp: i64) -> Self {
        AddaxTimestamp(u64::try_from(timestamp).unwrap())
    }
}
impl Into<i64> for AddaxTimestamp {
    fn into(self) -> i64 {
        i64::try_from(self.0).unwrap()
    }
}
