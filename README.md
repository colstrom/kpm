# kpm - Kernel Parameter Manager

## What is this?

`kpm` (as the name suggests) is a program to manage kernel parameters. It reads
target values from the environment and sets them if they do not match.

## Why does this exist?

To run as a `DaemonSet` on a Kubernetes cluster, to enforce a set of target
kernel parameters.

## How do I get this majestic tool?

```sh
$ cargo install kpm
```

## How do I use it?

For each managed Kernel Parameter, set an environment variable to the target
value. The name of this variable should be the parameter name, prefixed with
`sysctl-`.

```sh
env sysctl-fs.inotify.max_queued_events=16384 \
    sysctl-fs.inotify.max_user_instances=128 \
    sysctl-fs.inotify.max_user_watches=8192 \
    kpm enforce
```

By default, `kpm` only treats values that exactly match the target as
"correct". This can be overridden by prefixing the value with one of the
following identifiers:

| Prefix | Rule    | Description                                            |
|--------|---------|--------------------------------------------------------|
| ==     | EqualTo | Value must be exactly equal to the target.
| =>     | AtLeast | Value must be greater than or equal to the target.
| =<     | AtMost  | Value must be less than or equal to the target.

These do not work with non-integer parameters. Since the vast majority of
kernel parameters are integers, this is unlikely to be a problem. Use with
string parameters will report false negatives, and set the value every time.

## Environment Variables

| Name           | Default Value | Examples                         | Description              |
|----------------|---------------|----------------------------------|--------------------------|
| KPM_LOG_FILTER | info          | error, warn, info, debug, trace. | Configures log filtering |
| KPM_LOG_STYLE  | auto          | auto, always, never.             | Configures log styling   |

## License

`kpm` is available under the [MIT License](https://mit-license.org/), see `LICENSE.txt` for the full text.
