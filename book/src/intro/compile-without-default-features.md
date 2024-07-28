# Compiling Xvc without default features

You may want to customize the [feature set][features] when you want a smaller binary size. Not everyone needs all storage options and turning off them may result in smaller binary sizes.

When you turn off all remote storage features, async runtime (`tokio`) is also excluded from binary.

````bash
cargo build --no-default-features --release
[..]
    Finished `release` profile [optimized] target(s) in 4.65s
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

```bash
cargo build --no-default-features --features "reflink,s3" --release
[..]
    Finished `release` profile [optimized + debuginfo] target(s) in 56.40s
```

[features]: https://docs.rs/crate/xvc/latest/features
