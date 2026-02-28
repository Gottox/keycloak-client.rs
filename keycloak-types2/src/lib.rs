#[cfg(feature = "k8s-schema")]
mod k8s;
#[rustfmt::skip]
mod schema_gen;
pub use schema_gen::*;
