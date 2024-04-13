use serde::{Deserialize, Serialize};
use bevy::prelude::*;

#[derive(Serialize, Deserialize, Reflect, Clone, PartialEq, ::prost::Message)]
pub struct Thing {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String
}