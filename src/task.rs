use bevy::{prelude::*, reflect::Typed};
use documented::Documented;

/// A trait implemented by all tasks
pub trait Task: Reflect + Typed + Documented + FromReflect + Clone {
}