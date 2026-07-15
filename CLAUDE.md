# veritas

An open-source contract registry and verification dashboard for the Soroban ecosystem — search, verify, and inspect deployed smart contracts on Stellar.


You are scaffolding and half-building the Soroban Contract Registry & Verification Dashboard.

## What this project is
The Stellar/Soroban ecosystem has no open, community-maintained place to discover,
verify, and inspect deployed Soroban smart contracts. This project solves that — it is
an on-chain registry where maintainers can register their deployed contracts with
metadata (name, version, audit status), and a public dashboard where anyone can search,
browse, and verify contracts. Think Etherscan's contract verification, but purpose-built
for Soroban.

## Architecture
- Contract (Soroban/Rust): On-chain registry. Stores ContractEntry records keyed by
  contract address. Implements SEP-41-compatible token interface patterns for
  interoperability. Functions: register_contract, get_contract, update_audit_status.
- Backend (Rust/Axum): Indexes deployed contracts by polling the Stellar RPC, caches
  contract metadata in-memory, and exposes a REST API for the frontend. No database —
  in-memory HashMap is the store for now.
- Frontend (Next.js + TypeScript + Tailwind): Connects to Freighter wallet. Lets
  maintainers register their contracts by signing a transaction via Freighter. Public
  users can search and browse without connecting a wallet.

## Best practices to follow
- Contract: Use Soroban's instance storage for contract registry data. Use SEP-41
  interface patterns. All public functions must have doc comments. Use a custom Error
  enum with #[contracterror]. Never use unwrap() — return descriptive errors.
- Backend: Structure routes in separate files under src/routes/. Models in src/models/.
  RPC polling logic in src/services/indexer.rs. Use tower-http for CORS. Errors handled
  via a central AppError type implementing IntoResponse.
- Frontend: Use @stellar/freighter-api for wallet connection. All Stellar interactions
  go through src/lib/stellar.ts. API calls go through src/lib/api.ts. Types defined in
  src/types/index.ts and imported everywhere — no inline type definitions in components.

## Key decisions
- SEP-41 token interface is the standard for Soroban token interoperability — use its
  patterns even for non-token contracts where applicable (e.g. structured metadata).
- Freighter is the only wallet — no WalletConnect.
- In-memory store only — no database yet.

---

GITHUB_USERNAME=<fill in>
PROJECT_NAME=soroban-contract-registry

Rules:
- Commit after every meaningful step with a clear message.
- Never batch multiple layers into one commit.
- Run build/tests after every major milestone and fix errors before continuing.
- Use `gh repo create` to create each repo before pushing.
- Add GitHub Actions CI to every repo.

---

## REPO 1: Frontend

1. Create repo: `$PROJECT_NAME-frontend`
2. Scaffold Next.js 14 app with TypeScript and Tailwind
   COMMIT: "chore: init Next.js 14 app with TypeScript and Tailwind"
3. Install @stellar/freighter-api
   COMMIT: "chore: install Freighter wallet dependency"
4. Add folder structure:
   - /app — home, search, contract/[id] pages
   - /components — SearchBar, ContractCard, AuditBadge, ContractTable, WalletButton
   - /lib — stellar.ts (Freighter connection + transaction helpers), api.ts (backend calls)
   - /types — index.ts (Contract, AuditStatus, WalletState)
   COMMIT: "chore: add folder structure and placeholder files"
5. Implement WalletButton — connects/disconnects Freighter, displays truncated public key
   when connected, uses @stellar/freighter-api isConnected() and getPublicKey()
   COMMIT: "feat: implement WalletButton with Freighter connection"
6. Add layout with navbar (includes WalletButton) and footer stubs
   COMMIT: "feat: add layout with navbar and footer"
7. Implement stellar.ts — exports connectWallet(), getPublicKey(), signTransaction()
   COMMIT: "feat: implement Freighter helpers in stellar.ts"
8. Implement SearchBar — controlled input, calls onSearch prop on submit
   COMMIT: "feat: implement SearchBar component"
9. Implement AuditBadge — renders colored badge based on AuditStatus (verified/pending/unaudited)
   COMMIT: "feat: implement AuditBadge component"
