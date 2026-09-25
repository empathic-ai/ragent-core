use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{fmt::Display, marker::PhantomData};
use uuid::Uuid;

#[cfg(feature = "bevy")]
use bevy::prelude::*;
