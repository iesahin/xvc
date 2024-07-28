# Compiling Xvc without default features

You may want to customize the [feature set][features] when you want a smaller binary size. Not everyone needs all storage options and turning off them may result in smaller binary sizes.

When you turn off all remote storage features, async runtime (`tokio`) is also excluded from binary.

````bash
cargo build --no-default-features --release
warning: output filename collision.
The bin target `xvc` in package `xvc-workflow-tests v0.6.9-alpha.1 (/Users/iex/github.com/iesahin/xvc/workflow_tests)` has the same output filename as the bin target `xvc` in package `xvc v0.6.9-alpha.1 (/Users/iex/github.com/iesahin/xvc/lib)`.
Colliding filename is: /Users/iex/github.com/iesahin/xvc/target/release/xvc
The targets should have unique names.
Consider changing their names to be unique or compiling them separately.
This may become a hard error in the future; see <https://github.com/rust-lang/cargo/issues/6313>.
warning: output filename collision.
The bin target `xvc` in package `xvc-workflow-tests v0.6.9-alpha.1 (/Users/iex/github.com/iesahin/xvc/workflow_tests)` has the same output filename as the bin target `xvc` in package `xvc v0.6.9-alpha.1 (/Users/iex/github.com/iesahin/xvc/lib)`.
Colliding filename is: /Users/iex/github.com/iesahin/xvc/target/release/xvc.dSYM
The targets should have unique names.
Consider changing their names to be unique or compiling them separately.
This may become a hard error in the future; see <https://github.com/rust-lang/cargo/issues/6313>.
   Compiling proc-macro2 v1.0.86
   Compiling unicode-ident v1.0.12
   Compiling libc v0.2.155
   Compiling cfg-if v1.0.0
   Compiling autocfg v1.3.0
   Compiling version_check v0.9.4
   Compiling serde v1.0.204
   Compiling memchr v2.7.4
   Compiling once_cell v1.19.0
   Compiling crossbeam-utils v0.8.20
   Compiling bitflags v2.6.0
   Compiling zerocopy v0.7.35
   Compiling allocator-api2 v0.2.18
   Compiling ahash v0.8.11
   Compiling bytes v1.6.1
   Compiling log v0.4.22
   Compiling itoa v1.0.11
   Compiling pin-project-lite v0.2.14
   Compiling core-foundation-sys v0.8.6
   Compiling aho-corasick v1.1.3
   Compiling rustix v0.38.34
   Compiling regex-syntax v0.8.4
   Compiling futures-core v0.3.30
   Compiling typenum v1.17.0
   Compiling equivalent v1.0.1
   Compiling ident_case v1.0.1
   Compiling fnv v1.0.7
   Compiling num-traits v0.2.19
   Compiling slab v0.4.9
   Compiling generic-array v0.14.7
   Compiling futures-sink v0.3.30
   Compiling errno v0.3.9
   Compiling regex-automata v0.4.7
   Compiling mio v0.8.11
   Compiling quote v1.0.36
   Compiling num_cpus v1.16.0
   Compiling syn v2.0.72
   Compiling socket2 v0.5.7
   Compiling crossbeam-epoch v0.9.18
   Compiling crossbeam-channel v0.5.13
   Compiling tokio v1.38.1
   Compiling hashbrown v0.14.5
   Compiling crossbeam-deque v0.8.5
   Compiling core-foundation v0.9.4
   Compiling indexmap v2.2.6
   Compiling crunchy v0.2.2
   Compiling thiserror v1.0.63
   Compiling fastrand v2.1.0
   Compiling ryu v1.0.18
   Compiling tempfile v3.10.1
   Compiling http v0.2.12
   Compiling security-framework-sys v2.11.1
   Compiling is-terminal v0.4.12
   Compiling futures-channel v0.3.30
   Compiling lock_api v0.4.12
   Compiling native-tls v0.2.12
   Compiling either v1.13.0
   Compiling serde_derive v1.0.204
   Compiling thiserror-impl v1.0.63
   Compiling futures-macro v0.3.30
   Compiling lazy_static v1.5.0
   Compiling tiny-keccak v2.0.2
   Compiling tinyvec_macros v0.1.1
   Compiling anstyle v1.0.7
   Compiling smallvec v1.13.2
   Compiling futures-io v0.3.30
   Compiling rayon-core v1.12.1
   Compiling futures-task v0.3.30
   Compiling pin-utils v0.1.0
   Compiling parking_lot_core v0.9.10
   Compiling futures-util v0.3.30
   Compiling tinyvec v1.8.0
   Compiling colored v1.9.4
   Compiling security-framework v2.11.1
   Compiling bstr v1.9.1
   Compiling getrandom v0.2.15
   Compiling percent-encoding v2.3.1
   Compiling anyhow v1.0.86
   Compiling scopeguard v1.2.0
   Compiling serde_json v1.0.120
   Compiling heck v0.5.0
   Compiling same-file v1.0.6
   Compiling walkdir v2.5.0
   Compiling form_urlencoded v1.2.1
   Compiling rand_core v0.6.4
   Compiling fern v0.6.2
   Compiling unicode-normalization v0.1.23
   Compiling block-buffer v0.10.4
   Compiling crypto-common v0.1.6
   Compiling regex v1.10.5
   Compiling tracing-core v0.1.32
   Compiling subtle v2.6.1
   Compiling httparse v1.9.4
   Compiling utf8parse v0.2.2
   Compiling ppv-lite86 v0.2.17
   Compiling unicode-bidi v0.3.15
   Compiling const-random-macro v0.1.16
   Compiling rand_chacha v0.3.1
   Compiling anstyle-parse v0.2.4
   Compiling tracing v0.1.40
   Compiling digest v0.10.7
   Compiling xvc-logging v0.6.9-alpha.1 (/Users/iex/github.com/iesahin/xvc/logging)
   Compiling idna v0.5.0
   Compiling rayon v1.10.0
   Compiling globset v0.4.14
   Compiling tokio-util v0.7.11
   Compiling is_terminal_polyfill v1.70.0
   Compiling anstyle-query v1.1.0
   Compiling colorchoice v1.0.1
   Compiling rustversion v1.0.17
   Compiling strsim v0.11.1
   Compiling paste v1.0.15
   Compiling try-lock v0.2.5
   Compiling syn v1.0.109
   Compiling want v0.3.1
   Compiling url v2.5.2
   Compiling h2 v0.3.26
   Compiling darling_core v0.20.10
   Compiling anstream v0.6.14
   Compiling const-random v0.1.18
   Compiling rand v0.8.5
   Compiling http-body v0.4.6
   Compiling crossbeam-queue v0.3.11
   Compiling cpufeatures v0.2.12
   Compiling smartstring v1.0.1
   Compiling tower-service v0.3.2
   Compiling httpdate v1.0.3
   Compiling clap_lex v0.7.1
   Compiling system-configuration-sys v0.5.0
   Compiling adler v1.0.2
   Compiling miniz_oxide v0.7.4
   Compiling darling_macro v0.20.10
   Compiling hyper v0.14.30
   Compiling clap_builder v4.5.9
   Compiling crossbeam v0.8.4
   Compiling serde_spanned v0.6.6
   Compiling toml_datetime v0.6.6
   Compiling dlv-list v0.5.2
   Compiling tokio-native-tls v0.3.1
   Compiling clap_derive v4.5.8
   Compiling fsevent-sys v4.1.0
   Compiling filetime v0.2.23
   Compiling crc32fast v1.4.2
   Compiling time-core v0.1.2
   Compiling powerfmt v0.2.0
   Compiling static_assertions v1.1.0
   Compiling byteorder v1.5.0
   Compiling cc v1.1.6
   Compiling hashbrown v0.13.2
   Compiling winnow v0.6.14
   Compiling base64 v0.21.7
   Compiling num-conv v0.1.0
   Compiling time-macros v0.2.18
   Compiling ordered-multimap v0.6.0
   Compiling hyper-tls v0.5.0
   Compiling rmp v0.8.14
   Compiling blake3 v1.5.3
   Compiling deranged v0.3.11
   Compiling clap v4.5.9
   Compiling flate2 v1.0.30
   Compiling toml_edit v0.22.16
   Compiling notify v6.1.1
   Compiling strum_macros v0.26.4
   Compiling darling v0.20.10
   Compiling dashmap v6.0.1
   Compiling itertools v0.13.0
   Compiling dirs-sys-next v0.1.2
   Compiling unsafe-libyaml v0.2.11
   Compiling convert_case v0.4.0
   Compiling rxml_validation v0.9.1
   Compiling strsim v0.9.3
   Compiling home v0.5.9
   Compiling strum v0.26.3
   Compiling bitflags v1.3.2
   Compiling toml v0.8.15
   Compiling time v0.3.36
   Compiling system-configuration v0.5.1
   Compiling darling_core v0.10.2
   Compiling xvc-walker v0.6.9-alpha.1 (/Users/iex/github.com/iesahin/xvc/walker)
   Compiling rxml v0.9.1
   Compiling serde_yaml v0.9.34+deprecated
   Compiling cached_proc_macro v0.22.0
   Compiling directories-next v2.0.0
   Compiling rmp-serde v1.3.0
   Compiling async-compression v0.4.12
   Compiling rust-ini v0.19.0
   Compiling rustls-pemfile v1.0.4
   Compiling keccak v0.1.5
   Compiling sha2 v0.10.8
   Compiling attohttpc v0.26.1
   Compiling quick-xml v0.30.0
   Compiling serde_urlencoded v0.7.1
   Compiling hex v0.4.3
   Compiling futures-executor v0.3.30
   Compiling path-dedot v3.1.1
   Compiling encoding_rs v0.8.34
   Compiling web-time v1.1.0
   Compiling arrayvec v0.7.4
   Compiling constant_time_eq v0.3.0
   Compiling ipnet v2.9.0
   Compiling sync_wrapper v0.1.2
   Compiling mime v0.3.17
   Compiling arrayref v0.3.8
   Compiling cached_proc_macro_types v0.1.1
   Compiling cached v0.53.0
   Compiling reqwest v0.11.27
   Compiling aws-creds v0.36.0
   Compiling path-absolutize v3.1.1
   Compiling futures v0.3.30
   Compiling sha3 v0.10.8
   Compiling xvc-ecs v0.6.9-alpha.1 (/Users/iex/github.com/iesahin/xvc/ecs)
   Compiling xvc-config v0.6.9-alpha.1 (/Users/iex/github.com/iesahin/xvc/config)
   Compiling darling_macro v0.10.2
   Compiling minidom v0.15.2
   Compiling derive_more v0.99.18
   Compiling jwalk v0.8.1
   Compiling uuid v1.10.0
   Compiling relative-path v1.9.3
   Compiling blake2 v0.10.6
   Compiling hmac v0.12.1
   Compiling aws-region v0.25.4
   Compiling maybe-async v0.2.10
   Compiling async-trait v0.1.81
   Compiling tokio-stream v0.1.15
   Compiling pkg-config v0.3.30
   Compiling md5 v0.7.0
   Compiling vcpkg v0.2.15
   Compiling glob v0.3.1
   Compiling seahash v4.1.0
   Compiling darling v0.10.2
   Compiling which v6.0.1
   Compiling rust-s3 v0.34.0
   Compiling subprocess v0.2.9
   Compiling libsqlite3-sys v0.30.0
   Compiling xvc-core v0.6.9-alpha.1 (/Users/iex/github.com/iesahin/xvc/core)
   Compiling iana-time-zone v0.1.60
   Compiling predicates-core v1.0.6
   Compiling doc-comment v0.3.3
   Compiling derive_builder v0.9.0
   Compiling humantime v2.1.0
   Compiling derive_builder_core v0.9.0
   Compiling chrono v0.4.38
   Compiling parking_lot v0.12.3
   Compiling float-cmp v0.9.0
   Compiling reflink v0.1.3
   Compiling parse-size v1.0.0
   Compiling normalize-line-endings v0.3.0
   Compiling difflib v0.4.0
   Compiling termtree v0.4.1
   Compiling predicates-tree v1.0.9
