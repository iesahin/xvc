use std::collections::HashSet;

use crate::Result;
use serde::{Deserialize, Serialize};
use xvc_ecs::{HStore, Storable, XvcEntity, XvcStore};
use xvc_logging::warn;

/// Shows which information is identical, missing or different in diff calculations.
///
/// We use this to compare anything that's storable.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash, Clone)]
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

impl<T> std::default::Default for Diff<T>
where
    T: Storable,
{
    fn default() -> Self {
        Diff::<T>::Skipped
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
        let record_value = records.get(&xe);
        let actual_value = actuals.get(&xe);

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
/// If `add_new` is `true`, we add new values to `records`. (See [Diff::RecordMissing])
/// If `remove_missing` is `true`, we remove missing values from `records`. (See [Diff::ActualMissing])
/// We always update the values that are different. (See [Diff::Different])
pub fn update_with_actual<T: Storable>(
    records: &mut XvcStore<T>,
    diffs: DiffStore<T>,
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
            Diff::ActualMissing { record } => {
                if remove_missing {
                    records.remove(*xe);
                }
            }
            Diff::Different { record, actual } => {
                records.insert(*xe, actual.clone());
            }
            Diff::Skipped => {}
        }
    }
    Ok(())
}

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
            Diff::ActualMissing { record } => {
                if remove_missing {
                    new_records.remove(*xe);
                }
            }
            Diff::Different { record, actual } => {
                new_records.insert(*xe, actual.clone());
            }
        }
    }
    Ok(new_records)
}

pub struct Diff2<T, U>
where
    T: Storable,
    U: Storable,
{
    diff1: Diff<T>,
    diff2: Diff<U>,
}

pub struct Diff3<T, U, V>
where
    T: Storable,
    U: Storable,
    V: Storable,
{
    diff1: Diff<T>,
    diff2: Diff<U>,
    diff3: Diff<V>,
}

pub struct Diff4<T, U, V, W>
where
    T: Storable,
    U: Storable,
    V: Storable,
    W: Storable,
{
    pub diff1: Diff<T>,
    pub diff2: Diff<U>,
    pub diff3: Diff<V>,
    pub diff4: Diff<W>,
}

pub struct Diff5<T, U, V, W, X>
where
    T: Storable,
    U: Storable,
    V: Storable,
    W: Storable,
    X: Storable,
{
    diff1: Diff<T>,
    diff2: Diff<U>,
    diff3: Diff<V>,
    diff4: Diff<W>,
    diff5: Diff<X>,
}

impl<T: Storable> Diff<T> {
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

impl<T: Storable, U: Storable> Diff2<T, U> {
    pub fn changed(&self) -> bool {
        self.diff1.changed() || self.diff2.changed()
    }
}

impl<T: Storable, U: Storable, V: Storable> Diff3<T, U, V> {
    pub fn changed(&self) -> bool {
        self.diff1.changed() || self.diff2.changed() || self.diff3.changed()
    }
}

impl<T: Storable, U: Storable, V: Storable, W: Storable> Diff4<T, U, V, W> {
    pub fn changed(&self) -> bool {
        self.diff1.changed() || self.diff2.changed() || self.diff3.changed() || self.diff4.changed()
    }
}

impl<T: Storable, U: Storable, V: Storable, W: Storable, X: Storable> Diff5<T, U, V, W, X> {
    pub fn changed(&self) -> bool {
        self.diff1.changed()
            || self.diff2.changed()
            || self.diff3.changed()
            || self.diff4.changed()
            || self.diff5.changed()
    }
}

pub struct DiffStore2<T, U>(pub DiffStore<T>, pub DiffStore<U>)
where
    T: Storable,
    U: Storable;

impl<T, U> DiffStore2<T, U>
where
    T: Storable,
    U: Storable,
{
    pub fn get_diff2(&self, xe: XvcEntity) -> Diff2<T, U> {
        Diff2 {
            diff1: self.0.get(&xe).cloned().expect("Missing diff1"),
            diff2: self.1.get(&xe).cloned().expect("Missing diff2"),
        }
    }
}

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
    pub fn get_diff3(&self, xe: XvcEntity) -> Diff3<T, U, V> {
        Diff3 {
            diff1: self.0.get(&xe).cloned().expect("Missing diff1"),
            diff2: self.1.get(&xe).cloned().expect("Missing diff2"),
            diff3: self.2.get(&xe).cloned().expect("Missing diff3"),
        }
    }
}
pub struct DiffStore4<T: Storable, U: Storable, V: Storable, W: Storable>(
    DiffStore<T>,
    DiffStore<U>,
    DiffStore<V>,
    DiffStore<W>,
);

impl<T: Storable, U: Storable, V: Storable, W: Storable> DiffStore4<T, U, V, W> {
    pub fn get_diff4(&self, xe: XvcEntity) -> Diff4<T, U, V, W> {
        Diff4 {
            diff1: self.0.get(&xe).cloned().expect("Missing diff1"),
            diff2: self.1.get(&xe).cloned().expect("Missing diff2"),
            diff3: self.2.get(&xe).cloned().expect("Missing diff3"),
            diff4: self.3.get(&xe).cloned().expect("Missing diff4"),
        }
    }
}

pub struct DiffStore5<T, U, V, W, X>(
    DiffStore<T>,
    DiffStore<U>,
    DiffStore<V>,
    DiffStore<W>,
    DiffStore<X>,
)
where
    T: Storable,
    U: Storable,
    V: Storable,
    W: Storable,
    X: Storable;

impl<T, U, V, W, X> DiffStore5<T, U, V, W, X>
where
    T: Storable,
    U: Storable,
    V: Storable,
    W: Storable,
    X: Storable,
{
    pub fn get_diff5(&self, xe: XvcEntity) -> Diff5<T, U, V, W, X> {
        Diff5 {
            diff1: self.0.get(&xe).cloned().expect("Missing diff1"),
            diff2: self.1.get(&xe).cloned().expect("Missing diff2"),
            diff3: self.2.get(&xe).cloned().expect("Missing diff3"),
            diff4: self.3.get(&xe).cloned().expect("Missing diff4"),
            diff5: self.4.get(&xe).cloned().expect("Missing diff5"),
        }
    }
}
