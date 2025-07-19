# THE PLAN

Brought to you by an AI analyizing [this lovely project](https://github.com/IsaaacQINH/haos-ldap-auth). I love it, 
I just am allergic to go, and so I'ma do it in Rust.
Also that one had errors connecting to LDAP, and I'd rather fix it in Rust than suffer through Go

LDAP doesn't seem to work, because it wants some kind of user credential I think.
https://github.com/goauthentik/authentik/issues/3510#issuecomment-1233345028
LDAP did work, but my MFA on my user broke things. I need to figure that out.

## 1. Project Setup
- Initialize a new Rust project with `cargo init`.
- Establish a clear crate structure:
  - Use `src/main.rs` for the executable entry point.
  - Create modules like `config`, `ldap`, `auth`, and `logger` under `src/`.
- Manage dependencies via `Cargo.toml`:
  - Use crates such as `ldap3` for LDAP operations.
  - Use `serde` + `serde_yaml` for YAML configuration parsing.
  - use tracing and tracing_subscriber with environment

## 2. Configuration Management
- Define a configuration struct reflecting the YAML config using `serde` with `derive` macros.
- Implement a function to:
  - Read the config file path from CLI args (use `std::env::args` or `clap` crate).
  - Deserialize the YAML config using `serde_yaml`.
  - Provide defaults for missing fields if necessary.
- Consider making config immutable and thread-safe (`Arc<Config>`) if needed later.

## 3. Logging
- Use the `log` crate with a logger implementation (`env_logger` or `simplelog`).
- Initialize logging early in `main`.
- Write logs to a file as well as optionally stderr/stdout.
- Replicate log messages and levels suitably.

## 4. LDAP Connection and Operations
- Use the `ldap3` crate for LDAP operations:
  - Establish connection and binding with LDAP server.
  - Perform user search queries.
  - Attempt user bind for authentication verification.
- Model LDAP results into Rust structs.
- Properly handle and propagate errors using Rust `Result` and custom error types.

## 5. Authentication Workflow
- Implement the main authentication flow resembling `cmd.Auth()` in Go:
  - Load config.
  - Get user credentials from environment variables.
  - Connect and bind to LDAP server.
  - Search user details using search base and filters.
  - Attempt bind with user credentials.
  - Log success or failures accordingly.
  - Output user info to stdout on success.
  - Exit with status codes.

## 6. CLI Interface
- Implement CLI argument parsing with `clap` or `structopt`.
- Allow config path override via CLI.
- Provide helpful error messages to users.
- Ensure proper exit codes on error or success.

## 7. Error Handling
- Define a custom error enum encompassing config errors, LDAP errors, and IO errors.
- Use `thiserror` crate for ergonomics.
- Use `anyhow` or similar if you prefer general error handling.
- Avoid panics; rather, return errors and handle gracefully in `main`.

## 8. Testing
- Write unit tests for:
  - Config parsing.
  - Error handling.
  - Parsing environment variables for credentials.
- Consider integration tests using a local or test LDAP server.

## 9. Documentation and Examples
- Write Rustdoc comments for public functions and types.
- Update README with instructions for building and running the Rust version.
- Provide example `config.yml` and usage notes.

## 10. Packaging and Deployment
- Use `cargo build --release` to create optimized builds.
- Optionally create Dockerfile or distribution binaries.
- Ensure the CLI behaves like the original Go binary for smooth substitution.
