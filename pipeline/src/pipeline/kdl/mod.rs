//! KDL pipeline definition language.
//!
//! Converts between [KDL](https://kdl.dev) documents and [`XvcPipelineSchema`],
//! the same schema `xvc pipeline export` / `import` use for JSON and YAML.
//! A KDL document describes the pipeline as a graph: `node` children declare
//! dependencies, `step` children declare the commands that connect them.
//!
//! ```kdl
//! pipeline "train-model" {
//!     node "images" glob-items="data/images/*"
//!     node "params" param="params.yaml::train"
//!
//!     step "train" command="python src/train.py" when="by_dependencies" {
//!         deps "images" "params" file="data/train.bin"
//!         after "preprocess"
//!         outs {
//!             file "models/model.pt"
//!             metric "results/metrics.json"
//!         }
//!     }
//! }
//! ```
//!
//! Node type properties use the same spec syntax as the corresponding
//! `xvc pipeline step dependency` flags. Runtime bookkeeping (digests,
//! metadata) never appears in KDL documents.
//!
//! Unit tests for this module live in `iesahin/xvc-mono`'s `xvc-test` crate
//! (`test_pipeline_kdl_schema.rs`), which path-depends on this crate and can
//! exercise `pipeline_schema_from_kdl` / `pipeline_schema_to_kdl` directly.
mod generate;
mod parse;

pub use generate::pipeline_schema_to_kdl;
pub use parse::pipeline_schema_from_kdl;
