//! CxLine file access beneath the caller's resolved, trusted client home.
//! Reads never initialize files; only explicit saves create directories.

use super::config::CxLineConfig;
use super::themes::ThemePresets;
use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use tempfile::NamedTempFile;

pub(super) struct CxLineStore {
    root: PathBuf,
}

impl CxLineStore {
    pub(super) fn new(codex_home: &Path) -> Self {
        Self {
            root: codex_home.join("cxline"),
        }
    }

    pub(super) fn load_config(&self) -> io::Result<CxLineConfig> {
        match read_config(&self.root.join("config.toml")) {
            Ok(config) => Ok(config),
            Err(error) if error.kind() == io::ErrorKind::NotFound => self.load_theme("cometix"),
            Err(error) => Err(error),
        }
    }

    /// Stored themes take precedence over presets. Unusable themes fall back without
    /// repairing the file, matching the legacy loader while keeping reads side-effect free.
    pub(super) fn load_theme(&self, name: &str) -> io::Result<CxLineConfig> {
        let path = self.theme_path(name)?;
        match read_config(&path) {
            Ok(config) => return Ok(config),
            Err(error) if error.kind() == io::ErrorKind::NotFound => {}
            Err(error) => {
                tracing::warn!(theme = name, %error, "Failed to load CxLine theme; using preset")
            }
        }
        Ok(ThemePresets::get_builtin(name).unwrap_or_else(ThemePresets::get_default))
    }

    pub(super) fn save_config(&self, config: &CxLineConfig) -> io::Result<()> {
        write_config(&self.root.join("config.toml"), config, WriteMode::Replace)
    }

    /// Write the currently selected theme, replacing its file if it exists.
    pub(super) fn save_theme(&self, config: &CxLineConfig) -> io::Result<()> {
        write_config(&self.theme_path(&config.theme)?, config, WriteMode::Replace)
    }

    /// Save a copy under a new name without changing the draft or replacing any file.
    pub(super) fn save_new_theme(&self, name: &str, config: &CxLineConfig) -> io::Result<()> {
        let path = self.theme_path(name)?;
        let renamed = CxLineConfig {
            theme: name.to_string(),
            ..config.clone()
        };
        write_config(&path, &renamed, WriteMode::CreateNew)
    }

    fn theme_path(&self, name: &str) -> io::Result<PathBuf> {
        // Keep names portable between Linux and Windows, including Chinese names.
        // Reserve room for ".toml" within the common 255-byte filename limit.
        let stem = name
            .split('.')
            .next()
            .unwrap_or_default()
            .trim_end()
            .to_uppercase();
        let reserved = matches!(
            stem.as_str(),
            "CON" | "PRN" | "AUX" | "NUL" | "CONIN$" | "CONOUT$"
        ) || ["COM", "LPT"].iter().any(|prefix| {
            stem.strip_prefix(prefix).is_some_and(|suffix| {
                matches!(
                    suffix,
                    "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "¹" | "²" | "³"
                )
            })
        });
        if name.is_empty()
            || name.len() > 255 - ".toml".len()
            || name.ends_with(['.', ' '])
            || name
                .chars()
                .any(|c| c.is_control() || "/\\<>:\"|?*".contains(c))
            || reserved
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "theme name must be a single portable filename (up to 250 UTF-8 bytes)",
            ));
        }
        Ok(self.root.join("themes").join(format!("{name}.toml")))
    }
}

fn read_config(path: &Path) -> io::Result<CxLineConfig> {
    let content = fs::read_to_string(path)?;
    toml::from_str(&content).map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))
}

enum WriteMode {
    Replace,
    CreateNew,
}

fn write_config(path: &Path, config: &CxLineConfig, mode: WriteMode) -> io::Result<()> {
    let content = toml::to_string_pretty(config)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
    if matches!(mode, WriteMode::Replace) {
        match fs::symlink_metadata(path) {
            Ok(metadata) if metadata.file_type().is_symlink() => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "refusing to replace a CxLine configuration symlink",
                ));
            }
            Ok(_) => {}
            Err(error) if error.kind() == io::ErrorKind::NotFound => {}
            Err(error) => return Err(error),
        }
    }
    let parent = path.parent().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "configuration path has no parent",
        )
    })?;
    fs::create_dir_all(parent)?;
    let mut temporary = NamedTempFile::new_in(parent)?;
    temporary.write_all(content.as_bytes())?;
    temporary.as_file().sync_all()?;
    match mode {
        WriteMode::Replace => temporary.persist(path),
        WriteMode::CreateNew => temporary.persist_noclobber(path),
    }
    .map_err(|error| error.error)?;
    Ok(())
}
