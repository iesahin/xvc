//! Xvc Entity-Component System is the basic storage mechanism behind Xvc.
//! It defines an integer-based entity type ([XvcEntity]), a trait to be implemented by components
//! (structs) and stores to save, load and operate on these components.
//!
//! It's an alternative to an object-oriented design where the relations between classes should be
//! known beforehand.
//! It allows to implement new components for entities at any point in the
//! evolution of software.
//! These components can have 1-1, 1-N or M-N relationships.
//!
//! In a sense components are analogous to database tables, and entities are primary keys.
//! [XvcStore] can be considered akin to a table, and [R11Store], [R1NStore] and [RMNStore] can be
//! considered as relations.
#![warn(missing_docs)]
#![forbid(unsafe_code)]
pub mod ecs;
pub mod error;

pub use ecs::hstore::HStore;
pub use ecs::hstore::SharedHStore;
pub use ecs::init_generator;
pub use ecs::load_generator;
pub use ecs::r11store::R11Store;
pub use ecs::r1nstore::ChildEntity;
pub use ecs::r1nstore::R1NStore;
pub use ecs::rmnstore::RMNStore;
pub use ecs::storable::Storable;
pub use ecs::vstore::VStore;
pub use ecs::xvcstore::SharedXStore;
pub use ecs::xvcstore::XvcStore;
pub use ecs::XvcEntity;
pub use ecs::XvcEntityGenerator;

pub use error::Error;
pub use error::Result;
