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

- [Justfile](https://github.com/casey/just) to assist a Rust project
  - [Just a command runner](https://just.systems/)

# Dependencies

- just install 

```bash
cargo install just && cargo install ripgrep
```

# Install

```bash
$ git clone https://github.com/YoungHaKim7/justrs

$ cd justrs
 
$ cargo install --path .
```

- `cargo install`

```bash
cargo install justrs
```

# just list

```bash
$ just -l

Available recipes:
    a SEARCH              # just search "just a <search_str>"(cargo install ripgrep)
    asm METHOD            # cargo asm
    c                     # cargo check(Test Before Deployment)
    clean                 # clean file
    doc                   # cargo doc (documentation preview)
    eos                   # optimized assembly
    es                    # emit asm file
    ex                    # macro show(cargo expand)
    fi                    # final review
    gi                    # .gitignore setting
    hir                   # emit hir file
    llvm                  # emit llvm-ir file
    mir                   # emit mir file
    n                     # nightly setting(faster compilation)
    r                     # cargo run
    rr                    # (optimization)cargo run --release
    rupdate               # rust-analyzer is available in rustup:
    rustupdate            # rust-lang(rustc) update stable
    t                     # cargo test
    tex                   # cargo expand(test --lib)
    tn                    # nightly(cargo nextest run)
    tnp                   # nightly(cargo nextest run --nocapture)
    toolremove TOOLCHAINS # "rustup show" & remove toolchains
    tp                    # cargo test -- --nocapture
    w                     # cargo watch(check & test & run)
    ws                    # cargo watch(simple only run)
    xv STR                # hex view(cat "SEARCH" | rg -i --line-number --color=always "SEARCH"("rg -i <search>"))
    xx                    # hex view("rg -i <search>" | "grep -rni <search>")
```
