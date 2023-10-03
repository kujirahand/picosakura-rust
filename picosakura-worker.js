//
// picosakura-worker.js
//
const URL_SOUNDFONT = './fonts/TimGM6mb.sf2';

import init, { PicoResult, make_wav, make_wav_custom } from './pkg/picosakura.js';
init().then(() => {
    console.log('@loaded')
    self.postMessage({ type: 'loaded' });
}).catch(err => {
    console.error(err);
    self.postMessage({ type: 'error', data: err.toString() });
});
//
// worker event
//
self.addEventListener("message", (e) => {
    // メッセージを受け取ったときに動かすコード
    console.log("worker received a message", e);
    // makeWav
    if (e.data.type === 'makeWav') {
        const mml = e.data.mml;
        const out_format = e.data.out_format;
        makeWav(mml, out_format).then((obj) => {
            self.postMessage({ type: 'makeWav:ok', data: obj });
        }).catch(err => {
            console.error(err);
            self.postMessage({ type: 'error', data: err.toString() });
        });
    }
});

/// makeWav
async function makeWav(mml, out_format) {
    console.log('try to load soundfont')
    const soundfont = await loadBinary(URL_SOUNDFONT);
    console.log('soundfont.size=', soundfont.byteLength);
    // console.log('@mml=', mml);
    // (ex) make_wav_custom(mml_source, soundfont, SAMPLE_RATE, 32, "wav")
    let sample_rate = 44100;
    let bit_depth = 32;
    let format = 'wav'
    if (out_format === 'wav') {
        // default
        format = 'wav'
    }
    if (out_format === 'wav16') {
        bit_depth = 16;
        format = 'wav'
    }
    if (out_format === 'ogg') {
        format = 'ogg'
    }
    console.log('make_wav_custom(mml soundfont', sample_rate, bit_depth, format, ');');
    const result = make_wav_custom(mml, new Uint8Array(soundfont), sample_rate, bit_depth, format);
    if (!result.result) {
        const log = result.get_log()
        throw new Error(`[ERROR] soundfont error: ${log}`)
    }
    const log = result.get_log();
    const wav = result.get_bin();
    return {wav, log}
}

/// load
async function loadBinary(url) {
    const resp = await fetch(url);
    return await resp.arrayBuffer();
}
