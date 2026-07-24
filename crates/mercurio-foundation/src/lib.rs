//! Public facade for the language-neutral Mercurio semantic platform.
//!
//! Applications should prefer this crate over depending directly on Mercurio's
//! implementation crates. Language integrations, including SysML, build on the
//! contracts and runtime exposed here.

pub use mercurio_core::*;

#[cfg(test)]
mod tests {
    use super::{Graph, Runtime};

    #[test]
    fn facade_exposes_model_and_runtime_types() {
        fn accepts_public_types(_: Option<Graph>, _: Option<Runtime>) {}

        accepts_public_types(None, None);
    }
}
