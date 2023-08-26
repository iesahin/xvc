# (Hyper-)Parameter Dependencies

You may be keeping pipeline-wide parameters in structured text files. You can specify such parameters found in JSON,
TOML and YAML files as dependencies.


This command works only in Xvc repositories.

```console
$ git init
...
$ xvc init
```

Suppose we have a YAML file that we specify various parameters for the whole connection.

```yaml
param: value
database:
  server: example.com
  port: 5432
  connection:
    timeout: 5000
numeric_param: 13
```

```
$ xvc pipeline step new --step-name read-database-config --command 'echo "Updated Database Configuration"'

$ xvc pipeline step new --step-name read-hyperparams --command 'echo "Update Hyperparameters"'

```

Let's create different steps for various pieces of this parameters file:

```console
$ xvc pipeline step dependency --step-name read-database-config --param 'myparams.yaml::database.port' --param 'myparams.yaml::database.server' --param 'myparams.yaml::database.connection'

$ xvc pipeline step dependency --step-name read-hyperparams --param 'myparams.yaml::param' --param 'myparams.yaml::numeric_param'

```

Run for the first time, as initially all dependencies are invalid:

```console
$ xvc pipeline run
[OUT] [read-hyperparams] Update Hyperparameters

[OUT] [read-database-config] Updated Database Configuration


```

For the second time, it won't read the configuration as nothing is changed:

```console
$ xvc pipeline run

```

When you update a value in this file, it will only invalidate the steps that depend on the value, not other dependencies
that rely on the same file.

Let's update the database port:

```console
$ perl -pi -e 's/5432/9876/g' myparams.yaml

$ xvc --debug pipeline run
[OUT] [read-database-config] Updated Database Configuration

[OUT] [read-hyperparams] Update Hyperparameters


```
