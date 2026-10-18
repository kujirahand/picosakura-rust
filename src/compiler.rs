//! MML compiler backend, selected at build time by a cargo feature.
//!
//! - `sakuramml` (default): the sakuramml package
//! - `sakuramml-v2`: the sakuramml-v2 package

#[cfg(all(feature = "sakuramml", feature = "sakuramml-v2"))]
compile_error!("features `sakuramml` and `sakuramml-v2` are mutually exclusive; use `--no-default-features --features sakuramml-v2`");
#[cfg(not(any(feature = "sakuramml", feature = "sakuramml-v2")))]
compile_error!("enable one of the features `sakuramml` or `sakuramml-v2`");

/// Compile result: MIDI bytes (None on failure) and the log text
pub struct Compiled {
    pub midi: Option<Vec<u8>>,
    pub log: String,
}

/// Version of the linked compiler
#[cfg(feature = "sakuramml")]
pub fn get_version() -> String {
    sakuramml::get_version().to_string()
}
#[cfg(feature = "sakuramml-v2")]
pub fn get_version() -> String {
    "2 (sakuramml-v2)".to_string()
}

/// Compile MML source to a Standard MIDI File
#[cfg(feature = "sakuramml")]
pub fn compile(mml_source: &str, debug_level: u32) -> Compiled {
    let mut sakura = sakuramml::SakuraCompiler::new();
    sakura.set_debug_level(debug_level);
    let midi = sakura.compile(mml_source);
    let log = sakura.get_log();
    let ok = !log.contains("ERROR");
    Compiled { midi: if ok { Some(midi) } else { None }, log }
}
#[cfg(feature = "sakuramml-v2")]
pub fn compile(mml_source: &str, _debug_level: u32) -> Compiled {
    match sakuramml_v2::compile(mml_source) {
        Ok(out) => {
            let mut lines: Vec<String> = out.messages;
            lines.extend(out.warnings.iter().map(|w| w.to_string()));
            Compiled { midi: Some(out.smf), log: lines.join("\n") }
        }
        Err(e) => Compiled { midi: None, log: format!("[ERROR] {}", e) },
    }
}
