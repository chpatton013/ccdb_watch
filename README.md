# ccdb_watch

*Compile Commands Database Watch Utility*

## Summary

`ccdb_watch` is a utility that rebuilds compile commands databases when source
files have changed. `ccdb_watch` can be configured to allow control over:

1. the set of watched files
1. the aggressiveness of database (re)generation
1. the compile commands generation command

## Requirements

`ccdb_watch` depends on `libfswatch`, which must be available on your system to
dynamically link at runtime.

`libfswatch` is available in most package managers, or a source release can be
downloaded from [GitHub](https://github.com/emcrisostomo/fswatch/releases).

## License

`ccdb_watch` is licensed under the terms of the MIT License, as described in
[LICENSE.md](LICENSE.md)

## Contributing

Contributions are welcome in the form of bug reports, feature requests, or pull
requests.

Contribution to `ccdb_watch` is organized under the terms of the [Contributor
Covenant](CONTRIBUTOR_COVENANT.md).
