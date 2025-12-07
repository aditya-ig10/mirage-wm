use smithay::utils::Logical;
use smithay::utils::Rectangle;

#[derive(Debug, Clone)]
pub struct LaunchpadApp {
    pub name: String,
    pub icon_path: Option<String>,
    pub command: String,
    pub category: AppCategory,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppCategory {
    Development,
    System,
    Utilities,
    Office,
    Media,
    Internet,
    Games,
    Other,
}

impl AppCategory {
    pub fn to_string(&self) -> String {
        match self {
            AppCategory::Development => "Development".to_string(),
            AppCategory::System => "System".to_string(),
            AppCategory::Utilities => "Utilities".to_string(),
            AppCategory::Office => "Office".to_string(),
            AppCategory::Media => "Media".to_string(),
            AppCategory::Internet => "Internet".to_string(),
            AppCategory::Games => "Games".to_string(),
            AppCategory::Other => "Other".to_string(),
        }
    }
}

pub struct Launchpad {
    pub apps: Vec<LaunchpadApp>,
    pub is_visible: bool,
    pub is_animating: bool,
    pub animation_progress: f32,
    pub grid_cols: usize,
    pub grid_rows: usize,
    pub icon_size: i32,
    pub icon_spacing: i32,
    pub margin: i32,
    pub search_query: String,
}

impl Default for Launchpad {
    fn default() -> Self {
        Launchpad {
            apps: Vec::new(),
            is_visible: false,
            is_animating: false,
            animation_progress: 0.0,
            grid_cols: 5,
            grid_rows: 4,
            icon_size: 64,
            icon_spacing: 20,
            margin: 40,
            search_query: String::new(),
        }
    }
}

impl Launchpad {
    pub fn new() -> Self {
        let mut launchpad = Self::default();
        launchpad.add_default_apps();
        launchpad
    }

    pub fn add_default_apps(&mut self) {
        // Development tools
        self.add_app(LaunchpadApp {
            name: "VS Code".to_string(),
            icon_path: None,
            command: "code".to_string(),
            category: AppCategory::Development,
        });

        // System utilities
        self.add_app(LaunchpadApp {
            name: "Settings".to_string(),
            icon_path: None,
            command: "gnome-control-center".to_string(),
            category: AppCategory::System,
        });

        // Terminal
        self.add_app(LaunchpadApp {
            name: "Terminal".to_string(),
            icon_path: None,
            command: "kitty".to_string(),
            category: AppCategory::System,
        });

        // Files
        self.add_app(LaunchpadApp {
            name: "Files".to_string(),
            icon_path: None,
            command: "nautilus".to_string(),
            category: AppCategory::Utilities,
        });

        // Media
        self.add_app(LaunchpadApp {
            name: "GIMP".to_string(),
            icon_path: None,
            command: "gimp".to_string(),
            category: AppCategory::Media,
        });

        // Browser
        self.add_app(LaunchpadApp {
            name: "Firefox".to_string(),
            icon_path: None,
            command: "firefox".to_string(),
            category: AppCategory::Internet,
        });

        // Text editor
        self.add_app(LaunchpadApp {
            name: "Text Editor".to_string(),
            icon_path: None,
            command: "gedit".to_string(),
            category: AppCategory::Office,
        });

        // Additional tools
        self.add_app(LaunchpadApp {
            name: "VLC".to_string(),
            icon_path: None,
            command: "vlc".to_string(),
            category: AppCategory::Media,
        });
    }

    pub fn add_app(&mut self, app: LaunchpadApp) {
        if !self.apps.iter().any(|a| a.name == app.name) {
            self.apps.push(app);
        }
    }

    pub fn remove_app(&mut self, name: &str) {
        self.apps.retain(|app| app.name != name);
    }

    pub fn toggle_visibility(&mut self) {
        self.is_visible = !self.is_visible;
        self.is_animating = true;
        self.animation_progress = if self.is_visible { 0.0 } else { 1.0 };
    }

    pub fn set_search_query(&mut self, query: String) {
        self.search_query = query.to_lowercase();
    }

    pub fn clear_search(&mut self) {
        self.search_query.clear();
    }

    pub fn get_filtered_apps(&self) -> Vec<&LaunchpadApp> {
        if self.search_query.is_empty() {
            self.apps.iter().collect()
        } else {
            self.apps
                .iter()
                .filter(|app| app.name.to_lowercase().contains(&self.search_query))
                .collect()
        }
    }

