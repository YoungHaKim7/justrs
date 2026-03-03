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
$ cargo install justrs

# or (nightly ver)
$ cargo install --git https://github.com/YoungHaKim7/justrs.git
```

# just list

```bash
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
    hir                   # emit hir file(nightly)
    llvm                  # emit llvm-ir file
    mir                   # emit mir file(nightly)
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
    w                     # cargo watch(check & test & run)(cargo install cargo-watch)
    ws                    # cargo watch(simple only run)
    xv STR                # hex view(cat "SEARCH" | rg -i --line-number --color=always "SEARCH"("rg -i <search>"))
    xx                    # hex view("rg -i <search>" | "grep -rni <search>")
```

# justfile공부하기 좋다

- https://github.com/lucasgelfond/zerobrew/blob/main/Justfile

# cargo-audit(보안 업데이트)
- https://rustsec.org

```bash
$ cargo install cargo-audit --locked
```

# **["2>&1"은 무엇을 의미하나?](<https://news.hada.io/topic?id=27061&utm_source=discord&utm_medium=bot&utm_campaign=1480>)**
- **표준 오류(stderr)** 와 **표준 출력(stdout)** 을 하나의 스트림으로 합치기 위해 사용하는 **리디렉션 구문**  
- 숫자 **1은 stdout**, **2는 stderr**를 의미하며, `&`는 **파일 디스크립터를 참조**한다는 표시로 사용됨  
- `2>&1`은 “stderr를 현재 stdout이 향하는 곳으로 보낸다”는 뜻이며, **출력 순서에…

# AI fix

- All tests pass. The fix was to use `.current_dir(temp_dir.path())` on the Command instead of changing the process's current directory with
  - `env::set_current_dir()`. This ensures cargo run executes in the temp directory and creates the justfile in the correct location.
