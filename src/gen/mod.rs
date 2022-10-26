#![allow(clippy::derive_partial_eq_without_eq)]

pub mod backend_beta {
    include!("lekko.backend.v1beta1.rs");
}
pub mod feature_beta {
    include!("lekko.feature.v1beta1.rs");
}
pub mod rules_beta {
    include!("lekko.rules.v1beta1.rs");
}
