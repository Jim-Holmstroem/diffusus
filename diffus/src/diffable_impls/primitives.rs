use crate::{edit::Edit, Diffable};

macro_rules! primitive_impl {
    ($($typ:ty),*) => {
        $(
            impl<'a> Diffable<'a> for $typ {
                type Diff = (&'a $typ, &'a $typ);

                fn diff(&'a self, other: &'a Self) -> Edit<Self::Diff> {
                    if self == other {
                        Edit::Copy
                    } else {
                        Edit::Change((self, other))
                    }
                }
            }
        )*
    }
}

primitive_impl! { i64, i32, i16, i8, u64, u32, u16, u8, char, bool, isize, usize, () }

#[cfg(feature = "uuid-impl")]
primitive_impl! { uuid::Uuid }

#[cfg(feature = "snake_case-impl")]
primitive_impl! { snake_case::SnakeCase }
