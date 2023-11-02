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
[OUT] [xvc-docs-update] Xvc docs updated!
 
[DONE] xvc-docs-update (echo 'Xvc docs updated!')

```

The step won't run again until a new version of the page is published.

```console
$ xvc pipeline run

```

Note that, Xvc doesn't download the page every time. It checks the `Last-Modified` and `Etag` headers and only downloads the page if it has changed.

If there are more complex requirements than just the URL changing, you can use a generic dependency to get the output of a command and use that as a dependency.

