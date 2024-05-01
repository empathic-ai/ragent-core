#[cfg(feature = "bevy")]
use bevy::{prelude::*, reflect::Typed};
use documented::Documented;

#[cfg(feature = "bevy")]
/// A trait implemented by all tasks
pub trait Task: Reflect + Typed + Documented + FromReflect + Clone {
}

#[cfg(not(feature = "bevy"))]
/// A trait implemented by all tasks
pub trait Task: Documented + Clone {
}