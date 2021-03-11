# Bio Ballyhoo

***ballyhoo** (noun):  bombastic or pretentious nonsense*

> "Having found themselves caught up in the **ballyhoo** of the first two days of proceedings, not one of the programmers present expressed concern about the decision to *Rewrite It In Rust*."

## Build & Test

Assuming you have [Rust installed](https://www.rust-lang.org/tools/install), enter the project root, then run `cargo build --release`.  It's Rust, so prepare to wait.

Once that finishes, all of the binaries will be under `./target/release`.  Feel free to move or symlink them somewhere on your path.

To run tests (such as they are), enter the project root and run `make`.  The first time may take a while as currently tests are run against the `debug` profile rather than `release`.

## License

Dual-licensed to be compatible with the Rust project.

Licensed under the Apache License, Version 2.0 or the MIT license, at your option. This program may not be copied, modified, or distributed except according to those terms.
