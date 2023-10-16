//! Compare two stores and find out which values are different.
//!
//! [Diff] keeps possible differences between two stores of the same type.
//! [DiffStore] keeps the diffs for all entities in a stores.
//!
//! Two [Storable] types are compared and the result is a [Diff] of the same
//! type.
//! These diffs make up a store, which is a [DiffStore].

use std::collections::HashSet;

use crate::Result;
use serde::{Deserialize, Serialize};
use xvc_ecs::{HStore, Storable, XvcEntity, XvcStore};
use xvc_logging::{warn, watch};

/// Shows which information is identical, missing or different in diff calculations.
///
/// We use this to compare anything that's storable.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash, Clone, Default)]
#[serde(bound = "T: Serialize, for<'lt> T: Deserialize<'lt>")]
pub enum Diff<T: Storable> {
    /// Both record and actual values are identical.
    Identical,
    /// We don't have the record, but we have the actual value
    RecordMissing {
        /// The actual value found (probably) in workspace
        actual: T,
    },
    /// We have the record, but we don't have the actual value
    ActualMissing {
        /// The record value found in the stores
        record: T,
    },
    /// Both record and actual values are present, but they differ
    Different {
        /// The value found in store
        record: T,
        /// The value found in workspace
        actual: T,
    },
    /// We skipped this comparison.
    /// It's not an error, but it means we didn't compare this field.
    /// It may be shortcutted, we don't care or irrelevant.
    #[default]
    Skipped,
}

impl<T> Storable for Diff<T>
where
    T: Storable,
{
    fn type_description() -> String {
        format!("diff-{}", T::type_description())
    }
}

/// Keeping track of differences between two stores of the same type.
pub type DiffStore<T> = HStore<Diff<T>>;

/// Compare `records` with `actuals` and return the missing or changed values.
/// This is used find out when something changes in the workspace.
///
/// If `subset` is `None`, we compare all entities in both stores. Otherwise we only compare the entities in `subset`.
pub fn diff_store<T: Storable>(
    records: &XvcStore<T>,
    actuals: &HStore<T>,
    subset: Option<&HashSet<XvcEntity>>,
) -> DiffStore<T> {
    let mut all_entities = HashSet::new();
    let entities = if let Some(subset) = subset {
        subset
    } else {
        all_entities.extend(records.keys().chain(actuals.keys()).copied());
        &all_entities
    };

    let mut diff_store = HStore::new();

    for xe in entities {
        let record_value = records.get(xe);
        let actual_value = actuals.get(xe);

        match (record_value, actual_value) {
            (None, None) => {
                warn!("Both record and actual values are missing for {:?}", xe);
                continue;
            }
            (None, Some(actual_value)) => {
                diff_store.insert(
                    *xe,
                    Diff::RecordMissing {
                        actual: actual_value.clone(),
                    },
                );
            }
            (Some(record_value), None) => {
                diff_store.insert(
                    *xe,
                    Diff::ActualMissing {
                        record: record_value.clone(),
                    },
                );
            }
            (Some(record_value), Some(actual_value)) => {
                if record_value == actual_value {
                    diff_store.insert(*xe, Diff::Identical);
                } else {
                    diff_store.insert(
                        *xe,
                        Diff::Different {
                            record: record_value.clone(),
                            actual: actual_value.clone(),
                        },
                    );
                }
            }
        }
    }
    diff_store
}

/// Update `records` loaded from store with the changed values in `diffs`.
/// When the actual values are changed and we want to update the store, we use this function.
///
/// It always updates values that are changed. (See [Diff::Different])
/// If `add_new` is `true`, we add new values to `records`. (See [Diff::RecordMissing])
/// If `remove_missing` is `true`, we remove missing values from `records`. (See [Diff::ActualMissing])
///
/// See [apply_diff] for a version that doesn't modify the original store.
pub fn update_with_actual<T: Storable>(
    records: &mut XvcStore<T>,
    diffs: &DiffStore<T>,
    add_new: bool,
    remove_missing: bool,
) -> Result<()> {
    for (xe, diff) in diffs.iter() {
        match diff {
            Diff::Identical => {}
            Diff::RecordMissing { actual } => {
                if add_new {
                    records.insert(*xe, actual.clone());
                }
            }
            Diff::ActualMissing { .. } => {
                if remove_missing {
                    records.remove(*xe);
                }
            }
            Diff::Different { actual, .. } => {
                records.insert(*xe, actual.clone());
            }
            Diff::Skipped => {}
        }
    }
    Ok(())
}

/// Create a new store from `records` and with the changed values in `diffs`.
/// When the actual values are changed and we want a new store with new values, we use this function.
///
/// It always updates values that are changed. (See [Diff::Different])
/// If `add_new` is `true`, we add new values to `records`. (See [Diff::RecordMissing])
/// If `remove_missing` is `true`, we remove missing values from `records`. (See [Diff::ActualMissing])
///
/// See [update_with_actual] for a version that modifies the original store in
/// place.
pub fn apply_diff<T: Storable>(
    records: &XvcStore<T>,
    diffs: &DiffStore<T>,
    add_new: bool,
    remove_missing: bool,
) -> Result<XvcStore<T>> {
    let mut new_records = records.clone();
    for (xe, diff) in diffs.iter() {
        match diff {
            Diff::Identical | Diff::Skipped => {}
            Diff::RecordMissing { actual } => {
                if add_new {
                    new_records.insert(*xe, actual.clone());
                }
            }
            Diff::ActualMissing { .. } => {
                if remove_missing {
                    new_records.remove(*xe);
                }
            }
            Diff::Different { actual, .. } => {
                new_records.insert(*xe, actual.clone());
            }
        }
    }
    Ok(new_records)
}

