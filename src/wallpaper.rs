/// Wallpaper rendering support

use std::path::Path;

/// Wallpaper configuration
pub struct Wallpaper {
    pub path: String,
    pub is_enabled: bool,
}

impl Wallpaper {
    pub fn new(path: impl AsRef<Path>) -> Self {
        let path_str = path.as_ref().to_string_lossy().to_string();
        let exists = Path::new(&path_str).exists();
        
        Self {
            path: path_str,
            is_enabled: exists,
        }
    }

    pub fn default() -> Self {
        Self::new("/home/aditya/Projects/mirage-wm/Mirage-Default.jpg")
    }

    pub fn is_valid(&self) -> bool {
        self.is_enabled && Path::new(&self.path).exists()
    }
}

impl Default for Wallpaper {
    fn default() -> Self {
        Self::default()
    }
}
