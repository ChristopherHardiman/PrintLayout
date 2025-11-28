// config.rs - Configuration and preferences management
// Phase 5: Persistence & State Management

use crate::layout::{Layout, PaperSize, PaperType, PrintQuality, ColorMode, Orientation};
use chrono::{DateTime, Utc};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Settings from the last successful print job
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LastPrintSettings {
    pub printer_name: Option<String>,
    pub paper_size: Option<PaperSize>,
    pub paper_type: Option<PaperType>,
    pub print_quality: Option<PrintQuality>,
    pub color_mode: Option<ColorMode>,
    pub orientation: Option<Orientation>,
    pub borderless: Option<bool>,
    pub copies: Option<u32>,
    pub margins: Option<(f32, f32, f32, f32)>, // top, bottom, left, right
    pub last_success_time: Option<DateTime<Utc>>,
}

/// User preferences that persist across sessions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    pub last_printer: Option<String>,
    pub default_paper_size: PaperSize,
    pub default_paper_type: PaperType,
    pub default_margins: (f32, f32, f32, f32), // top, bottom, left, right
    pub last_open_directory: Option<PathBuf>,
    pub zoom_level: f32,
    pub window_size: (u32, u32),
    pub window_position: Option<(i32, i32)>,
    pub recent_files: Vec<PathBuf>,
    pub auto_save_enabled: bool,
    pub auto_save_interval_seconds: u32,
    pub show_dpi_warnings: bool,
    pub snap_to_grid: bool,
    pub grid_size_mm: f32,
    /// Settings from the last successful print
    #[serde(default)]
    pub last_print_settings: LastPrintSettings,
}

impl Default for UserPreferences {
    fn default() -> Self {
        Self {
            last_printer: None,
            default_paper_size: PaperSize::A4,
            default_paper_type: PaperType::Plain,
            default_margins: (25.4, 25.4, 25.4, 25.4), // 1 inch all sides
            last_open_directory: None,
            zoom_level: 1.0,
            window_size: (1200, 800),
            window_position: None,
            recent_files: Vec::new(),
            auto_save_enabled: true,
            auto_save_interval_seconds: 300, // 5 minutes
            show_dpi_warnings: true,
            snap_to_grid: false,
            grid_size_mm: 10.0,
            last_print_settings: LastPrintSettings::default(),
        }
    }
}

/// A complete project layout for saving/loading
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectLayout {
    pub version: String,
    pub layout: Layout,
    pub created_at: DateTime<Utc>,
    pub last_modified: DateTime<Utc>,
    pub name: String,
    pub description: String,
}

impl ProjectLayout {
    pub fn new(layout: Layout, name: String) -> Self {
        let now = Utc::now();
        Self {
            version: env!("CARGO_PKG_VERSION").to_string(),
            layout,
            created_at: now,
            last_modified: now,
            name,
            description: String::new(),
        }
    }

    pub fn update_modified(&mut self) {
        self.last_modified = Utc::now();
    }
}

/// Configuration file management
#[derive(Clone)]
pub struct ConfigManager {
    config_dir: PathBuf,
    cache_dir: PathBuf,
}

impl ConfigManager {
    pub fn new() -> Result<Self, std::io::Error> {
        let proj_dirs = ProjectDirs::from("", "", "print_layout")
            .ok_or_else(|| std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Could not determine config directory"
            ))?;

        let config_dir = proj_dirs.config_dir().to_path_buf();
        let cache_dir = proj_dirs.cache_dir().to_path_buf();

        // Ensure directories exist
        fs::create_dir_all(&config_dir)?;
        fs::create_dir_all(&cache_dir)?;
        fs::create_dir_all(config_dir.join("backups"))?;