warning: use of deprecated macro `EnumVariantNames`: please use `#[derive(VariantNames)]` instead
  --> core/src/types/hashalgorithm.rs:27:5
   |
27 |     EnumVariantNames,
   |     ^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(deprecated)]` on by default

   Compiling predicates v3.1.0
   Compiling crossterm v0.27.0
   Compiling ignore v0.4.22
   Compiling hashlink v0.9.1
   Compiling fallible-iterator v0.3.0
   Compiling fixedbitset v0.4.2
   Compiling unicode-width v0.1.13
   Compiling assert_cmd v2.0.14
   Compiling fallible-streaming-iterator v0.1.9
   Compiling globwalk v0.9.1
   Compiling comfy-table v7.1.1
   Compiling petgraph v0.6.5
   Compiling tabbycat v0.1.3
   Compiling sad_machine v1.0.0
   Compiling rusqlite v0.32.0
   Compiling git-version-macro v0.3.9
   Compiling wait-timeout v0.2.0
   Compiling git-version v0.3.9
   Compiling assert_fs v1.1.1
   Compiling xvc-test-helper v0.6.9-alpha.1 (/Users/iex/github.com/iesahin/xvc/test_helper)
   Compiling xvc-storage v0.6.9-alpha.1 (/Users/iex/github.com/iesahin/xvc/storage)
warning: unused import: `panic`
  --> storage/src/storage/mod.rs:35:26
   |
35 | use xvc_logging::{error, panic, watch, XvcOutputSender};
   |                          ^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: unused variable: `xvc_root`
   --> storage/src/storage/async_common.rs:273:50
    |
273 |     fn init(&mut self, output: &XvcOutputSender, xvc_root: &XvcRoot) -> Result<XvcStorageInitEvent>
    |                                                  ^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_xvc_root`
    |
    = note: `#[warn(unused_variables)]` on by default

