#![no_std]

mod errors;
mod storage;
mod types;

use soroban_sdk::{contract, contractimpl};

pub use errors::ContractError;
pub use types::{AuditStatus, ContractEntry};

#[contract]
pub struct ContractRegistry;

#[contractimpl]
impl ContractRegistry {}
