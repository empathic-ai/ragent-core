#![allow(warnings)]
#![allow(unused)]

pub mod task;
pub mod thing;

pub mod prelude {
    pub use crate::task::*;
    pub use crate::thing::*;
}
