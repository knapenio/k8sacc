# k8sacc

k8sacc is a simple wrapper around the `aws` and `doctl` command line tools.
It can be [configured](#configuration-file) using a simple YAML file.

## Usage

### CLI

```sh
k8sacc --help
```

```
Usage: k8sacc [OPTIONS] <COMMAND>

Commands:
  list      Print list of available accounts
  activate  Activate a given account
  help      Print this message or the help of the given subcommand(s)

Options:
  -c, --config <CONFIG>  Path to the configuration file [default: ~/.k8sacc]
  -h, --help             Print help information
```

### Configuration file

```yaml
-
  alias: do-example
  provider: do
  params:
    cluster: 91c7726c-1034-4318-0be1-a7e5a7bbe6dd
    # context: example
-
  alias: eks-example
  provider: eks
  params:
    name: development
    # region: eu-central-1
    # profile: example
```

## License

This library is provided under the MIT license. See [LICENSE](LICENSE).
