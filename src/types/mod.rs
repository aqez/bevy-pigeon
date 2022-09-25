//! Provides network-able types for common bevy types.
//!
//! Types:
//!  - [Transform]
//!  - [OrthographicProjection]
//!  - [AmbientLight]
//!  - [DirectionalLight]
//!  - [PointLight]
//!  - [Name]
//!  - [Visibility]
//!  - [AlphaMode]
//!  - [EulerRot]
//!
//! If you think other network-able types would be helpful to many users, and think it should be
//! included here, please send a PR.

#[cfg(feature = "render-types")]
mod light;
#[cfg(feature = "render-types")]
mod misc;
mod transform;

#[cfg(feature = "render-types")]
pub use light::*;
#[cfg(feature = "render-types")]
pub use misc::*;

pub use transform::*;
