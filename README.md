# cast_checks

A procedural macro to check for invalid casts

Like [`-C overflow-checks`], `cast_checks` is enabled only for debug builds by default. To enable `cast_checks` for release builds, set the crate-level `release` feature.

## How it works

`cast_checks::enable` essentially rewrites each expression of the form:

```rust,ignore
expr as T
```

to an expression involving [`try_into`]:

```rust,ignore
<_ as TryInto::< T >>::try_into( expr ).expect("invalid cast")
```

So when an invalid cast occurs, a message like the following results:

```text
thread 'checked_truncation' panicked at 'invalid cast: TryFromIntError(())', cast_checks/tests/basic.rs:30:13
```

We say "essentially rewrites" because the actual generated code is slightly more complex. It uses [Nikolai Vazquez]'s [`impls`]' [trick] to determine whether an appropriate [`TryInto`] implementation exists.

## How to use

### With a stable compiler

You must use `cast_checks::enable` as an outer [attribute]. Example:

```rust
#[cast_checks::enable]
fn as_u16(x: u64) -> u16 {
    x as u16
}
```

### With a nightly compiler

**We recommend enabling Rust features [`custom_inner_attributes`] and [`proc_macro_hygiene`], and compiling with the [`procmacro2_semver_exempt`] config flag, e.g.:**

```sh
RUSTFLAGS='--cfg procmacro2_semver_exempt' cargo build
```

If you enable the [`custom_inner_attributes`] and [`proc_macro_hygiene`] features, you can use `cast_checks::enable` as an inner [attribute]. Example:

```rust,ignore
#![feature(custom_inner_attributes, proc_macro_hygiene)]

mod m {
    #![cast_checks::enable]

    /* items */
}
```

However, in our experience, this can cause panics to refer to the wrong locations.

To help counter the above, if you compile with the [`procmacro2_semver_exempt`] config flag, `cast_checks` will include more elaborate `expect` messages. Example:

```text
thread 'checked_truncation' panicked at 'invalid cast in `x as u8` at cast_checks/tests/basic.rs:30:13: TryFromIntError(())', cast_checks/tests/basic.rs:30:13
```

## `CAST_CHECKS_LOG`

If you are concerned that some casts are not being checked, try setting `CAST_CHECKS_LOG` when compiling. This will cause `cast_checks` to dump all rewritten locations to standard output. Example:

```text
cast_checks rewriting `x as u8` at cast_checks/tests/basic.rs:13:17
cast_checks rewriting `y as u128` at cast_checks/tests/basic.rs:22:13
...
```

Note that `CAST_CHECKS_LOG` requires `--cfg procmacro2_semver_exempt` to be passed to rustc.

[`-c overflow-checks`]: https://doc.rust-lang.org/rustc/codegen-options/index.html#overflow-checks
[attribute]: https://doc.rust-lang.org/reference/attributes.html
[`custom_inner_attributes`]: https://github.com/rust-lang/rust/issues/54726
[`procmacro2_semver_exempt`]: https://github.com/dtolnay/proc-macro2#unstable-features
[`proc_macro_hygiene`]: https://github.com/rust-lang/rust/issues/54727
[`rustflags='--cfg procmacro2_semver_exempt'`]: https://github.com/dtolnay/proc-macro2#unstable-features
[nikolai vazquez]: https://github.com/nvzqz
[`impls`]: https://github.com/nvzqz/impls
[trick]: https://github.com/nvzqz/impls#how-it-works
[`tryinto`]: https://doc.rust-lang.org/std/convert/trait.TryInto.html
[`try_into`]: https://doc.rust-lang.org/std/convert/trait.TryInto.html#tymethod.try_into
