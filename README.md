# Problem

The specified features in `exe1`'s toml affect `exe2`.

There is a library in this workspace with one feature.

exe1 wants to use that feature

`lib = { path = "../lib", features = ["feature_a"] }`

exe2 does not.

`lib = { path = "../lib" }`

But the feature is being enabled for both exes.

# Solution

Specify package, not bin:

```shell
cargo run --package exe2
```

See https://github.com/rust-lang/cargo/issues/8157

## Alternative
Another option is to expose the feature at the bin level and enable during exe1 build:

Add:

```toml
[features]
feature_a = ["lib/feature_a"]
```

To exe1 toml, and build like so:

```shell
cargo run --package exe1 --features feature_a
```

# Repro steps

```shell
cargo run --bin exe1
```

Result:
```
This is exe1
From library: Feature A active
```

This is correct, as feature A is specified in exe1's toml.

```shell
cargo run --bin exe2
```

Result:
```
This is exe2
From library: Feature A active
```

This is *incorrect*, as feature A is not specified in exe2's toml.

Removing the feature from exe1 fixes exe2's behaviour as does removing exe1 from the workspace.

```toml
members = [
    #"exe1",
    "exe2",
    "lib",
]
```

```
This is exe2
From library: Feature A inactive
```
