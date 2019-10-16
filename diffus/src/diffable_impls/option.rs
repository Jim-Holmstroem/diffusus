use crate::{
    edit::{enm, Edit},
    Diffable,
};

#[cfg(feature = "serialize-impl")]
use serde::Serialize;

macro_rules! option_impl_constraint {
    (: $($constraints:ident),*) => {
        impl<'a, T: Diffable<'a> $(+$constraints)? + 'a> Diffable<'a> for Option<T> {
            type Diff = enm::Edit<'a, Self, T::Diff>;

            fn diff(&'a self, other: &'a Self) -> Edit<Self::Diff> {
                match (self, other) {
                    (None, None) => Edit::Copy,
                    (Some(a), Some(b)) => match a.diff(&b) {
                        Edit::Copy => Edit::Copy,
                        Edit::Change(d) => Edit::Change(enm::Edit::AssociatedChanged(d)),
                    },
                    _ => Edit::Change(enm::Edit::VariantChanged(self, other)),
                }
            }
        }
    }
}

#[cfg(feature = "serialize-impl")]
option_impl_constraint!{ : Serialize }
#[cfg(not(feature = "serialize-impl"))]
option_impl_constraint!{ : }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_copy() {
        assert!((None as Option<u32>).diff(&None).is_copy());
        assert!(Some(3).diff(&Some(3)).is_copy());
    }

    #[test]
    fn variant_changed() {
        if let Some(enm::Edit::VariantChanged(&None, &Some(3))) = None.diff(&Some(3)).change() {
        } else {
            unreachable!();
        }
    }

    #[test]
    fn associate_change() {
        if let Some(enm::Edit::AssociatedChanged((&1, &2))) = Some(1).diff(&Some(2)).change() {
        } else {
            unreachable!();
        }
    }
}
