use soroban_sdk::{contracttype, Address, String};

/// Audit status of a registered contract.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AuditStatus {
    Verified,
    Pending,
    Unaudited,
}

/// A registry entry describing a deployed Soroban contract.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContractEntry {
    pub address: String,
    pub name: String,
    pub version: String,
    pub maintainer: Address,
    pub audit_status: AuditStatus,
    pub registered_at: u64,
}
