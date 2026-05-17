pub mod auth;
pub mod health;

pub use auth::{get_me, login, patch_me, register};
pub use health::{health_check};