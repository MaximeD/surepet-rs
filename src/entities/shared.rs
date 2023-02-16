use dyn_partial_eq::*;

#[dyn_partial_eq]
pub trait Information: std::fmt::Debug {
    fn information(&self) -> String;
}
