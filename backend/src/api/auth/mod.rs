use crate::AppState;
use axum::{routing::post, Router};

pub mod error;
pub mod google;
pub mod jwt;
pub(crate) mod login;
pub mod middleware;

// used for OpenAPI generation, maybe not picked up by the compiler as "used".
#[allow(unused)]
pub use login::login;

/// Defines routes for the /auth path.
pub fn router() -> Router<AppState> {
    Router::<AppState>::new().route("/login", post(login::login))
}
