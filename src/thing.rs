use std::fmt::Display;

use serde::{Deserialize, Serialize};
use bevy::prelude::*;

#[derive(Serialize, Deserialize, Reflect, Clone, PartialEq, ::prost::Message, Hash, Eq)]
pub struct Thing {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String
}

impl Display for Thing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.id.fmt(f)
    }
}