warning: unused variable: `xvc_root`
   --> storage/src/storage/async_common.rs:313:9
    |
313 |         xvc_root: &xvc_core::XvcRoot,
    |         ^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_xvc_root`

warning: unused variable: `xvc_root`
   --> storage/src/storage/async_common.rs:327:9
    |
327 |         xvc_root: &xvc_core::XvcRoot,
    |         ^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_xvc_root`

warning: unused variable: `xvc_root`
   --> storage/src/storage/async_common.rs:340:9
    |
340 |         xvc_root: &xvc_core::XvcRoot,
    |         ^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_xvc_root`

warning: unused variable: `output`
   --> storage/src/storage/generic.rs:448:9
    |
448 |         output: &XvcOutputSender,
    |         ^^^^^^ help: if this is intentional, prefix it with an underscore: `_output`

warning: unused variable: `xvc_root`
   --> storage/src/storage/generic.rs:449:9
    |
449 |         xvc_root: &XvcRoot,
    |         ^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_xvc_root`

warning: unused variable: `path`
   --> storage/src/storage/generic.rs:450:9
    |
450 |         path: &XvcCachePath,
    |         ^^^^ help: if this is intentional, prefix it with an underscore: `_path`

warning: unused variable: `period`
   --> storage/src/storage/generic.rs:451:9
    |
