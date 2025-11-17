//! Justfile to assist a Rust project
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
//! ```
//! // just -l
//!
//! // Available recipes:
//! //     asm METHOD # cargo asm
//! //     c          # cargo check(Test Before Deployment)
//! //     clean      # clean file
//! //     eos        # optimized assembly
//! //     es         # emit asm file
//! //     ex         # macro show(cargo expand)
//! //     fi         # final review
//! //     gi         # .gitignore setting
//! //     hir        # emit hir file
//! //     llvm       # emit llvm-ir file
//! //     mir        # emit mir file
//! //     n          # nightly setting(faster compilation)
//! //     r          # cargo run
//! //     rr         # (optimization)cargo run --release
//! //     san SAN    # (nightly)clang sanitize(ASan=address / LSan=leak / TSan=thread / MSan=memory / UBSan=undefined)
//! //     t          # cargo test
//! //     tex        # cargo expand(test --lib)
//! //     tn         # nightly(cargo nextest run)
//! //     tnp        # nightly(cargo nextest run --nocapture)
//! //     tp         # cargo test -- --nocapture
//! //     w          # cargo watch(check & test & run)
//! //     ws         # cargo watch(simple only run)
//! //     xx         # hex view("rg -i <search>" | "grep -rni <search>")
//! ```
//!
//! # Just a command runner
//! ```bash
//! $ just r
//! ```
pub mod execute_shell_command;