        Ok(Self {
            config_dir,
            cache_dir,
        })
    }

    /// Load user preferences from config file
    pub fn load_config(&self) -> UserPreferences {
        let config_path = self.config_dir.join("config.json");
        
        if !config_path.exists() {
            log::info!("Config file not found, using defaults");
            return UserPreferences::default();
        }

        match fs::read_to_string(&config_path) {
            Ok(contents) => match serde_json::from_str(&contents) {
                Ok(config) => {
                    log::info!("Loaded config from {:?}", config_path);
                    config
                }
                Err(e) => {
                    log::warn!("Failed to parse config: {}, using defaults", e);
                    UserPreferences::default()
                }
            },
            Err(e) => {
                log::warn!("Failed to read config: {}, using defaults", e);
                UserPreferences::default()
            }
        }
    }

    /// Save user preferences to config file
    pub fn save_config(&self, prefs: &UserPreferences) -> Result<(), std::io::Error> {
        let config_path = self.config_dir.join("config.json");
        let json = serde_json::to_string_pretty(prefs)?;
        
        // Atomic write: write to temp file, then rename
        let temp_path = config_path.with_extension("tmp");
        fs::write(&temp_path, json)?;
        fs::rename(temp_path, &config_path)?;
        
        log::info!("Saved config to {:?}", config_path);
        Ok(())
    }

    /// Save a project layout to file
    pub fn save_layout(&self, project: &ProjectLayout, path: &PathBuf) -> Result<(), std::io::Error> {
        let json = serde_json::to_string_pretty(project)?;
        
        // Create backup if file exists
        if path.exists() {
            self.create_backup(path)?;
        }
        
        // Atomic write
        let temp_path = path.with_extension("tmp");
        fs::write(&temp_path, json)?;
        fs::rename(temp_path, path)?;
        
        log::info!("Saved layout to {:?}", path);
        Ok(())
    }

    /// Load a project layout from file
    pub fn load_layout(&self, path: &PathBuf) -> Result<ProjectLayout, std::io::Error> {
        let contents = fs::read_to_string(path)?;
        let project: ProjectLayout = serde_json::from_str(&contents)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        
        log::info!("Loaded layout from {:?}", path);
        Ok(project)
    }

    /// Create a backup of a layout file
    fn create_backup(&self, path: &PathBuf) -> Result<(), std::io::Error> {
        let backup_dir = self.config_dir.join("backups");
        let filename = path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("layout");
        
        let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
        let backup_name = format!("{}_backup_{}.pxl", filename, timestamp);
        let backup_path = backup_dir.join(backup_name);
        
        fs::copy(path, &backup_path)?;
        log::info!("Created backup at {:?}", backup_path);
        
        // Keep only last 5 backups
        self.cleanup_old_backups(&backup_dir, filename)?;
        
        Ok(())
    }

    /// Remove old backups, keeping only the 5 most recent
    fn cleanup_old_backups(&self, backup_dir: &PathBuf, filename: &str) -> Result<(), std::io::Error> {
        let mut backups: Vec<_> = fs::read_dir(backup_dir)?
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry.file_name()
                    .to_str()
                    .map(|name| name.starts_with(filename) && name.ends_with(".pxl"))
                    .unwrap_or(false)
            })
            .collect();
        
        // Sort by modification time, newest first
        backups.sort_by_key(|entry| {
            entry.metadata()
                .and_then(|m| m.modified())
                .ok()
        });
        backups.reverse();
        
        // Remove old backups beyond the 5 most recent
        for backup in backups.iter().skip(5) {
            if let Err(e) = fs::remove_file(backup.path()) {
                log::warn!("Failed to remove old backup: {}", e);
            }
        }
        
        Ok(())
    }

    /// Save auto-save file
    pub fn auto_save(&self, layout: &Layout) -> Result<(), std::io::Error> {
        let auto_save_path = self.cache_dir.join("auto_save.pxl");
        let project = ProjectLayout::new(layout.clone(), "Auto-save".to_string());
        let json = serde_json::to_string_pretty(&project)?;
        fs::write(&auto_save_path, json)?;
        log::debug!("Auto-saved layout");
        Ok(())
    }

    /// Check if auto-save file exists
    pub fn has_auto_save(&self) -> bool {
        self.cache_dir.join("auto_save.pxl").exists()
    }

    /// Load auto-save file
    pub fn load_auto_save(&self) -> Result<ProjectLayout, std::io::Error> {
        let auto_save_path = self.cache_dir.join("auto_save.pxl");
        self.load_layout(&auto_save_path)
    }

    /// Delete auto-save file
    pub fn delete_auto_save(&self) -> Result<(), std::io::Error> {
        let auto_save_path = self.cache_dir.join("auto_save.pxl");
        if auto_save_path.exists() {
            fs::remove_file(&auto_save_path)?;
            log::info!("Deleted auto-save file");
        }
        Ok(())
    }

    /// Add a file to recent files list
    pub fn add_recent_file(&self, prefs: &mut UserPreferences, path: PathBuf) {
        // Remove if already exists
        prefs.recent_files.retain(|p| p != &path);
        
        // Add to front
        prefs.recent_files.insert(0, path);
        
        // Keep only 10 most recent
        prefs.recent_files.truncate(10);
    }
}

impl Default for ConfigManager {
    fn default() -> Self {
        Self::new().expect("Failed to create config manager")
    }
}