451 |         period: std::time::Duration,
    |         ^^^^^^ help: if this is intentional, prefix it with an underscore: `_period`

warning: unused variable: `output`
   --> storage/src/storage/rsync.rs:475:9
    |
475 |         output: &XvcOutputSender,
    |         ^^^^^^ help: if this is intentional, prefix it with an underscore: `_output`

warning: unused variable: `xvc_root`
   --> storage/src/storage/rsync.rs:476:9
    |
476 |         xvc_root: &XvcRoot,
    |         ^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_xvc_root`

warning: unused variable: `path`
   --> storage/src/storage/rsync.rs:477:9
    |
477 |         path: &XvcCachePath,
    |         ^^^^ help: if this is intentional, prefix it with an underscore: `_path`

warning: unused variable: `period`
   --> storage/src/storage/rsync.rs:478:9
    |
478 |         period: std::time::Duration,
    |         ^^^^^^ help: if this is intentional, prefix it with an underscore: `_period`

warning: missing documentation for a module
 --> storage/src/storage/mod.rs:3:1
  |
3 | pub mod async_common;
  | ^^^^^^^^^^^^^^^^^^^^
  |
note: the lint level is defined here
 --> storage/src/lib.rs:6:9
  |
6 | #![warn(missing_docs)]
  |         ^^^^^^^^^^^^

warning: missing documentation for a module
 --> storage/src/storage/mod.rs:4:1
  |
4 | pub mod common;
  | ^^^^^^^^^^^^^^

warning: missing documentation for a trait
  --> storage/src/storage/async_common.rs:33:1
   |
33 | pub trait XvcS3StorageOperations {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: missing documentation for a method
  --> storage/src/storage/async_common.rs:34:5
   |
34 |     fn storage_prefix(&self) -> String;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: missing documentation for a method
  --> storage/src/storage/async_common.rs:35:5
   |
35 |     fn guid(&self) -> &XvcStorageGuid;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: missing documentation for a method
  --> storage/src/storage/async_common.rs:36:5
   |
36 |     fn get_bucket(&self) -> Result<Bucket>;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: missing documentation for a method
  --> storage/src/storage/async_common.rs:37:5
   |
37 |     fn credentials(&self) -> Result<Credentials>;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: missing documentation for a method
  --> storage/src/storage/async_common.rs:38:5
   |
38 |     fn bucket_name(&self) -> String;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: missing documentation for a method
  --> storage/src/storage/async_common.rs:39:5
   |
39 |     fn build_storage_path(&self, cache_path: &XvcCachePath) -> XvcStoragePath {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: missing documentation for a method
  --> storage/src/storage/async_common.rs:48:5
   |
48 |     fn region(&self) -> String;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: missing documentation for a method
  --> storage/src/storage/async_common.rs:49:5
   |
49 |     async fn write_storage_guid(&self) -> Result<()> {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: use of `async fn` in public traits is discouraged as auto trait bounds cannot be specified
  --> storage/src/storage/async_common.rs:49:5
   |
49 |     async fn write_storage_guid(&self) -> Result<()> {
   |     ^^^^^
   |
   = note: you can suppress this lint if you plan to use the trait only in your own code, or do not care about auto traits like `Send` on the `Future`
   = note: `#[warn(async_fn_in_trait)]` on by default
