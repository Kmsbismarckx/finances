# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

Money Manager — personal finance tracking app (like Money Manager), written in Rust. Target platform: Android (min SDK 24 / Android 7 first), with a native iOS UI planned later on top of the same Rust core.

## Current state

Early implementation stage, starting with the auth slice. Single crate (no workspace split yet), modules under `src/domain` and `src/infrastructure` (module intentionally named `infrastructure`, not `infra` — see note below).

Done so far, all with unit tests (integration test for the SQLite repo):
- `domain::email::Email` — validated value object
- `domain::user::{User, UserId, PasswordHash}` — Always-Valid entity, `User::register(id, email, password_hash)`; `User::from_persisted(...)` to rehydrate from storage without re-running register's business rules
- `domain::device::{Device, DeviceId, RefreshTokenHash}` — Always-Valid entity representing both an auth session and a sync client (see architecture.md); `Device::register(id, user_id, name, refresh_token_hash)`, `revoke()` — not yet wired into any command or repository
- `domain::user_repository::UserRepository` — port trait (`find_by_email`, `save`), fallible (`RepositoryError`, owned by `domain`, not `infrastructure`)
- `domain::password_hasher::PasswordHasher` — port trait (`hash`)
- `domain::register_user::register_user` — command composing the above: rejects duplicate email, hashes password, persists
- `domain::fakes` (test-only, `#[cfg(test)]`) — `InMemoryUserRepository`, `FakePasswordHasher`
- `infrastructure::crypto` — argon2 `hash_password`/`verify_password`
- `infrastructure::password_hasher::Argon2PasswordHasher` — implements the `PasswordHasher` port
- `infrastructure::db::open` — opens a SQLite connection and creates the `users` table if missing
- `infrastructure::user_repository::SqliteUserRepository` — implements `UserRepository` via rusqlite
- `server::auth::register` (Axum) — `POST /auth/register`, maps `DomainError` to HTTP status codes (400/409/500); verified working end-to-end with `curl` (201 on success, 409 on duplicate email)
- `main.rs` runs the Axum server on `127.0.0.1:3000`, opens `finances.db` on startup

This is a full working vertical slice (HTTP → domain → SQLite) for registration only.

Android/UniFFI walking skeleton is also done and verified on an emulator:
- Crate is now library+binary: `[lib]` with `crate-type = ["cdylib", "rlib"]`, `src/lib.rs` declares the modules (`main.rs` imports from the `finances` lib crate instead of declaring modules itself)
- `src/ffi.rs` — `#[uniffi::export] fn greet(name: String) -> String`, first (throwaway) FFI function, no real business logic yet
- `src/bin/uniffi-bindgen.rs` — thin binary (`uniffi::uniffi_bindgen_main()`) used to generate Kotlin bindings; needs the `cli` feature on the `uniffi` dependency
- Android project lives at `android/` inside this repo (Kotlin + Jetpack Compose, min SDK 24, package `com.bismarckx.rationem`)
- Build flow (manual for now, not yet scripted): `cargo ndk -o android/app/src/main/jniLibs -t armeabi-v7a -t arm64-v8a -t x86_64 build`, then `cargo run --bin uniffi-bindgen generate --library target/aarch64-linux-android/debug/libfinances.so --language kotlin --out-dir android/app/src/main/java`
- Kotlin bindings use JNA (`com.sun.jna.Library`) under the hood — added `net.java.dev.jna:jna:5.15.0@aar` to `android/app/build.gradle.kts`
- `jniLibs` and generated `uniffi/` Kotlin sources are gitignored (build outputs, regenerated via the commands above, not committed)
- Confirmed working end-to-end: `MainActivity.kt`'s `Greeting` composable calls `greet(name)`, emulator shows the Rust-generated string

