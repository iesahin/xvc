# Start

- [Introduction](./01-introduction.md)
- [Installation](./02-installation.md)
  - [Configuration](./021-configuration.md)
- [Get Started](./start/index.md)
  - [Xvc for Data](./start/data.md)
  - [Xvc for Machine Learning](./start/ml.md)
  - [Xvc for Software Developers](./start/developer.md)
  - [Xvc for Everyone](./start/everyone.md)

# Guides

- [How to Compile Xvc](./how-to/compile.md)

# Command Reference

- [xvc init](./ref/xvc-init.md)
- [File Management (xvc file)](./ref/xvc-file.md)
  - [xvc file track](./ref/xvc-file-track.md)
  - [xvc file list](./ref/xvc-file-list.md)
  - [xvc file hash](./ref/xvc-file-hash.md)
  - [xvc file checkout](./ref/xvc-file-checkout.md)
  - [xvc file push](./ref/xvc-file-push.md)
  - [xvc file fetch](./ref/xvc-file-fetch.md)
  - [xvc file pull](./ref/xvc-file-pull.md)
- [Data-Model Pipelines (xvc pipeline)](./ref/xvc-pipeline.md)
  - [xvc pipeline new](./ref/xvc-pipeline-new.md)
  - [xvc pipeline list](./ref/xvc-pipeline-list.md)
  - [xvc pipeline step](./ref/xvc-pipeline-step.md)
    - [xvc pipeline step dependency](./ref/xvc-pipeline-step-dependency.md)
    - [xvc pipeline step new](./ref/xvc-pipeline-step-new.md)
    - [xvc pipeline step output](./ref/xvc-pipeline-step-output.md)
    - [xvc pipeline step show](./ref/xvc-pipeline-step-show.md)
    - [xvc pipeline step update](./ref/xvc-pipeline-step-update.md)
  - [xvc pipeline run](./ref/xvc-pipeline-run.md)
  - [xvc pipeline delete](./ref/xvc-pipeline-delete.md)
  - [xvc pipeline export](./ref/xvc-pipeline-export.md)
  - [xvc pipeline import](./ref/xvc-pipeline-import.md)
  - [xvc pipeline update](./ref/xvc-pipeline-update.md)
  - [xvc pipeline dag](./ref/xvc-pipeline-dag.md)
- [(Remote) Storages (`xvc storage`)](./ref/xvc-storage.md)
  - [xvc storage list](./ref/xvc-storage-list.md)
  - [xvc storage remove](./ref/xvc-storage-remove.md)
  - [xvc storage new](./ref/xvc-storage-new.md)
    - [xvc storage new local](./ref/xvc-storage-new-local.md)
    - [xvc storage new generic](./ref/xvc-storage-new-generic.md)
    - [xvc storage new s3](./ref/xvc-storage-new-s3.md)
    - [xvc storage new gcs](./ref/xvc-storage-new-gcs.md)
    - [xvc storage new minio](./ref/xvc-storage-new-minio.md)
    - [xvc storage new r2](./ref/xvc-storage-new-r2.md)
    - [xvc storage new wasabi](./ref/xvc-storage-new-wasabi.md)
    - [xvc storage new digital-ocean](./ref/xvc-storage-new-digital-ocean.md)
- [Utilities](./ref/utilities.md)
  - [xvc root](./ref/xvc-root.md)
  - [xvc check-ignore](./ref/xvc-check-ignore.md)

# API 

- [`xvc`](./api/xvc.md)
- [`xvc-config`](./api/xvc-config.md)
- [`xvc-core`](./api/xvc-core.md)
- [`xvc-ecs`](./api/xvc-ecs.md)
- [`xvc-file`](./api/xvc-file.md)
- [`xvc-logging`](./api/xvc-logging.md)
- [`xvc-pipeline`](./api/xvc-pipeline.md)
- [`xvc-storage`](./api/xvc-storage.md)
- [`xvc-walker`](./api/xvc-walker.md)


# Architecture

- [Overview](./arch/index.md)
- [Goals](./arch/goals.md)
- [Xvc ECS](./arch/ecs.md)
- [Comparisons](./arch/delta.md)
- [Storages](./arch/storages.md)
- [Concepts](./concepts/index.md)
  - [Digest](./concepts/digest.md)
  - [Associated Digest](./concepts/associated-digest.md)

# Meta

- [Conventions](./20-conventions.md)
