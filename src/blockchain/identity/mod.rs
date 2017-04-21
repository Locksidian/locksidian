//! Identity management base module.
//!
//! Re-exports the structures of the `domain`, `dto` and `repository` submodules and exposes a handful
//! batch of utility functions.

mod identity_domain;
mod identity_dto;
mod identity_repository;
pub mod identity_cli;

pub use self::identity_domain::Identity;
pub use self::identity_dto::IdentityDto;
pub use self::identity_repository::{IdentityEntity, IdentityRepository};
pub use self::identity_cli as cli;