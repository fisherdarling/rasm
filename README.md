# **W**eb**A**ssembly **B**inary **I**nterpreter

Wabi is a _highly experimental_ interpreter of WebAssembly bytecode. Current functionality includes parsing, instantiating, and executing self contained, single-module WebAssembly.

## Quick Start

Since the crate is not yet published, clone the repository and add it to your `Cargo.toml`:

```toml
[dependencies]
wabi = { version = "*", path = "path/to/wabi" }
```

Add some imports:

```rust
use wabi::{args, runtime::ModuleInstance};
```

Load a module from some bytes, create some arguments,
and execute a function:

```rust
use wabi::{args, runtime::ModuleInstance};

fn main() {
    let input = byte_slice;
    let args = args![5_i32, 5_i32];

    let mut instance = ModuleInstance::builder()
        .bytes(input)
        .build()
        .unwrap();

    let res = instance.invoke("add", &args).unwrap();

    println!("{:?}", res);
}
```
-----
## Running a .wast file:

> `cargo run --bin run_wast -- -f ./path/to/testsuite/labels.wast`

```
================ ./path/to/testsuite/labels.wast ================
[MODULE] Name: None
[0291] [PASSED] block
[0292] [PASSED] loop1
[0293] [PASSED] loop2
/* snip */
[0308] [PASSED] return
[0313] [PASSED] br
[0314] [PASSED] shadowing
[0315] [PASSED] redefinition
```

Pass in multiple files to run them all.

<br>
<br>
<br>
<br>

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

> **Highly experimental, do not use!**