help: you can alternatively desugar to a normal `fn` that returns `impl Future` and add any desired bounds such as `Send`, but these cannot be relaxed without a breaking API change
   |
49 ~     fn write_storage_guid(&self) -> impl std::future::Future<Output = Result<()>> + Send {async {
50 |         let guid_str = self.guid().to_string();
 ...
63 |         }
64 ~     } }
   |

warning: missing documentation for a method
  --> storage/src/storage/async_common.rs:66:5
   |
66 |     async fn a_init(&mut self, output_snd: &XvcOutputSender) -> Result<XvcStorageInitEvent> {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: use of `async fn` in public traits is discouraged as auto trait bounds cannot be specified
  --> storage/src/storage/async_common.rs:66:5
   |
66 |     async fn a_init(&mut self, output_snd: &XvcOutputSender) -> Result<XvcStorageInitEvent> {
   |     ^^^^^
   |
   = note: you can suppress this lint if you plan to use the trait only in your own code, or do not care about auto traits like `Send` on the `Future`
help: you can alternatively desugar to a normal `fn` that returns `impl Future` and add any desired bounds such as `Send`, but these cannot be relaxed without a breaking API change
   |
66 ~     fn a_init(&mut self, output_snd: &XvcOutputSender) -> impl std::future::Future<Output = Result<XvcStorageInitEvent>> + Send {async {
67 |         let res_response = self.write_storage_guid().await;
 ...
77 |         }
78 ~     } }
   |

warning: missing documentation for a method
  --> storage/src/storage/async_common.rs:80:5
   |
80 | /     async fn a_list(
81 | |         &self,
82 | |         output: &XvcOutputSender,
83 | |         xvc_root: &xvc_core::XvcRoot,
84 | |     ) -> Result<XvcStorageListEvent> {
   | |____________________________________^

warning: use of `async fn` in public traits is discouraged as auto trait bounds cannot be specified
  --> storage/src/storage/async_common.rs:80:5
   |
80 |     async fn a_list(
   |     ^^^^^
   |
   = note: you can suppress this lint if you plan to use the trait only in your own code, or do not care about auto traits like `Send` on the `Future`
help: you can alternatively desugar to a normal `fn` that returns `impl Future` and add any desired bounds such as `Send`, but these cannot be relaxed without a breaking API change
   |
80 ~     fn a_list(
81 |         &self,
82 |         output: &XvcOutputSender,
83 |         xvc_root: &xvc_core::XvcRoot,
84 ~     ) -> impl std::future::Future<Output = Result<XvcStorageListEvent>> + Send {async {
85 |         let credentials = self.credentials()?;
 ...
130|         }
131~     } }
   |

warning: missing documentation for a method
   --> storage/src/storage/async_common.rs:133:5
    |
133 | /     async fn a_send(
134 | |         &self,
135 | |         output_snd: &XvcOutputSender,
136 | |         xvc_root: &xvc_core::XvcRoot,
137 | |         paths: &[xvc_core::XvcCachePath],
138 | |         _force: bool,
139 | |     ) -> crate::Result<super::XvcStorageSendEvent> {
    | |__________________________________________________^

warning: use of `async fn` in public traits is discouraged as auto trait bounds cannot be specified
   --> storage/src/storage/async_common.rs:133:5
    |
133 |     async fn a_send(
    |     ^^^^^
    |
    = note: you can suppress this lint if you plan to use the trait only in your own code, or do not care about auto traits like `Send` on the `Future`
help: you can alternatively desugar to a normal `fn` that returns `impl Future` and add any desired bounds such as `Send`, but these cannot be relaxed without a breaking API change
    |
133 ~     fn a_send(
134 |         &self,
  ...
138 |         _force: bool,
139 ~     ) -> impl std::future::Future<Output = crate::Result<super::XvcStorageSendEvent>> + Send {async {
140 |         let mut copied_paths = Vec::<XvcStoragePath>::new();
  ...
172 |         })
173 ~     } }
    |

warning: missing documentation for a method
   --> storage/src/storage/async_common.rs:175:5
    |
175 | /     async fn a_receive(
176 | |         &self,
177 | |         output_snd: &XvcOutputSender,
178 | |         paths: &[xvc_core::XvcCachePath],
179 | |         _force: bool,
180 | |     ) -> Result<(XvcStorageTempDir, XvcStorageReceiveEvent)> {
    | |____________________________________________________________^

warning: use of `async fn` in public traits is discouraged as auto trait bounds cannot be specified
   --> storage/src/storage/async_common.rs:175:5
    |
175 |     async fn a_receive(
    |     ^^^^^
    |
    = note: you can suppress this lint if you plan to use the trait only in your own code, or do not care about auto traits like `Send` on the `Future`
help: you can alternatively desugar to a normal `fn` that returns `impl Future` and add any desired bounds such as `Send`, but these cannot be relaxed without a breaking API change
    |
175 ~     fn a_receive(
176 |         &self,
  ...
179 |         _force: bool,
180 ~     ) -> impl std::future::Future<Output = Result<(XvcStorageTempDir, XvcStorageReceiveEvent)>> + Send {async {
181 |         let mut copied_paths = Vec::<XvcStoragePath>::new();
  ...
217 |         ))
218 ~     } }
    |

warning: missing documentation for a method
   --> storage/src/storage/async_common.rs:220:5
    |
220 | /     async fn a_delete(
221 | |         &self,
222 | |         output: &XvcOutputSender,
223 | |         paths: &[XvcCachePath],
224 | |     ) -> Result<XvcStorageDeleteEvent> {
    | |______________________________________^

warning: use of `async fn` in public traits is discouraged as auto trait bounds cannot be specified
   --> storage/src/storage/async_common.rs:220:5
    |
220 |     async fn a_delete(
    |     ^^^^^
    |
    = note: you can suppress this lint if you plan to use the trait only in your own code, or do not care about auto traits like `Send` on the `Future`
help: you can alternatively desugar to a normal `fn` that returns `impl Future` and add any desired bounds such as `Send`, but these cannot be relaxed without a breaking API change
    |
220 ~     fn a_delete(
221 |         &self,
222 |         output: &XvcOutputSender,
223 |         paths: &[XvcCachePath],
224 ~     ) -> impl std::future::Future<Output = Result<XvcStorageDeleteEvent>> + Send {async {
225 |         let mut deleted_paths = Vec::<XvcStoragePath>::new();
  ...
240 |         })
241 ~     } }
    |

warning: missing documentation for a method
   --> storage/src/storage/async_common.rs:243:5
    |
243 | /     async fn a_share(
244 | |         &self,
245 | |         output: &XvcOutputSender,
246 | |         path: &XvcCachePath,
247 | |         duration: std::time::Duration,
248 | |     ) -> Result<XvcStorageExpiringShareEvent> {
    | |_____________________________________________^

warning: use of `async fn` in public traits is discouraged as auto trait bounds cannot be specified
   --> storage/src/storage/async_common.rs:243:5
    |
243 |     async fn a_share(
    |     ^^^^^
    |
    = note: you can suppress this lint if you plan to use the trait only in your own code, or do not care about auto traits like `Send` on the `Future`
help: you can alternatively desugar to a normal `fn` that returns `impl Future` and add any desired bounds such as `Send`, but these cannot be relaxed without a breaking API change
    |
243 ~     fn a_share(
244 |         &self,
  ...
247 |         duration: std::time::Duration,
248 ~     ) -> impl std::future::Future<Output = Result<XvcStorageExpiringShareEvent>> + Send {async {
249 |         let bucket = self.get_bucket()?;
  ...
268 |         })
269 ~     } }
    |

   Compiling xvc-file v0.6.9-alpha.1 (/Users/iex/github.com/iesahin/xvc/file)
warning: unused import: `HStore`
 --> file/src/share/mod.rs:7:15
  |
7 | use xvc_ecs::{HStore, XvcStore};
  |               ^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: missing documentation for a function
  --> file/src/share/mod.rs:29:1
   |
29 | pub fn cmd_share(output_snd: &XvcOutputSender, xvc_root: &XvcRoot, opts: ShareCLI) -> Result<()> {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
note: the lint level is defined here
  --> file/src/lib.rs:7:9
   |
7  | #![warn(missing_docs)]
   |         ^^^^^^^^^^^^

warning: `xvc-core` (lib) generated 1 warning
   Compiling xvc-pipeline v0.6.9-alpha.1 (/Users/iex/github.com/iesahin/xvc/pipeline)
   Compiling xvc v0.6.9-alpha.1 (/Users/iex/github.com/iesahin/xvc/lib)
warning: missing documentation for a function
   --> lib/src/cli/mod.rs:212:1
    |
212 | pub fn dispatch_with_root(cli_opts: cli::XvcCLI, xvc_root_opt: XvcRootOpt) -> Result<XvcRootOpt> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
note: the lint level is defined here
   --> lib/src/lib.rs:1:9
    |
1   | #![warn(missing_docs)]
    |         ^^^^^^^^^^^^

   Compiling xvc-workflow-tests v0.6.9-alpha.1 (/Users/iex/github.com/iesahin/xvc/workflow_tests)
warning: `xvc-storage` (lib) generated 37 warnings (run `cargo fix --lib -p xvc-storage` to apply 1 suggestion)
warning: `xvc-file` (lib) generated 2 warnings (run `cargo fix --lib -p xvc-file` to apply 1 suggestion)
warning: `xvc` (lib) generated 1 warning
    Finished `release` profile [optimized] target(s) in 41.49s
```

## Compiling Xvc without Reflink support

[reflink] crate may cause compilation errors on platforms where it's not supported.

Xvc adds a `reflink` feature flag that's turned on by default. When reflink
causes errors, you can turn off default features and select only those you'll
use.

```bash
cargo build --no-default-features --features "reflink" --release
[..]
    Finished `release` profile [optimized + debuginfo] target(s) in 56.40s
````

Note that when you supply `--no-default-features`, all other default features
like `s3` etc are also turned off. You'll have to specify which [features] you
want in the features list. Otherwise Xvc cannot connect to your storages.

```
cargo build --no-default-features --features "reflink,s3" --release
[..]
    Finished `release` profile [optimized + debuginfo] target(s) in 56.40s
```

[features]: https://docs.rs/crate/xvc/latest/features
