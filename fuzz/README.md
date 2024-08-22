How to run the fuzzer:

```
rustup update nightly
```

```
cargo install cargo-fuzz
```

```
cargo +nightly fuzz build
```

To run roundtrips for modules.
```
cargo +nightly fuzz run orca
```

To run roundtrips for components.
```
cargo +nightly fuzz run component
```