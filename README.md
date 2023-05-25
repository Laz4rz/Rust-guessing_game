- variables are immutable by default in Rust
- mutable variables are created with `let mut`
- references are also immutable, can be passed in mutable form by `& mut variable_name`

### Cargo dependencies
After adding dependency to Cargo.toml and building the package with it, it will be saved in Cargo.lock.
We can explicitly update the dependancy by using

```
$ cargo update
```

### Cargo documentation

By running

```
cargo doc --open
```

The documentation html will open
