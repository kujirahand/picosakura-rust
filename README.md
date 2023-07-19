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

