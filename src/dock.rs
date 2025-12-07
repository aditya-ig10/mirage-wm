use smithay::utils::Logical;
use smithay::utils::Rectangle;

#[derive(Debug, Clone)]
pub struct DockApp {
    pub name: String,
    pub icon_path: Option<String>,
    pub command: String,
    pub is_running: bool,
}

pub struct Dock {
    pub apps: Vec<DockApp>,
    pub is_visible: bool,
    pub position_bottom: i32,
    pub icon_size: i32,
    pub icon_spacing: i32,
    pub background_height: i32,
}

impl Default for Dock {
    fn default() -> Self {
        Dock {
            apps: Vec::new(),
            is_visible: true,
            position_bottom: 20,
            icon_size: 48,
            icon_spacing: 10,
            background_height: 80,
        }
    }
}

impl Dock {
    pub fn new() -> Self {
        let mut dock = Self::default();
        dock.add_default_apps();
        dock
    }

    pub fn add_default_apps(&mut self) {
        // Add common macOS-like apps to dock
        self.add_app(DockApp {
            name: "Terminal".to_string(),
            icon_path: None,
            command: "kitty".to_string(),
            is_running: false,
        });

        self.add_app(DockApp {
            name: "Files".to_string(),
            icon_path: None,
            command: "nautilus".to_string(),
            is_running: false,
        });

        self.add_app(DockApp {
            name: "Text Editor".to_string(),
            icon_path: None,
            command: "gedit".to_string(),
            is_running: false,
        });
    }

    pub fn add_app(&mut self, app: DockApp) {
        if !self.apps.iter().any(|a| a.name == app.name) {
            self.apps.push(app);
        }
    }

    pub fn remove_app(&mut self, name: &str) {
        self.apps.retain(|app| app.name != name);
    }

    pub fn get_dock_rect(&self, screen_width: i32, screen_height: i32) -> Rectangle<i32, Logical> {
        let dock_width = (self.apps.len() as i32) * (self.icon_size + self.icon_spacing) + self.icon_spacing * 2;
        let x = (screen_width - dock_width) / 2;
        let y = screen_height - self.position_bottom - self.background_height;

        Rectangle::from_loc_and_size((x, y), (dock_width, self.background_height))
    }

    pub fn get_app_rect(
        &self,
        app_index: usize,
        screen_width: i32,
        screen_height: i32,
    ) -> Option<Rectangle<i32, Logical>> {
        if app_index >= self.apps.len() {
            return None;
        }

        let dock_rect = self.get_dock_rect(screen_width, screen_height);
        let x = dock_rect.loc.x + self.icon_spacing + (app_index as i32) * (self.icon_size + self.icon_spacing);
        let y = dock_rect.loc.y + (self.background_height - self.icon_size) / 2;

        Some(Rectangle::from_loc_and_size((x, y), (self.icon_size, self.icon_size)))
    }

    pub fn app_at_point(
        &self,
        point_x: f64,
        point_y: f64,
        screen_width: i32,
        screen_height: i32,
    ) -> Option<usize> {
        for (index, _app) in self.apps.iter().enumerate() {
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

    pub fn toggle_visibility(&mut self) {
        self.is_visible = !self.is_visible;
    }

    pub fn get_dock_color(&self) -> [f32; 3] {
        // Dark semi-transparent background, macOS-like
        [0.15, 0.15, 0.15]
    }

    pub fn get_app_icon_color(&self, is_running: bool) -> [f32; 3] {
        if is_running {
            // Bright color for running apps
            [0.2, 0.7, 1.0]
        } else {
            // Light gray for non-running apps
            [0.8, 0.8, 0.8]
        }
    }

    pub fn launch_app(&mut self, index: usize) -> bool {
        if index >= self.apps.len() {
            return false;
        }

        let app = &mut self.apps[index];
        // Try to launch the application
        let result = std::process::Command::new(&app.command)
            .spawn()
            .is_ok();

        if result {
            app.is_running = true;
        }

        result
    }

    pub fn set_app_running(&mut self, index: usize, running: bool) {
        if index < self.apps.len() {
            self.apps[index].is_running = running;
        }
    }
}
