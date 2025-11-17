//! [Justfile](https://github.com/casey/just) to assist a Rust project
//!
//! # Install
//!
//! ```bash
//! $ https://github.com/YoungHaKim7/justrs
//!
//! $ cd justrs
//!
//! $ cargo install --path .
//! ```
//!
//! - `cargo install`
//!
//! ```bash
//! cargo install justrs
//! ```
//!
//! # just list
//!
//! ```bash
//! $ just -l
//!
//! Available recipes:
//!     asm METHOD # cargo asm
//!     c          # cargo check(Test Before Deployment)
//!     clean      # clean file
//!     eos        # optimized assembly
//!     es         # emit asm file
//!     ex         # macro show(cargo expand)
//!     fi         # final review
//!     gi         # .gitignore setting
//!     hir        # emit hir file
//!     llvm       # emit llvm-ir file
//!     mir        # emit mir file
//!     n          # nightly setting(faster compilation)
//!     r          # cargo run
//!     rr         # (optimization)cargo run --release
//!     san SAN    # (nightly)clang sanitize(ASan=address / LSan=leak / TSan=thread / MSan=memory / UBSan=undefined)
//!     t          # cargo test
//!     tex        # cargo expand(test --lib)
//!     tn         # nightly(cargo nextest run)
//!     tnp        # nightly(cargo nextest run --nocapture)
//!     tp         # cargo test -- --nocapture
//!     w          # cargo watch(check & test & run)
//!     ws         # cargo watch(simple only run)
//!     xx         # hex view("rg -i <search>" | "grep -rni <search>")
//! ```
//!
//! # Just a command runner
//! ```bash
//! $ just r
//! ```
//!
//! # justr prototype
//! - [justr prototype](https://github.com/YoungHaKim7/justrs/commit/7c0157ac24a406f7841e3b84e7dd321c560455fc)
//! - [2nd prototype](https://docs.rs/justrs/0.1.2/src/justrs/execute_shell_command.rs.html#1-58)
pub mod execute_shell_command;
