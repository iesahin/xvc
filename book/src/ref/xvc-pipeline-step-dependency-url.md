### URL Dependencies

This command works only in Xvc repositories.

```console
$ git init
...
$ xvc init
```

You can use a web URL as a dependency to a step.
When the URL is fetched, the output hash is saved to compare and the step is invalidated when the output of the URL is changed.

You can use this with any URL.

```console
$ xvc pipeline step new --step-name xvc-docs-update --command "echo 'Xvc docs updated!'"

$ xvc pipeline step dependency --step-name xvc-docs-update --url https://docs.xvc.dev/

```

The step is invalidated when the page is updated.

```console
$ xvc pipeline run
thread '<unnamed>' panicked at 'UrlRequestError { source: reqwest::Error { kind: Request, url: Url { scheme: "https", cannot_be_a_base: false, username: "", password: None, host: Some(Domain("docs.xvc.dev")), port: None, path: "/", query: None, fragment: None }, source: hyper::Error(Connect, ConnectError("dns error", Custom { kind: Uncategorized, error: "failed to lookup address information: nodename nor servname provided, or not known" })) } }', thread '<unnamed>' panicked at '[PANIC] UrlRequestError { source: reqwest::Error { kind: Request, url: Url { scheme: "https", cannot_be_a_base: false, username: "", password: None, host: Some(Domain("docs.xvc.dev")), port: None, path: "/", query: None, fragment: None }, source: hyper::Error(Connect, ConnectError("dns error", Custom { kind: Uncategorized, error: "failed to lookup address information: nodename nor servname provided, or not known" })) } }, [pipeline/src/pipeline/mod.rs::1122]', pipeline/src/pipeline/mod.rs:lib/src/cli/mod.rs:263:521122
:note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
28

```

The step won't run again until a new version of the page is published.

```console
$ xvc pipeline run
thread '<unnamed>' panicked at 'UrlRequestError { source: reqwest::Error { kind: Request, url: Url { scheme: "https", cannot_be_a_base: false, username: "", password: None, host: Some(Domain("docs.xvc.dev")), port: None, path: "/", query: None, fragment: None }, source: hyper::Error(Connect, ConnectError("dns error", Custom { kind: Uncategorized, error: "failed to lookup address information: nodename nor servname provided, or not known" })) } }', pipeline/src/pipeline/mod.rs:1122:28
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread '<unnamed>' panicked at '[PANIC] UrlRequestError { source: reqwest::Error { kind: Request, url: Url { scheme: "https", cannot_be_a_base: false, username: "", password: None, host: Some(Domain("docs.xvc.dev")), port: None, path: "/", query: None, fragment: None }, source: hyper::Error(Connect, ConnectError("dns error", Custom { kind: Uncategorized, error: "failed to lookup address information: nodename nor servname provided, or not known" })) } }, [pipeline/src/pipeline/mod.rs::1122]', lib/src/cli/mod.rs:263:52

```

Note that, Xvc doesn't download the page every time. It checks the `Last-Modified` and `Etag` headers and only downloads the page if it has changed.

If there are more complex requirements than just the URL changing, you can use a generic dependency to get the output of a command and use that as a dependency.

