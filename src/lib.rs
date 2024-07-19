use sakuramml;
use wav_io;
use rustysynth::{SynthesizerSettings, Synthesizer, SoundFont, MidiFile, MidiFileSequencer};

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

const SAMPLE_RATE: usize = 44_100;


/// Picosakura Result
#[wasm_bindgen]
// #[derive(Debug)]
pub struct PicoResult {
    /// Result of compilation
    pub result: bool,
    /// MIDI binary data
    bin: Vec<u8>,
    /// MIDI debug log
    log: String,
}
// #[wasm_bindgen]
impl PicoResult {
    /// Get MIDI binary data
    pub fn get_bin(&self) -> Vec<u8> {
        self.bin.clone()
    }
    /// Get MIDI debug log
    pub fn get_log(&self) -> String {
        self.log.clone()
    }
}

/// Compile MML to MIDI
#[wasm_bindgen]
pub fn compile_to_midi(mml_source: &str) -> PicoResult {
    let mut result = PicoResult {
        result: false,
        bin: Vec::new(),
        log: String::new(),
    };
    if mml_source == "" {
        result.log = "[ERROR] MML source is empty".to_string();
        return result;
    }
    let mut sakura = sakuramml::SakuraCompiler::new();
    let midi = sakura.compile(&mml_source);
    let log = sakura.get_log();
    result.log = log.clone();
    if log.contains("ERROR") {
        return result
    }
    result.result = true;
    result.bin = midi;
    return result
}

/// make wav from MML
#[wasm_bindgen]
pub fn make_wav_custom(mml_source: &str, soundfont: Vec<u8>, sample_rate: usize, sample_bit: usize, out_format: &str) -> PicoResult {
    let mut result = PicoResult {
        result: false,
        log: String::new(),
        bin: Vec::new(),
    };
    // MMLをMIDIに変換
    let midi_result = compile_to_midi(mml_source);
    if !midi_result.result { return midi_result; }
    result.log.push_str("compiled to midi\n");
    // Load soundfont
    let mut reader = std::io::Cursor::new(soundfont);
    let sound_font = match SoundFont::new(&mut reader) {
        Ok(s) => s,
        Err(e) => {
            result.log = format!("[ERROR] Failded to load SoundFont, {}", e);
            return result;
        }
    };
    let sound_font = std::sync::Arc::new(sound_font);
    result.log.push_str("loaded soundfont\n");
    // シンセサイザーの作成
    let settings = SynthesizerSettings::new(SAMPLE_RATE as i32);
    let synthesizer = Synthesizer::new(&sound_font, &settings).unwrap();
    // MIDIシーケンサーの作成
    let mut sequencer = MidiFileSequencer::new(synthesizer);
    // MIDIファイルの読み込み
    let mut midi_reader = std::io::Cursor::new(midi_result.get_bin());
    let midi_file = std::sync::Arc::new(MidiFile::new(&mut midi_reader).unwrap());
    let midi_time_len = midi_file.get_length();
    result.log.push_str("loaded midi file\n");
    // MIDIデータをオーディオ化するのに必要なサンプル数を計算
    let sample_count = (SAMPLE_RATE as f64 * midi_time_len) as usize;
    result.log.push_str(format!("samples.length={}\n", sample_count).as_str());
    // 書き込み先のバッファを確保
    let mut samples = vec![0.0f32; sample_count * 2];
    let mut left_buf =  vec![0.0f32; sample_count];
    let mut right_buf = vec![0.0f32; sample_count];
    // サウンドフォントを書き込み
    sequencer.play(&midi_file, false);
    sequencer.render(&mut left_buf[..], &mut right_buf[..]);
    for i in 0..left_buf.len() {
        samples[i*2+0] = left_buf[i];
        samples[i*2+1] = right_buf[i];
    }
    // WAVファイルを出力
    let mut wav_head = wav_io::new_stereo_header();
    // sample_rate
    wav_head.sample_rate = sample_rate as u32;
    if sample_bit == 8 {
        wav_head.bits_per_sample = 8;
        wav_head.set_int_format();
    } else if sample_bit == 16 {
        wav_head.bits_per_sample = 16;
        wav_head.set_int_format();
    } else if sample_bit == 24 {
        wav_head.bits_per_sample = 24;
        wav_head.set_int_format();
    } else {
        wav_head.bits_per_sample = 32;
    }
    // check mode
    if out_format == "ogg" {
        // Ogg-orpus mode
        let samples:Vec<f32> = wav_io::resample::linear(samples, 2, sample_rate as u32, 16000).try_into().unwrap();
        if samples.len() == 0 {
            result.log = format!("[ERROR] {}", "Failed to resample");
            return result;
        }
        let samples:Vec<i16> = convert_samples_f32_to_i16(&samples);
        if samples.len() == 0 {
            result.log = format!("[ERROR] {}", "Failed to convert samples f32 to i16");
            return result;
        }
        //
        result.log = format!("[ERROR] ogg-orpus: Not supported");
        result.result = false;
        return result;
    } else {
        match wav_io::write_to_bytes(&wav_head, &samples) {
            Ok(bytes) => {
                result.result = true;
                result.bin = bytes;
            },
            Err(e) => {
                result.log = format!("[ERROR] write wav: {}", e);
                result.result = false;
                return result;
            }
        }
    }
    result
}

/// make wav from MML
#[wasm_bindgen]
pub fn make_wav(mml_source: &str, soundfont: Vec<u8>) -> PicoResult {
    make_wav_custom(mml_source, soundfont, SAMPLE_RATE, 32, "wav")
}

/// convert f32 to i16 samples
pub fn convert_samples_f32_to_i16(samples: &Vec<f32>) -> Vec<i16> {
    let mut samples_i16 = vec![];
    for v in samples {
        samples_i16.push((*v * std::i16::MAX as f32) as i16);
    }
    samples_i16
}

