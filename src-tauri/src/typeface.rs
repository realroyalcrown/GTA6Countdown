//! Locates the GTA display typeface among the fonts the user has installed.
//!
//! WKWebView deliberately hides locally installed fonts from web content, so
//! the widget cannot simply name Pricedown in a `font-family`. Instead the
//! backend reads the file the user installed themselves and hands it to the
//! webview at runtime. Nothing is bundled with the app, which keeps this within
//! Pricedown's free desktop licence.

use std::fs;
use std::path::{Path, PathBuf};

use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use serde::Serialize;

/// The name the stylesheet refers to, independent of the file's internal name.
pub const CSS_FAMILY: &str = "GTA Display";

const FILE_STEMS: [&str; 1] = ["pricedown"];
const EXTENSIONS: [&str; 3] = ["otf", "ttf", "woff2"];

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TypefaceAsset {
    pub family: String,
    /// The font file as a `data:` URL, ready for a `FontFace` constructor.
    pub source: String,
    /// Shown in the README and useful when debugging a missing font.
    pub origin: String,
}

pub fn find() -> Option<TypefaceAsset> {
    search_directories()
        .iter()
        .filter_map(|directory| first_match(directory))
        .find_map(|path| read_asset(&path))
}

fn search_directories() -> Vec<PathBuf> {
    let mut directories = Vec::new();

    if let Some(home) = std::env::var_os("HOME") {
        directories.push(PathBuf::from(&home).join("Library/Fonts"));
    }
    directories.push(PathBuf::from("/Library/Fonts"));

    directories
}

fn first_match(directory: &Path) -> Option<PathBuf> {
    let entries = fs::read_dir(directory).ok()?;

    entries
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .find(|path| is_candidate(path))
}

fn is_candidate(path: &Path) -> bool {
    let Some(stem) = path.file_stem().and_then(|value| value.to_str()) else {
        return false;
    };
    let Some(extension) = path.extension().and_then(|value| value.to_str()) else {
        return false;
    };

    let stem = stem.to_ascii_lowercase();
    let extension = extension.to_ascii_lowercase();

    EXTENSIONS.contains(&extension.as_str())
        && FILE_STEMS.iter().any(|needle| stem.starts_with(needle))
}

fn read_asset(path: &Path) -> Option<TypefaceAsset> {
    let bytes = fs::read(path).ok()?;
    let mime = mime_for(path);
    let encoded = STANDARD.encode(&bytes);

    Some(TypefaceAsset {
        family: CSS_FAMILY.to_string(),
        source: format!("data:{mime};base64,{encoded}"),
        origin: path.display().to_string(),
    })
}

fn mime_for(path: &Path) -> &'static str {
    match path
        .extension()
        .and_then(|value| value.to_str())
        .map(str::to_ascii_lowercase)
        .as_deref()
    {
        Some("ttf") => "font/ttf",
        Some("woff2") => "font/woff2",
        _ => "font/otf",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recognises_pricedown_files_case_insensitively() {
        assert!(is_candidate(Path::new("/Library/Fonts/Pricedown Bl.otf")));
        assert!(is_candidate(Path::new("/Library/Fonts/pricedown.ttf")));
    }

    #[test]
    fn ignores_unrelated_fonts_and_formats() {
        assert!(!is_candidate(Path::new("/Library/Fonts/Helvetica.ttc")));
        assert!(!is_candidate(Path::new("/Library/Fonts/Pricedown Bl.txt")));
    }
}
