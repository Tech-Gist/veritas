# veritas-contract

The on-chain Soroban smart contract powering the **Soroban Contract Registry &
Verification Dashboard** — an open, community-maintained registry where
maintainers register their deployed Soroban contracts with metadata (name,
version, audit status), so anyone can search, browse, and verify them. Think
Etherscan's contract verification, but purpose-built for Soroban.

This repo contains only the on-chain contract. The indexing API lives in the
`-backend` repo and the public dashboard lives in the `-frontend` repo.

## What the contract does

The contract is a simple on-chain registry keyed by contract address:

- Maintainers **register** their deployed contract's metadata (name, version,
  maintainer, audit status, registration timestamp).
- Anyone can **read** a registered entry by contract address — no
  authentication required.
- The original maintainer can **update the audit status** of their entry
  (e.g. from `Unaudited` to `Verified`) as audits complete.

Storage uses Soroban instance storage, keyed by the contract's address
string. There is no way to overwrite or delete another maintainer's entry —
registration is authenticated by the maintainer's `Address`, and audit status
updates are restricted to that same maintainer.

## Functions

### `register_contract(env: Env, entry: ContractEntry) -> Result<(), ContractError>`

Registers a new contract entry. Requires authorization (`require_auth`) from
`entry.maintainer`.

| Failure | Condition |
|---|---|
| `InvalidInput` | `address`, `name`, or `version` is an empty string |
| `AlreadyRegistered` | An entry already exists for `entry.address` |

Emits a `registrd` event with the registered contract's address.

### `get_contract(env: Env, address: String) -> Result<ContractEntry, ContractError>`

Fetches a registered entry by contract address. Public — no authorization
required.

| Failure | Condition |
|---|---|
| `NotFound` | No entry exists for `address` |

### `update_audit_status(env: Env, address: String, caller: Address, new_status: AuditStatus) -> Result<(), ContractError>`

Updates the audit status of an existing entry. Requires authorization
(`require_auth`) from `caller`.

| Failure | Condition |
|---|---|
| `NotFound` | No entry exists for `address` |
| `Unauthorized` | `caller` is not the entry's original `maintainer` |

Emits an `audit` event with the updated contract's address.

## Types

**`AuditStatus`** — `Verified` \| `Pending` \| `Unaudited`

**`ContractEntry`**

| Field | Type | Description |
|---|---|---|
| `address` | `String` | The registered contract's address (strkey) |
| `name` | `String` | Human-readable contract name |
| `version` | `String` | Contract version string |
| `maintainer` | `Address` | The account authorized to manage this entry |
| `audit_status` | `AuditStatus` | Current audit state |
| `registered_at` | `u64` | Ledger timestamp at registration |

## Error codes

| Variant | Code | Meaning |
|---|---|---|
| `AlreadyRegistered` | 1 | An entry already exists for the given address |
| `NotFound` | 2 | No entry exists for the given address |
| `Unauthorized` | 3 | Caller is not the entry's maintainer |
| `InvalidInput` | 4 | A required string field was empty |

## Building

Requires Rust stable with the `wasm32-unknown-unknown` target:

```bash
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release
```

The compiled contract wasm is written to
`target/wasm32-unknown-unknown/release/soroban_contract_registry.wasm`.

## Testing

```bash
cargo test
```

## Deploying

Deploying requires the [Stellar CLI](https://developers.stellar.org/docs/tools/cli/stellar-cli):

```bash
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/soroban_contract_registry.wasm \
  --source <YOUR_IDENTITY> \
  --network testnet
```

This prints the deployed contract's address, which the backend indexer polls
via the Stellar RPC to discover registered entries.

## Project layout

```
src/
  lib.rs       — contract entry point, public functions
  storage.rs   — instance storage read/write helpers
  types.rs     — ContractEntry, AuditStatus
  errors.rs    — ContractError (#[contracterror])
```
