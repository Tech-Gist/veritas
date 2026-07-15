#![no_std]

mod errors;
mod storage;
mod types;

use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, String};

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

    /// Fetches a registered contract entry by its contract address.
    pub fn get_contract(env: Env, address: String) -> Result<ContractEntry, ContractError> {
        storage::get_contract(&env, &address).ok_or(ContractError::NotFound)
    }

    /// Updates the audit status of a registered contract.
    ///
    /// Only the original maintainer may update the audit status.
    pub fn update_audit_status(
        env: Env,
        address: String,
        caller: Address,
        new_status: AuditStatus,
    ) -> Result<(), ContractError> {
        caller.require_auth();

        let mut entry =
            storage::get_contract(&env, &address).ok_or(ContractError::NotFound)?;

        if entry.maintainer != caller {
            return Err(ContractError::Unauthorized);
        }

        entry.audit_status = new_status;
        storage::save_contract(&env, &entry);

        env.events()
            .publish((symbol_short!("audit"),), entry.address);

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::testutils::Address as _;

    fn sample_entry(env: &Env, maintainer: &Address) -> ContractEntry {
        ContractEntry {
            address: String::from_str(env, "CABCDEF0000000000000000000000000000000000000000000000000000"),
            name: String::from_str(env, "example-token"),
            version: String::from_str(env, "1.0.0"),
            maintainer: maintainer.clone(),
            audit_status: AuditStatus::Unaudited,
            registered_at: 1,
        }
    }

    #[test]
    fn test_register_contract_success() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(ContractRegistry, ());
        let client = ContractRegistryClient::new(&env, &contract_id);

        let maintainer = Address::generate(&env);
        let entry = sample_entry(&env, &maintainer);

        client.register_contract(&entry);

        let fetched = client.get_contract(&entry.address);
        assert_eq!(fetched, entry);
    }

    #[test]
    fn test_register_contract_duplicate_fails() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(ContractRegistry, ());
        let client = ContractRegistryClient::new(&env, &contract_id);

        let maintainer = Address::generate(&env);
        let entry = sample_entry(&env, &maintainer);

        client.register_contract(&entry);
        let result = client.try_register_contract(&entry);

        assert_eq!(result, Err(Ok(ContractError::AlreadyRegistered)));
    }

    #[test]
    fn test_get_contract_not_found() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(ContractRegistry, ());
        let client = ContractRegistryClient::new(&env, &contract_id);

        let missing_address = String::from_str(&env, "CDOESNOTEXIST0000000000000000000000000000000000000000000000");
        let result = client.try_get_contract(&missing_address);

        assert_eq!(result, Err(Ok(ContractError::NotFound)));
    }

    #[test]
    fn test_update_audit_status_unauthorized() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(ContractRegistry, ());
        let client = ContractRegistryClient::new(&env, &contract_id);

        let maintainer = Address::generate(&env);
        let impostor = Address::generate(&env);
        let entry = sample_entry(&env, &maintainer);

        client.register_contract(&entry);

        let result = client.try_update_audit_status(&entry.address, &impostor, &AuditStatus::Verified);

        assert_eq!(result, Err(Ok(ContractError::Unauthorized)));
    }
}