    pub fn get_apps_by_category(&self, category: AppCategory) -> Vec<&LaunchpadApp> {
        self.apps
            .iter()
            .filter(|app| app.category == category)
            .collect()
    }

    pub fn get_categories(&self) -> Vec<AppCategory> {
        let mut categories = Vec::new();
        for app in &self.apps {
            if !categories.contains(&app.category) {
                categories.push(app.category);
            }
        }
        categories.sort_by_key(|c| match c {
            AppCategory::Development => 0,
            AppCategory::System => 1,
            AppCategory::Utilities => 2,
            AppCategory::Office => 3,
            AppCategory::Media => 4,
            AppCategory::Internet => 5,
            AppCategory::Games => 6,
            AppCategory::Other => 7,
        });
        categories
    }

    pub fn get_launchpad_rect(&self, screen_width: i32, screen_height: i32) -> Rectangle<i32, Logical> {
        Rectangle::from_loc_and_size((0, 0), (screen_width, screen_height))
    }

    pub fn get_grid_rect(&self, screen_width: i32, screen_height: i32) -> Rectangle<i32, Logical> {
        let available_width = screen_width - (self.margin * 2);
        let available_height = screen_height - (self.margin * 2);

        Rectangle::from_loc_and_size((self.margin, self.margin), (available_width, available_height))
    }

    pub fn get_app_rect(
        &self,
        app_index: usize,
        screen_width: i32,
        screen_height: i32,
    ) -> Option<Rectangle<i32, Logical>> {
        let filtered_apps = self.get_filtered_apps();
        if app_index >= filtered_apps.len() {
            return None;
        }

        let grid_rect = self.get_grid_rect(screen_width, screen_height);
        let col = app_index % self.grid_cols;
        let row = app_index / self.grid_cols;

        let col_width = grid_rect.size.w / self.grid_cols as i32;
        let row_height = grid_rect.size.h / self.grid_rows as i32;

        let x = grid_rect.loc.x + (col as i32) * col_width + (col_width - self.icon_size) / 2;
        let y = grid_rect.loc.y + (row as i32) * row_height + (row_height - self.icon_size) / 2;

        Some(Rectangle::from_loc_and_size((x, y), (self.icon_size, self.icon_size)))
    }

    pub fn app_at_point(
        &self,
        point_x: f64,
        point_y: f64,
        screen_width: i32,
        screen_height: i32,
    ) -> Option<usize> {
        let filtered_apps = self.get_filtered_apps();
        for (index, _app) in filtered_apps.iter().enumerate() {
            if let Some(rect) = self.get_app_rect(index, screen_width, screen_height) {
                if point_x >= rect.loc.x as f64
                    && point_x < (rect.loc.x + rect.size.w) as f64
                    && point_y >= rect.loc.y as f64
                    && point_y < (rect.loc.y + rect.size.h) as f64
                {
                    return Some(index);
                }
            }
        }
        None
    }

    pub fn launch_app(&self, index: usize) -> bool {
        let filtered_apps = self.get_filtered_apps();
        if index >= filtered_apps.len() {
            return false;
        }

        let app = filtered_apps[index];
        std::process::Command::new(&app.command)
            .spawn()
            .is_ok()
    }

    pub fn get_background_color(&self) -> [f32; 3] {
        // Dark background with slight transparency effect, macOS-like
        [0.05, 0.05, 0.05]
    }

    pub fn get_icon_color(&self) -> [f32; 3] {
        // Light gray for app icons
        [0.85, 0.85, 0.85]
    }

    pub fn get_search_bar_color(&self) -> [f32; 3] {
        // Semi-transparent white for search bar
        [0.2, 0.2, 0.2]
    }

    pub fn update_animation(&mut self, _delta_time: f32) {
        if self.is_animating {
            let animation_speed = 0.02;
            if self.is_visible {
                self.animation_progress = (self.animation_progress + animation_speed).min(1.0);
            } else {
                self.animation_progress = (self.animation_progress - animation_speed).max(0.0);
            }

            if self.animation_progress >= 1.0 || self.animation_progress <= 0.0 {
                self.is_animating = false;
            }
        }
    }
}
