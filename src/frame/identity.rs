//! Implements support for the identity module.
pub mod claim;
pub mod did;
pub mod did_property;
pub mod fact;
use crate::frame::{
    system::System,
    Call,
};
use claim::Statement;
use codec::Encode;
use did::Did;
use did_property::DidProperty;
use sp_core::H256;

/// Module name
pub const MODULE: &str = "Identity";

pub type CatalogId = H256;

pub trait Identity: System {}
mod calls {
    pub const REGISTER_DID: &str = "register_did";
    pub const REGISTER_DID_FOR: &str = "register_did_for";
    pub const REGISTER_DID_CATALOG: &str = "register_did_catalog";
    pub const ADD_DID_PROPERTIES: &str = "add_did_properties";
    pub const AUTHORIZE_CLAIM_CONSUMER: &str = "authorize_claim_consumer";
    pub const AUTHORIZE_CLAIM_VERIFIER: &str = "authorize_claim_verifier";
    pub const MAKE_CLAIM: &str = "make_claim";
    pub const ATTEST_CLAIM: &str = "attest_claim";
    pub const REVOKE_CLAIM: &str = "revoke_claim";
    pub const CREATE_CATALOG: &str = "create_catalog";
    pub const CATALOG_ADD_DID: &str = "catalog_add_did";
}
/// events
#[allow(unused)]
pub mod events {
    pub const REGISTERED: &str = "Registered";
    pub const PROPERTIES_ADDED: &str = "PropertiesAdded";
    pub const CLAIM_CONSUMER_ADDED: &str = "ClaimConsumerAdded";
    pub const CLAIM_VERIFIER_ADDED: &str = "ClaimVerifierAdded";
    pub const CLAIM_MADE: &str = "ClaimMade";
    pub const CLAIM_ATTESTED: &str = "ClaimAttested";
    pub const CLAIM_REVOKED: &str = "ClaimRevoked";
    pub const CREATED_CATALOG: &str = "CreatedCatalog";
    pub const ADD_TO_CATALOG: &str = "AddToCatalog";
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
    target_did: Did,
    claim_consumer: Did,
    description: Vec<u8>,
    statements: Vec<Statement>,
}

pub fn make_claim(
    target_did: Did,
    claim_consumer: Did,
    description: Vec<u8>,
    statements: Vec<Statement>,
) -> Call<MakeClaimArgs> {
    Call::new(
        MODULE,
        calls::MAKE_CLAIM,
        MakeClaimArgs {
            target_did,
            claim_consumer,
            description,
            statements,
        },
    )
}

#[derive(Encode)]
pub struct AttestClaimArgs {
    target_did: Did,
    claim_verifier: Did,
    claim_id: i64,
    statements: Vec<Statement>,
    expiry: i64,
}

pub fn attest_claim(
    target_did: Did,
    claim_verifier: Did,
    claim_id: i64,
    statements: Vec<Statement>,
    expiry: i64,
) -> Call<AttestClaimArgs> {
    Call::new(
        MODULE,
        calls::ATTEST_CLAIM,
        AttestClaimArgs {
            target_did,
            claim_verifier,
            claim_id,
            statements,
            expiry,
        },
    )
}

#[derive(Encode)]
pub struct RevokeClaimArgs {
    claim_verifier: Did,
    claim_id: i64,
}

pub fn revoke_claim(claim_verifier: Did, claim_id: i64) -> Call<RevokeClaimArgs> {
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
    name: Vec<u8>,
}

pub fn create_catalog(name: Vec<u8>) -> Call<CreateCatalogArgs> {
    Call::new(MODULE, calls::CREATE_CATALOG, CreateCatalogArgs { name })
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
