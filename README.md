# picosakura-rust

MML Compiler Sakura Commandline player with SoundFont

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
picosakura test.mml --wav test.wav
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

