/// Window decorations - title bars, buttons, etc.

use smithay::utils::{Logical, Point, Rectangle};

/// Represents a window decoration element
#[derive(Debug, Clone, Copy)]
pub enum DecorationElement {
    TitleBar,
    CloseButton,
    MinimizeButton,
    MaximizeButton,
    ResizeHandle,
}

/// Window decoration parameters
#[derive(Debug, Clone)]
pub struct WindowDecoration {
    pub title: String,
    pub width: i32,
    pub height: i32,
    pub is_focused: bool,
    pub is_maximized: bool,
}

impl WindowDecoration {
    pub fn new(title: String, width: i32, height: i32, is_focused: bool) -> Self {
        Self {
            title,
            width,
            height,
            is_focused,
            is_maximized: false,
        }
    }

    /// Get the rectangle for the title bar
    pub fn title_bar_rect(&self) -> Rectangle<i32, Logical> {
        Rectangle::from_loc_and_size((0, 0), (self.width, 32))
    }

    /// Get the rectangle for the close button
    pub fn close_button_rect(&self) -> Rectangle<i32, Logical> {
        let right = self.width - 12;
        Rectangle::from_loc_and_size((right - 20, 6), (20, 20))
    }

    /// Get the rectangle for the minimize button
    pub fn minimize_button_rect(&self) -> Rectangle<i32, Logical> {
        let right = self.width - 12;
        Rectangle::from_loc_and_size((right - 50, 6), (20, 20))
    }

    /// Get the rectangle for the maximize button
    pub fn maximize_button_rect(&self) -> Rectangle<i32, Logical> {
        let right = self.width - 12;
        Rectangle::from_loc_and_size((right - 80, 6), (20, 20))
    }

    /// Get the content area (excluding decorations)
    pub fn content_rect(&self) -> Rectangle<i32, Logical> {
        Rectangle::from_loc_and_size(
            (0, 32),
            (self.width, self.height.saturating_sub(32)),
        )
    }

    /// Check if a point is on the close button
    pub fn point_on_close_button(&self, point: Point<f64, Logical>) -> bool {
        let rect = self.close_button_rect();
        point.x >= rect.loc.x as f64
            && point.x < (rect.loc.x + rect.size.w) as f64
            && point.y >= rect.loc.y as f64
            && point.y < (rect.loc.y + rect.size.h) as f64
    }

    /// Check if a point is on the minimize button
    pub fn point_on_minimize_button(&self, point: Point<f64, Logical>) -> bool {
        let rect = self.minimize_button_rect();
        point.x >= rect.loc.x as f64
            && point.x < (rect.loc.x + rect.size.w) as f64
            && point.y >= rect.loc.y as f64
            && point.y < (rect.loc.y + rect.size.h) as f64
    }

    /// Check if a point is on the maximize button
    pub fn point_on_maximize_button(&self, point: Point<f64, Logical>) -> bool {
        let rect = self.maximize_button_rect();
        point.x >= rect.loc.x as f64
            && point.x < (rect.loc.x + rect.size.w) as f64
            && point.y >= rect.loc.y as f64
            && point.y < (rect.loc.y + rect.size.h) as f64
    }

    /// Check if a point is on the title bar
    pub fn point_on_title_bar(&self, point: Point<f64, Logical>) -> bool {
        let rect = self.title_bar_rect();
        point.x >= rect.loc.x as f64
            && point.x < (rect.loc.x + rect.size.w) as f64
            && point.y >= rect.loc.y as f64
            && point.y < (rect.loc.y + rect.size.h) as f64
    }

    /// Get title bar color based on focus state
    pub fn title_bar_color(&self) -> (f32, f32, f32, f32) {
        if self.is_focused {
            (0.2, 0.2, 0.2, 1.0) // Dark gray for focused
        } else {
            (0.15, 0.15, 0.15, 1.0) // Darker gray for unfocused
        }
    }

    /// Get button color
    pub fn button_color(&self) -> (f32, f32, f32, f32) {
        (0.3, 0.3, 0.3, 1.0)
    }

    /// Get close button color (red)
    pub fn close_button_color(&self) -> (f32, f32, f32, f32) {
        (0.9, 0.2, 0.2, 1.0)
    }

    /// Get minimize button color (yellow)
    pub fn minimize_button_color(&self) -> (f32, f32, f32, f32) {
        (0.9, 0.8, 0.2, 1.0)
    }

    /// Get maximize button color (green)
    pub fn maximize_button_color(&self) -> (f32, f32, f32, f32) {
        (0.2, 0.9, 0.2, 1.0)
    }
}