10. Implement ContractCard — renders contract address, name, version, AuditBadge
    COMMIT: "feat: implement ContractCard component"
11. Implement /search page — SearchBar + mock contract list rendered as ContractCards
    COMMIT: "feat: implement search page with mock data"
12. Implement /contract/[id] page — full contract detail view from mock data: address,
    maintainer, audit status, deployed_at, description
    COMMIT: "feat: implement contract detail page with mock data"
13. Add .env.example: NEXT_PUBLIC_API_URL
    COMMIT: "chore: add .env.example"
14. RUN: `npm run build` — fix all errors before continuing
    COMMIT: "fix: resolve build errors"
15. Add GitHub Actions CI at .github/workflows/ci.yml:
    - Trigger: push and pull_request to main
    - Steps: checkout, setup Node 20, npm ci, npm run build
    COMMIT: "ci: add GitHub Actions CI workflow"
16. Add README.md — what the project is, how to run it, how Freighter is used,
    how it connects to the backend
    COMMIT: "docs: add README"
17. Push to GitHub

---

## REPO 2: Backend

1. Create repo: `$PROJECT_NAME-backend`
2. Init Rust project with `cargo init`
   COMMIT: "chore: init Rust project"
3. Add to Cargo.toml: axum, tokio (full features), serde, serde_json, reqwest,
   tower-http (cors feature), dotenv, uuid (v4)
   COMMIT: "chore: add core dependencies"
4. Scaffold folder structure:
   - src/main.rs — server entry, router setup, in-memory store init
   - src/routes/mod.rs, contracts.rs, health.rs
   - src/models/mod.rs, contract.rs
   - src/services/mod.rs, indexer.rs, rpc.rs
   - src/errors.rs — AppError enum implementing IntoResponse
   COMMIT: "chore: scaffold folder structure with module stubs"
5. Define AppError in errors.rs — variants: NotFound, BadRequest, InternalError.
   Implement IntoResponse returning appropriate status codes and JSON error body
   COMMIT: "feat: implement AppError with IntoResponse"
6. Define Contract struct in models/contract.rs:
   id (uuid), name, address, version, audit_status (enum: Verified/Pending/Unaudited),
   maintainer, description, deployed_at. Derive Serialize, Deserialize, Clone
   COMMIT: "feat: define Contract model"
7. Define AppState struct in main.rs holding Arc<RwLock<HashMap<String, Contract>>>.
   Seed with 3 mock contracts on startup
   COMMIT: "feat: implement in-memory store with mock seed data"
8. Implement Axum router in main.rs with CORS via tower-http, mount all routes,
   bind to PORT from env (default 8080)
   COMMIT: "feat: implement Axum server with CORS and router"
9. Implement /health — returns 200 OK with JSON {status: "ok"}
   COMMIT: "feat: implement /health route"
10. Implement GET /contracts — reads from AppState, returns Vec<Contract> as JSON
    COMMIT: "feat: implement GET /contracts"
11. Implement GET /contracts/:id — returns Contract by id or AppError::NotFound
    COMMIT: "feat: implement GET /contracts/:id"
12. Add .env.example: STELLAR_RPC_URL, PORT
    COMMIT: "chore: add .env.example"
13. RUN: `cargo build` — fix all errors before continuing
    COMMIT: "fix: resolve build errors"
14. Add unit tests in contracts.rs for GET /contracts and GET /contracts/:id
    using axum::test helpers
    COMMIT: "test: add route handler unit tests"
15. Add unit test for Contract model serialization/deserialization
    COMMIT: "test: add Contract model serde tests"
16. RUN: `cargo test` — fix all failures before continuing
    COMMIT: "fix: resolve test failures" (only if needed)
17. Add GitHub Actions CI at .github/workflows/ci.yml:
    - Trigger: push and pull_request to main
    - Steps: checkout, install Rust stable via rustup, cargo build --release, cargo test
    COMMIT: "ci: add GitHub Actions CI workflow"
18. Add README.md — what the project is, how to run it, env vars, all API routes
    COMMIT: "docs: add README"
19. Push to GitHub
