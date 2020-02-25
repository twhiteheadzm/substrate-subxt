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

/// Module name
pub const MODULE: &str = "Identity";

pub type CatalogId = u32;
pub type ClaimIndex = u64;
pub type Moment = u64;
pub type ShortName = Vec<u8>;
pub type DidPropertyName = Vec<u8>;

pub trait Identity: System {}
mod calls {
    pub const REGISTER_DID: &str = "register_did";
    pub const REGISTER_DID_FOR: &str = "register_did_for";
    pub const UPDATE_DID: &str = "update_did";
    pub const REPLACE_DID: &str = "replace_did";
    pub const UPDATE_DID_CONTROLLERS: &str = "update_did_controllers";

    pub const AUTHORIZE_CLAIM_CONSUMERS: &str = "authorize_claim_consumers";
    pub const REVOKE_CLAIM_CONSUMERS: &str = "revoke_claim_consumers";
    pub const AUTHORIZE_CLAIM_ISSUERS: &str = "authorize_claim_issuers";
    pub const REVOKE_CLAIM_ISSUERS: &str = "revoke_claim_issuers";

    pub const MAKE_CLAIM: &str = "make_claim";

    pub const ATTEST_CLAIM: &str = "attest_claim";
    pub const REVOKE_ATTESTATION: &str = "revoke_attestation";

    pub const CREATE_CATALOG: &str = "create_catalog";
    pub const REMOVE_CATALOG: &str = "remove_catalog";
    pub const ADD_DIDS_TO_CATALOG: &str = "add_dids_to_catalog";
    pub const REMOVE_DIDS_FROM_CATALOG: &str = "remove_dids_from_catalog";
}
/// events
#[allow(unused)]
pub mod events {
    pub const REGISTERED: &str = "Registered";
    pub const DID_UPDATED: &str = "DidUpdated";
    pub const DID_REPLACED: &str = "DidReplaced";
    pub const DID_CONTROLLERS_UPDATED: &str = "DidControllersUpdated";

    pub const CLAIM_CONSUMERS_ADDED: &str = "ClaimConsumersAdded";
    pub const CLAIM_CONSUMERS_REMOVED: &str = "ClaimConsumersRemoved";
    pub const CLAIM_ISSUERS_ADDED: &str = "ClaimIssuersAdded";
    pub const CLAIM_ISSUERS_REMOVED: &str = "ClaimIssuersRemoved";

    pub const CLAIM_MADE: &str = "ClaimMade";

    pub const CLAIM_ATTESTED: &str = "ClaimAttested";
    pub const CLAIM_ATTESTATION_REVOKED: &str = "ClaimAttestationRevoked";

    pub const CATALOG_CREATED: &str = "CatalogCreated";
    pub const CATALOG_REMOVED: &str = "CatalogRemoved";
    pub const CATALOG_DIDS_ADDED: &str = "CatalogDidsAdded";
    pub const CATALOG_DIDS_REMOVED: &str = "CatalogDidsRemoved";
}

#[derive(Encode)]
pub struct RegisterDidArgs {
    properties: Option<Vec<DidProperty>>,
}

pub fn register_did(properties: Option<Vec<DidProperty>>) -> Call<RegisterDidArgs> {
    Call::new(MODULE, calls::REGISTER_DID, RegisterDidArgs { properties })
}

#[derive(Encode)]
pub struct RegisterDidForArgs<T: Identity> {
    subject: <T as System>::AccountId,
    properties: Option<Vec<DidProperty>>,
}

pub fn register_did_for<T: Identity>(
    subject: <T as System>::AccountId,
    properties: Option<Vec<DidProperty>>,
) -> Call<RegisterDidForArgs<T>> {
    Call::new(
        MODULE,
        calls::REGISTER_DID_FOR,
        RegisterDidForArgs {
            subject,
            properties,
        },
    )
}
#[derive(Encode)]
pub struct UpdateDidArgs {
    did: Did,
    add_properties: Option<Vec<DidProperty>>,
    remove_keys: Option<Vec<DidPropertyName>>,
}

pub fn update_did(
    did: Did,
    add_properties: Option<Vec<DidProperty>>,
    remove_keys: Option<Vec<DidPropertyName>>,
) -> Call<UpdateDidArgs> {
    Call::new(
        MODULE,
        calls::UPDATE_DID,
        UpdateDidArgs {
            did,
            add_properties,
            remove_keys,
        },
    )
}
#[derive(Encode)]
pub struct ReplaceDidArgs {
    did: Did,
    properties: Option<Vec<DidProperty>>,
}

pub fn replace_did(
    did: Did,
    properties: Option<Vec<DidProperty>>,
) -> Call<ReplaceDidArgs> {
    Call::new(
        MODULE,
        calls::REPLACE_DID,
        ReplaceDidArgs { did, properties },
    )
}
#[derive(Encode)]
pub struct UpdateDidControllersArgs<T: Identity> {
    did: Did,
    add: Option<Vec<<T as System>::AccountId>>,
    remove: Option<Vec<<T as System>::AccountId>>,
}

