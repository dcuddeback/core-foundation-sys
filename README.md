# CoreFoundation Rust Bindings

The `CoreFoundation-sys` package provides declarations and linkage for the `CoreFoundation` C
library on OS X. Following the `*-sys` package conventions, the `CoreFoundation-sys` package does
not define higher-level abstractions over the native library.

## Status
The `CoreFoundation-sys` crate is a work in progress. It currently exports the most basic types
(array, dictionary, string, etc) and functions from CoreFoundation and has been tested only on the
x86_64 architecture on OS X.

## Usage
Add `CoreFoundation-sys` as a dependency in `Cargo.toml`:

```toml
[dependencies]
CoreFoundation-sys = "0.1.4"
```

Import the `CoreFoundation_sys` crate and use the functions as they're defined in the native
`CoreFoundation` library provided by Apple.

```rust
extern crate CoreFoundation_sys as cf;
```

## Contributing
You may find that you need some functionality that is missing from `CoreFoundation-sys`. If that's
the case, please open an issue on Github or send a pull request with the added functionality.

If you plan to submit a pull request, please note the structure of the code. There is one file for
each header file in the CoreFoundation framework. For example, `src/string.rs` contains the
definitions from `CoreFoundation/CFString.h`. The definitions in each file are more or less in the
same order that they appear in the matching header file. Each file is then re-exported in the crate
root, e.g., `pub use string::*`.

### Contributors

* [dcuddeback](https://github.com/dcuddeback)
* [oopsies49](https://github.com/oopsies49)
* [burtonageo](https://github.com/burtonageo)

## License
Copyright © 2015 David Cuddeback

Distributed under the [MIT License](LICENSE).
