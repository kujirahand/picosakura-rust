use rustysynth::{SynthesizerSettings, Synthesizer, SoundFont, MidiFile, MidiFileSequencer};
use tinyaudio::prelude::*;
use sakuramml;
use std::env;

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
    let mut wav_mode = false;
    // check args
    let mut i = 1;
    while i < args.len() {
        let arg = args[i].clone();
        if arg == "-h" || arg == "--help" {
            println!("usage: picosakura [options] [mmlfile]");
            println!("options:");
            println!("  -h, --help      show this help");
            println!("  -v, --version   show version");
            println!("  -d, --debug     show debug log");
            println!("  -s, --soundfont [soundfont]   specify soundfont file");
            println!("  -m, --midi      [midifile]    specify midi file");
            println!("  -w, --wav       [wavfile]     specify wav file");
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
        if arg == "-w" || arg == "--wav" {
            wav_mode = true;
            i += 1;
            if i < args.len() {
                wav = args[i].clone();
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
        println!("picosakura v{}", sakuramml::get_version());
        return;
    }
    // check input
    if input == "" {
        println!("Usage: picosakura [options] [mmlfile]");
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
    if wav_mode {
        save_to_wav(&input, &midi, &wav, &soundfont, debug_level);
        return;
    }
    // play
    play_audio(&input, &midi, &soundfont, debug_level);
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

fn play_audio(mmlfile: &str, midifile: &str, soundfont: &str, debug_level: u32) {
    // MMLをMIDIに変換
    let com_result = compile_to_midi(mmlfile, midifile, debug_level);
    if !com_result { return; }

    // オーディオの出力設定 --- (*2)
    let params = OutputDeviceParameters {
        channels_count: 2, // ステレオ
        sample_rate: SAMPLE_RATE, // サンプリング周波数
        channel_sample_count: SAMPLE_RATE / 2, // バッファサイズ
    };
    // 書き込み先のバッファを確保 --- (*3)
    let mut left_buf:Vec<f32> =  vec![0.0f32; params.channel_sample_count];
    let mut right_buf:Vec<f32> = vec![0.0f32; params.channel_sample_count];
    // 用意したサウンドフォントを読み込む --- (*4)
    let mut sf2 = std::fs::File::open(soundfont).unwrap_or_else(|_err| {
        println!("[WARN] soundfont file not found: {}", soundfont);
        println!("[WARN] try to use default font");
        let tmp = match std::fs::File::open(DEFUALT_SOUNDFONT) {
            Ok(f) => f,
            Err(_) => {
                println!("[ERROR] default soundfont file not found: {}", DEFUALT_SOUNDFONT);
                return std::fs::File::open(DEFUALT_SOUNDFONT).unwrap();
            }
        };
        return tmp;
    });
    let sound_font = std::sync::Arc::new(SoundFont::new(&mut sf2).unwrap());
    // シンセサイザーの作成 --- (*5)
    let settings = SynthesizerSettings::new(SAMPLE_RATE as i32);
    let synthesizer = Synthesizer::new(&sound_font, &settings).unwrap();
    // MIDIシーケンサーを作成してMIDIファイルを読み込む --- (*6)
    let mut sequencer = MidiFileSequencer::new(synthesizer);
    let mut mid = std::fs::File::open(midifile).unwrap();
    let midi_file = std::sync::Arc::new(MidiFile::new(&mut mid).unwrap());
    // シーケンサーの開始 --- (*7)
    sequencer.play(&midi_file, true); // 繰り返し再生を有効にする
    // オーディオの出力を開始 --- (*8)
    let _device = run_output_device(params, {
        move |data| {
            // 再生位置を標準出力に表示
            println!("{:03.1}/{} [Enter]で終了", sequencer.get_position(), 
                midi_file.get_length() as u32);
            // シーケンサーによる波形生成 --- (*9)
            let mut clock = 0;
            sequencer.render(&mut left_buf[..], &mut right_buf[..]);
            // 出力デバイスに書き込む --- (*10)
            for samples in data.chunks_mut(params.channels_count as usize) {
                // チャンネルごとに波形を書き込む --- (*11)
                for (ch, sample) in samples.iter_mut().enumerate() {
                    let v = if ch == 0 { left_buf[clock] } else { right_buf[clock] };
                    *sample = v;
                }
                clock = (clock + 1) % params.channel_sample_count;
            }
        }
    })
    .unwrap();
    // [Enter]で終了 --- (*12)
    std::io::stdin().read_line(&mut String::new()).unwrap();
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
