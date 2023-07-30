use rustysynth::{SynthesizerSettings, Synthesizer, SoundFont, MidiFile, MidiFileSequencer};
use sakuramml;
use std::env;

const VERSION: &str = "0.1.0";
const SAMPLE_RATE: usize = 44_100;
const DEFUALT_SOUNDFONT: &str = "fonts/TimGM6mb.sf2";

fn main() {
    let args: Vec<String> = env::args().collect();
    // options
    let mut input = "".to_string();
    let mut soundfont = "".to_string();
    let mut midi = "".to_string();
    let mut wav = "".to_string();
    let mut debug = false;
    let mut version = false;
    // check args
    let mut i = 1;
    while i < args.len() {
        let arg = args[i].clone();
        if arg == "-h" || arg == "--help" {
            println!("usage: mml2wav [options] [mmlfile]");
            println!("options:");
            println!("  -h, --help      show this help");
            println!("  -v, --version   show version");
            println!("  -d, --debug     show debug log");
            println!("  -s, --soundfont [soundfont]   specify soundfont file");
            return;
        }
        if arg == "-v" || arg == "--version" {
            version = true;
            break;
        }
        if arg == "-d" || arg == "--debug" {
            debug = true;
            continue;
        }
        if arg == "-m" || arg == "--midi" {
            i += 1;
            if i < args.len() {
                midi = args[i].clone();
                i += 1;
            }
            continue;
        }
        if arg == "-s" || arg == "--soundfont" {
            i += 1;
            if i < args.len() {
                soundfont = args[i].clone();
                i += 1;
            }
            continue;
        }
        if input == "" {
            input = arg.clone();
        }
        i += 1;
    }
    // version
    if version {
        println!("- mml2wav v{}", VERSION);
        println!("  - compiler v{}", sakuramml::get_version());
        return;
    }
    // check input
    if input == "" {
        println!("Usage: mml2wav [options] [mmlfile]");
        println!("[INFO] To get more information, please specify `--help`");
        return;
    }
    if soundfont == "" {
        soundfont = DEFUALT_SOUNDFONT.to_string();
    }
    println!("[INFO] soundfont={}", soundfont);
    if midi == "" {
        midi = input.clone();
        midi.push_str(".mid");
        midi = midi.replacen(".mml.mid", ".mid", 1);
    }
    // debug
    let debug_level = if debug { 1 } else { 0 };
    // wav
    if wav == "" {
        wav = input.clone();
        wav.push_str(".wav");
        wav = wav.replacen(".mml.wav", ".wav", 1);
    }
    save_to_wav(&input, &midi, &wav, &soundfont, debug_level);
}

fn compile_to_midi(mmlfile: &str, midifile: &str, debug_level: u32) -> bool {
    // MMLをMIDIに変換
    let mml_source = std::fs::read_to_string(mmlfile).unwrap_or_else(|_err|{
        println!("[ERROR] input file could not read: {}", mmlfile);
        return "".to_string();
    });
    println!("[INFO] compile mml to midi");
    if mml_source == "" { return false; }
    let mut sakura = sakuramml::SakuraCompiler::new();
    sakura.set_debug_level(debug_level);
    let midi = sakura.compile(&mml_source);
    let log = sakura.get_log();
    if log.contains("ERROR") {
        println!("[ERROR] Failed to compile\n{}", log);
        return false;
    } else {
        if log != "" {
            println!("{}", log);
        }
    }
    // MIDIファイルを保存
    std::fs::write(midifile, midi).unwrap();
    true
}

fn save_to_wav(mmlfile: &str, midifile: &str, wavfile: &str, soundfont: &str, debug_level: u32) {
    // MMLをMIDIに変換
    let com_result = compile_to_midi(mmlfile, midifile, debug_level);
    if !com_result { return; }
    // 用意したサウンドフォントを読み込む
    println!("[INFO] check soundfont file");
    let mut sf2 = std::fs::File::open(soundfont).unwrap();
    let sound_font = std::sync::Arc::new(SoundFont::new(&mut sf2).unwrap());
    // シンセサイザーの作成
    let settings = SynthesizerSettings::new(SAMPLE_RATE as i32);
    let synthesizer = Synthesizer::new(&sound_font, &settings).unwrap();
    // MIDIシーケンサーの作成
    let mut sequencer = MidiFileSequencer::new(synthesizer);
    // MIDIファイルの読み込み
    let mut mid = std::fs::File::open(midifile).unwrap();
    let midi_file = std::sync::Arc::new(MidiFile::new(&mut mid).unwrap());
    let midi_time_len = midi_file.get_length();
    // MIDIデータをオーディオ化するのに必要なサンプル数を計算
    let sample_count = (SAMPLE_RATE as f64 * midi_time_len) as usize;
    // 書き込み先のバッファを確保
    let mut samples = vec![0.0f32; sample_count * 2];
    let mut left_buf =  vec![0.0f32; sample_count];
    let mut right_buf = vec![0.0f32; sample_count];
    // サウンドフォントを書き込み
    println!("[INFO] render midi file: {}", midifile);
    sequencer.play(&midi_file, false);
    sequencer.render(&mut left_buf[..], &mut right_buf[..]);
    for i in 0..left_buf.len() {
        samples[i*2+0] = left_buf[i];
        samples[i*2+1] = right_buf[i];
    }
    // WAVファイルへ保存
    println!("[INFO] write to wav file: {}", wavfile);
    let mut wav_head = wav_io::new_stereo_header();
    wav_head.sample_rate = SAMPLE_RATE as u32;
    let mut wav_out = std::fs::File::create(wavfile).unwrap();
    wav_io::write_to_file(&mut wav_out, &wav_head, &samples).unwrap();
}
