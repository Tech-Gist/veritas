use soroban_sdk::contracterror;

/// Errors returned by the contract registry.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    AlreadyRegistered = 1,
    NotFound = 2,
    Unauthorized = 3,
    InvalidInput = 4,
}
