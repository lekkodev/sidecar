// This is the generated code from the cli repo & buf module.
// Run `make generate` to recreate.
// This file re-exports an auto-generated include file by prost-gen-prost-crate but adds
// clippy allows, which is incorrect b/c protobuf generated code doesn't implement Eq by
// default since they want you to use protobuf's equality specifically.
pub mod mod_cli;
#[allow(clippy::derive_partial_eq_without_eq)]
// mod mod_cli;
// mod mod_sdk;
// pub use mod_cli::*;
// pub use mod_sdk::*;
pub mod mod_sdk;
