# Problem

The specified features in `exe1`'s toml affect `exe2`.

There is a library in this workspace with one feature.

exe1 wants to use that feature

`lib = { path = "../lib", features = ["feature_a"] }`

exe2 does not.

`lib = { path = "../lib" }`

But the feature is being enabled for both exes.

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
