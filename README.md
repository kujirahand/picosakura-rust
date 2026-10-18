# picosakura-rust

Picosakura is an MML player for Sakuramml.
MML stands for Music Macro Language.
Picosakura use SoundFont to play MML files.
This is command line version.

- [WEB Site](https://sakuramml.com/picosakura/index.php)
- MML Compiler
  - [MML compiler sakuramml-rust](https://github.com/kujirahand/sakuramml-rust)
- Browser version
  - [Picosakura](https://github.com/kujirahand/picosakura)

## Install

This is command line tool.
Please download binary from [release(zip file)](https://github.com/kujirahand/picosakura-rust/releases).

- Windows / macOS

## Functions

- MML Player (SoundFont)
- MML Compiler
  - MML to MIDI
  - MML to WAV

## Usages

Play MML file

```
picosakura test.mml
```

Render to WAV file

```
picosakura test.mml --wav
```

Set SoundFont file

```
picosakura test.mml --soundfont test.sf2
```

## Commandline Options

```
Usage: picosakura [OPTIONS] <INPUT>

Arguments:
  <INPUT>  input mml file

Options:
  -s, --soundfont <SOUNDFONT>
  -m, --midi <MIDI>            output midi file
  -w, --wav <WAV>              output wav file
  -d, --debug <DEBUG>          debug level 0:none 1:info
  -h, --help                   Print help
```


## Choosing the MML compiler

Picosakura can link one of two MML compilers. The choice is made at build time with a cargo feature (see [issue #3](https://github.com/kujirahand/picosakura-rust/issues/3)).

| Feature | Package | Notes |
|---|---|---|
| `sakuramml` (default) | [sakuramml](https://crates.io/crates/sakuramml) | The current compiler (sakuramml-rust) |
| `sakuramml-v2` | [sakuramml-v2](https://crates.io/crates/sakuramml-v2) | A faithful port of the Pascal version |

```sh
# sakuramml (default)
cargo build --release
wasm-pack build --target web --release   # or: make wasm

# sakuramml-v2
cargo build --release --no-default-features --features sakuramml-v2
wasm-pack build --target web --release --no-default-features --features sakuramml-v2   # or: make wasm-v2
```

### How it works

- `Cargo.toml` declares both compilers as optional dependencies. `default = ["sakuramml"]`, so a plain build links only `sakuramml`. To use v2, disable the default features with `--no-default-features`.
- `src/compiler.rs` is the only file that touches the compiler packages. It provides `get_version()` and `compile(mml, debug_level)` for each feature, selected with `#[cfg(feature = "...")]`. `src/lib.rs` (WASM) and `src/main.rs` (CLI) call only this module.
- Only one compiler can be linked. Enabling both, or neither, is a compile error.
- Linking only one keeps the WASM small. Linking both makes it about 400KB larger (raw size: 533KB with `sakuramml`, 602KB with `sakuramml-v2`, 970KB with both).

### Differences with `sakuramml-v2`

- The debug level (`-d`) is ignored.
- `--version` prints a fixed string, because `sakuramml-v2` has no version function.
- Warnings are shown in the log as `[警告] ...`.