Next step — not yet decided, ask the author which to do first:
- `POST /auth/login` — needs `Device` wired into a repository + command, plus JWT access token + opaque refresh token issuance (see architecture design discussed earlier: refresh token rotation, per-device revocation)
- or: turn the manual cargo-ndk + uniffi-bindgen steps into a build script (per stack.md, a script not a gradle plugin), and/or start wiring real domain calls (e.g. register) through FFI instead of the throwaway `greet` function

No iOS project exists yet.

Full rationale for these decisions lives in two docs in the repo root — read them before making architectural suggestions:
- `stack.md` — tech stack choices
- `architecture.md` — architecture patterns and how to apply them, written as a learning reference (the author is new to these practices and is deliberately building experience with them)

## How to work with the author

The author is writing all the code themselves and wants Claude for advice, review, and explanation — not for autonomous implementation. **Always ask before making changes or taking action; do not just start writing/editing code or files.** This applies to every step, not just the first one.

## Stack (see stack.md for full detail)

- Architecture: shared Rust core + native UI per platform (UniFFI bindings), not a cross-platform Rust GUI framework
- Android UI: Kotlin + Jetpack Compose, min SDK 24
- iOS UI (later): Swift/SwiftUI, same Rust core
- DB: rusqlite (sync SQLite)
- Money: stored as `i64` minor units (cents/kopecks), no floats
- FFI: uniffi, proc-macro style (`#[uniffi::export]`), no `.udl` files
- Android build: cargo-ndk invoked via script (not the gradle plugin), so the build step stays visible
- Sync: own Rust backend server (HTTPS), not cloud-storage-based or P2P
- Conflict resolution: last-write-wins by server-assigned `updated_at`; soft deletes via `deleted_at` tombstones; incremental sync via a monotonic `sync_version` cursor per client
- Open/not yet decided: Cargo workspace crate layout, DB schema, server web framework (Axum vs Actix), authentication approach

## Architecture (see architecture.md for full detail with examples)

Hexagonal/clean architecture combined with DDD and Vladimir Khorikov's practices:

- Planned crate split: `domain` (pure business logic, no IO) / `infrastructure` (rusqlite repositories, HTTP client) / `ffi` (thin UniFFI adapter) / `server` (Axum API). Dependencies point inward only — `domain` depends on nothing else.
- Always-Valid Domain Model: entities have private fields and fallible smart constructors (`fn new(...) -> Result<Self, DomainError>`); invalid state must be unrepresentable, not just checked elsewhere.
- Functional Core, Imperative Shell: `domain` contains pure functions only; all side effects (DB, network, time) live in `infrastructure`/`ffi`/`server`.
- CQRS-lite: commands go through the domain model with validation; read-only queries/reports (e.g. spending by category) go straight to SQL projections, bypassing domain objects.
- `Result<T, DomainError>` for expected business errors; `panic!`/`unwrap` only for actual invariant violations (bugs).
- Sync uses the outbox pattern: local writes go into the entity table and an `outbox` table in the same transaction; a background worker drains the outbox when online; inbound changes are applied by a separate handler that resolves conflicts per the LWW rule above.
- Test priority: most tests on `domain` (cheap, no mocks needed since it's pure), fewer integration tests on `infrastructure`, minimal tests on `ffi`/`server` adapters.

Since these practices are new to the author: introduce them incrementally (start with one Always-Valid entity, keep domain/infrastructure split at module level before splitting into crates, add CQRS-lite only once a real report/aggregation exists) rather than applying the full pattern set upfront. See the "Как осваивать это практически" section in `architecture.md`. Note: the module is named `infrastructure` (not the shorter `infra`) per the author's preference — the error type inside it is still `InfraError`.

## Commit messages

Follow the Angular commit message guidelines (`type(scope): summary`, lowercase summary with no trailing period, body as a bullet list explaining what/why) whenever asked to write a commit message.

## Commands

No custom build/test tooling yet — standard cargo commands apply to the current scaffold:
- `cargo build`
- `cargo run`
- `cargo test`
