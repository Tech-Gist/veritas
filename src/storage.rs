use soroban_sdk::{Env, String};

use crate::types::ContractEntry;

/// Persists a contract entry in instance storage, keyed by its contract address.
pub fn save_contract(env: &Env, entry: &ContractEntry) {
    env.storage().instance().set(&entry.address, entry);
}

/// Reads a contract entry from instance storage by contract address.
pub fn get_contract(env: &Env, address: &String) -> Option<ContractEntry> {
    env.storage().instance().get(address)
}