impl<T: Storable> Diff<T> {
    /// Return true if the diff is not [Diff::Identical] or [Diff::Skipped].
    /// This is used to find out if <T> has changed.
    pub fn changed(&self) -> bool {
        match self {
            Diff::Identical => false,
            Diff::RecordMissing { .. } => true,
            Diff::ActualMissing { .. } => true,
            Diff::Different { .. } => true,
            Diff::Skipped => false,
        }
    }
}

/// Keep two diffs for the same set of entities
///
/// This is used, for example, to keep path and metadata diffs for the same
/// entities.
pub struct DiffStore2<T, U>(pub DiffStore<T>, pub DiffStore<U>)
where
    T: Storable,
    U: Storable;

impl<T, U> DiffStore2<T, U>
where
    T: Storable,
    U: Storable,
{
    /// Return a tuple of diffs for the same entity
    pub fn diff_tuple(&self, xe: XvcEntity) -> (Diff<T>, Diff<U>) {
        (
            self.0.get(&xe).cloned().expect("Missing diff1"),
            self.1.get(&xe).cloned().expect("Missing diff2"),
        )
    }
}

/// Keep three diffs for the same set of entities
///
/// This is used, for example, to keep path, metadata and digest diffs for the
/// same set of entities.
pub struct DiffStore3<T, U, V>(pub DiffStore<T>, pub DiffStore<U>, pub DiffStore<V>)
where
    T: Storable,
    U: Storable,
    V: Storable;

impl<T, U, V> DiffStore3<T, U, V>
where
    T: Storable,
    U: Storable,
    V: Storable,
{
    /// Return a tuple of diffs for the same entity
    pub fn diff_tuple(&self, xe: XvcEntity) -> (Diff<T>, Diff<U>, Diff<V>) {
        (
            self.0.get(&xe).cloned().expect("Missing diff1"),
            self.1.get(&xe).cloned().expect("Missing diff2"),
            self.2.get(&xe).cloned().expect("Missing diff3"),
        )
    }
}

/// Keep four diffs for the same set of entities
///
/// This is used, for example, to keep path, metadata, digest and collection
/// diffs for certain entities
pub struct DiffStore4<T: Storable, U: Storable, V: Storable, W: Storable>(
    DiffStore<T>,
    DiffStore<U>,
    DiffStore<V>,
    DiffStore<W>,
);

impl<T: Storable, U: Storable, V: Storable, W: Storable> DiffStore4<T, U, V, W> {
    /// Return a tuple of diffs for the same entity
    pub fn diff_tuple(&self, xe: XvcEntity) -> (Diff<T>, Diff<U>, Diff<V>, Diff<W>) {
        (
            self.0.get(&xe).cloned().expect("Missing diff1"),
            self.1.get(&xe).cloned().expect("Missing diff2"),
            self.2.get(&xe).cloned().expect("Missing diff3"),
            self.3.get(&xe).cloned().expect("Missing diff4"),
        )
    }
}

/// Used to find out if record and actual are different for type T.
pub trait Diffable {
    /// The type of the entity to compare.
    type Item: Storable;

    /// ⚠️ Usually you must update actual's metadata and timestamp before calling this.
    /// Use diff_superficial and diff_thorough for shortcut comparisons. (e.g. when metadata is not changed, no need to
    /// compare the content. )
    ///
    /// This is to convert optional entities to diffs.
    /// e.g. a file may be missing from the disk, but it may exist in the records.
    /// ((Some(record), None) -> Diff::ActualMissing)
    fn diff(record: Option<&Self::Item>, actual: Option<&Self::Item>) -> Diff<Self::Item> {
        watch!(record);
        watch!(actual);
        match (record, actual) {
            (None, None) => unreachable!("Both record and actual are None"),
            (None, Some(actual)) => Diff::RecordMissing {
                actual: actual.clone(),
            },
            (Some(record), None) => Diff::ActualMissing {
                record: record.clone(),
            },
            (Some(record), Some(actual)) => {
                if record == actual {
                    Diff::Identical
                } else {
                    Diff::Different {
                        record: record.clone(),
                        actual: actual.clone(),
                    }
                }
            }
        }
    }

    /// This is to compare two entities with a quick comparison.
    /// e.g. metadata of a file, timestamp of a URL etc.
    /// You may need to update actual's metadata or timestamp before calling this.
    fn diff_superficial(record: &Self::Item, actual: &Self::Item) -> Diff<Self::Item> {
        if record == actual {
            Diff::Identical
        } else {
            Diff::Different {
                record: record.clone(),
                actual: actual.clone(),
            }
        }
    }

    /// This is to calculate two entities with a thorough comparison.
    /// e.g. content of a file, content of a URL etc.
    /// You may need to update actual's content before calling this.
    fn diff_thorough(record: &Self::Item, actual: &Self::Item) -> Diff<Self::Item> {
        Self::diff_superficial(record, actual)
    }
}