pub fn update_did_controllers<T: Identity>(
    did: Did,
    add: Option<Vec<<T as System>::AccountId>>,
    remove: Option<Vec<<T as System>::AccountId>>,
) -> Call<UpdateDidControllersArgs<T>> {
    Call::new(
        MODULE,
        calls::UPDATE_DID_CONTROLLERS,
        UpdateDidControllersArgs { did, add, remove },
    )
}
#[derive(Encode)]
pub struct AuthorizeClaimConsumersArgs {
    target_did: Did,
    claim_consumers: Vec<(Did, Moment)>,
}

pub fn authorize_claim_consumers(
    target_did: Did,
    claim_consumers: Vec<(Did, Moment)>,
) -> Call<AuthorizeClaimConsumersArgs> {
    Call::new(
        MODULE,
        calls::AUTHORIZE_CLAIM_CONSUMERS,
        AuthorizeClaimConsumersArgs {
            target_did,
            claim_consumers,
        },
    )
}
#[derive(Encode)]
pub struct RevokeClaimConsumersArgs {
    target_did: Did,
    claim_consumers: Vec<Did>,
}
pub fn revoke_claim_consumers(
    target_did: Did,
    claim_consumers: Vec<Did>,
) -> Call<RevokeClaimConsumersArgs> {
    Call::new(
        MODULE,
        calls::REVOKE_CLAIM_CONSUMERS,
        RevokeClaimConsumersArgs {
            target_did,
            claim_consumers,
        },
    )
}
#[derive(Encode)]
pub struct AuthorizeClaimIssuersArgs {
    target_did: Did,
    claim_issuers: Vec<(Did, Moment)>,
}

pub fn authorize_claim_issuers(
    target_did: Did,
    claim_issuers: Vec<(Did, Moment)>,
) -> Call<AuthorizeClaimIssuersArgs> {
    Call::new(
        MODULE,
        calls::AUTHORIZE_CLAIM_ISSUERS,
        AuthorizeClaimIssuersArgs {
            target_did,
            claim_issuers,
        },
    )
}
#[derive(Encode)]
pub struct RevokeClaimIssuersArgs {
    target_did: Did,
    claim_issuers: Vec<Did>,
}
pub fn revoke_claim_issuers(
    target_did: Did,
    claim_issuers: Vec<Did>,
) -> Call<RevokeClaimIssuersArgs> {
    Call::new(
        MODULE,
        calls::REVOKE_CLAIM_ISSUERS,
        RevokeClaimIssuersArgs {
            target_did,
            claim_issuers,
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
    claim_issuer: Did,
    target_did: Did,
    claim_index: ClaimIndex,
    statements: Vec<Statement>,
    valid_until: Moment,
}
pub fn attest_claim(
    claim_issuer: Did,
    target_did: Did,
    claim_index: ClaimIndex,
    statements: Vec<Statement>,
    valid_until: Moment,
) -> Call<AttestClaimArgs> {
    Call::new(
        MODULE,
        calls::ATTEST_CLAIM,
        AttestClaimArgs {
            claim_issuer,
            target_did,
            claim_index,
            statements,
            valid_until,
        },
    )
}
#[derive(Encode)]
pub struct RevokeAttestationArgs {
    claim_issuer: Did,
    target_did: Did,
    claim_index: ClaimIndex,
}
pub fn revoke_attestation(
    claim_issuer: Did,
    target_did: Did,
    claim_index: ClaimIndex,
) -> Call<RevokeAttestationArgs> {
    Call::new(
        MODULE,
        calls::REVOKE_ATTESTATION,
        RevokeAttestationArgs {
            claim_issuer,
            target_did,
            claim_index,
        },
    )
}
#[derive(Encode)]
pub struct CreateCatalogArgs {
    owner_did: Did,
}
pub fn create_catalog(owner_did: Did) -> Call<CreateCatalogArgs> {
    Call::new(
        MODULE,
        calls::CREATE_CATALOG,
        CreateCatalogArgs { owner_did },
    )
}
#[derive(Encode)]
pub struct RemoveCatalogArgs {
    owner_did: Did,
    catalog_id: CatalogId,
}
pub fn remove_catalog(owner_did: Did, catalog_id: CatalogId) -> Call<RemoveCatalogArgs> {
    Call::new(
        MODULE,
        calls::REMOVE_CATALOG,
        RemoveCatalogArgs {
            owner_did,
            catalog_id,
        },
    )
}
#[derive(Encode)]
pub struct AddDidsToCatalogArgs {
    owner_did: Did,
    catalog_id: CatalogId,
    dids: Vec<(Did, ShortName)>,
}
pub fn add_dids_to_catalog(
    owner_did: Did,
    catalog_id: CatalogId,
    dids: Vec<(Did, ShortName)>,
) -> Call<AddDidsToCatalogArgs> {
    Call::new(
        MODULE,
        calls::ADD_DIDS_TO_CATALOG,
        AddDidsToCatalogArgs {
            owner_did,
            catalog_id,
            dids,
        },
    )
}
#[derive(Encode)]
pub struct RemoveDidsFromCatalogArgs {
    owner_did: Did,
    catalog_id: CatalogId,
    dids: Vec<Did>,
}
pub fn remove_dids_from_catalog(
    owner_did: Did,
    catalog_id: CatalogId,
    dids: Vec<Did>,
) -> Call<RemoveDidsFromCatalogArgs> {
    Call::new(
        MODULE,
        calls::REMOVE_DIDS_FROM_CATALOG,
        RemoveDidsFromCatalogArgs {
            owner_did,
            catalog_id,
            dids,
        },
    )
}
