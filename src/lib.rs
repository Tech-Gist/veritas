#![no_std]

mod errors;
mod storage;
mod types;

use soroban_sdk::{contract, contractimpl, symbol_short, Env};

pub use errors::ContractError;
pub use types::{AuditStatus, ContractEntry};

#[contract]
pub struct ContractRegistry;

#[contractimpl]
impl ContractRegistry {
    /// Registers a new contract entry in the registry.
    ///
    /// Requires authorization from `entry.maintainer`. Fails if a contract is
    /// already registered under `entry.address`, or if any string field is empty.
    pub fn register_contract(env: Env, entry: ContractEntry) -> Result<(), ContractError> {
        entry.maintainer.require_auth();

        if entry.address.is_empty() || entry.name.is_empty() || entry.version.is_empty() {
            return Err(ContractError::InvalidInput);
        }

        if storage::get_contract(&env, &entry.address).is_some() {
            return Err(ContractError::AlreadyRegistered);
        }

        storage::save_contract(&env, &entry);

        env.events()
            .publish((symbol_short!("registrd"),), entry.address);

        Ok(())
    }
}
