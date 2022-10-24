//! Contains [Storable] trait that marks a type suitable to use in [crate::XvcStore].
use serde::{Deserialize, Serialize};

/// Marks the traits as storable in XvcStore
///
/// It requires to implement an fn to describe the store names.
/// It also requires the implementer to be (serde) serializable, and to implement Clone, Debug, Ord
/// and PartialEq.
///
/// These are to use JSON as storage format, and to be able to use BTreeMaps as data structures.
pub trait Storable:
    Serialize + for<'lt> Deserialize<'lt> + Clone + std::fmt::Debug + Ord + PartialEq
{
    /// A string representation for the type.
    ///
    /// By convention this should be in kebab-case, like `xvc-path` for [xvc-core::XvcPath].
    /// Xvc uses this to create filenames in serialization.
    ///
    /// See [crate::persist] macro to specify this representation conveniently.
    fn type_description() -> String;
}

impl Storable for String {
    fn type_description() -> String {
        "string".to_string()
    }
}

impl<T: Storable> Storable for Option<T> {
    fn type_description() -> String {
        format!("option-{}", <T as Storable>::type_description())
    }
}

impl<T: Storable, U: Storable> Storable for (T, U) {
    fn type_description() -> String {
        format!(
            "tuple-{}-{}",
            <T as Storable>::type_description(),
            <U as Storable>::type_description()
        )
    }
}
impl Storable for i32 {
    fn type_description() -> String {
        "i32".to_string()
    }
}

impl Storable for usize {
    fn type_description() -> String {
        "usize".to_string()
    }
}
