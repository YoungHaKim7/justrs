<h1 align="center">justrs</h1>
<br />

<div align="center">
  <!-- Crates version -->
  <a href="https://crates.io/crates/justrs">
    <img src="https://img.shields.io/crates/v/justrs.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/justrs">
    <img src="https://img.shields.io/crates/d/justrs.svg?style=flat-square"
      alt="Download" />
  </a>
  <!-- docs.rs docs -->
  <a href="https://docs.rs/justrs">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
</div>
<br/>

# justrs
Justfile to assist a Rust project

# Install

```bash
$ https://github.com/YoungHaKim7/justrs

$ cd justrs
 
$ cargo install --path .
```

- `cargo install`

```bash
cargo install justrs
```

# just list

```
$ just -l
Available recipes:
    asm METHOD # cargo asm
    c          # cargo check(Test Before Deployment)
    clean      # clean file
    eos        # optimized assembly
    es         # emit asm file
    ex         # macro show(cargo expand)
    fi         # final review
    gi         # .gitignore setting
    hir        # emit hir file
    llvm       # emit llvm-ir file
    mir        # emit mir file
    n          # nightly setting(faster compilation)
    r          # cargo run
    rr         # (optimization)cargo run --release
    san SAN    # (nightly)clang sanitize(ASan=address / LSan=leak / TSan=thread / MSan=memory / UBSan=undefined)
    t          # cargo test
    tex        # cargo expand(test --lib)
    tn         # nightly(cargo nextest run)
    tnp        # nightly(cargo nextest run --nocapture)
    tp         # cargo test -- --nocapture
    w          # cargo watch(check & test & run)
    ws         # cargo watch(simple only run)
    xx         # hex view("rg -i <search>" | "grep -rni <search>")
```
