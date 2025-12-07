use smithay::utils::{Point, Size, Rectangle, Logical};

/// Information about a window's position and size
#[derive(Debug, Clone, Copy)]
pub struct WindowGeometry {
    pub location: Point<i32, Logical>,
    pub size: Size<i32, Logical>,
}

impl WindowGeometry {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            location: Point::new(x, y),
            size: Size::from((width, height)),
        }
    }

    /// Get the bounding rectangle of this window
    pub fn rect(&self) -> Rectangle<i32, Logical> {
        Rectangle::new(self.location, self.size)
    }

    /// Check if a point is inside this window
    pub fn contains_point(&self, point: Point<f64, Logical>) -> bool {
        let x = point.x as i32;
        let y = point.y as i32;
        let rect = self.rect();
        rect.contains((x, y))
    }
}

/// Simple tiling layout - splits screen into vertical tiles
pub struct TilingLayout {
    screen_size: Size<i32, Logical>,
}

impl TilingLayout {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            screen_size: Size::from((width, height)),
        }
    }

    pub fn update_screen_size(&mut self, width: i32, height: i32) {
        self.screen_size = Size::from((width, height));
    }

    /// Calculate geometry for a window at the given index
    pub fn calculate_geometry(&self, window_index: usize, total_windows: usize) -> WindowGeometry {
        if total_windows == 0 {
            return WindowGeometry::new(0, 0, self.screen_size.w, self.screen_size.h);
        }

        if total_windows == 1 {
            // Single window takes full screen
            return WindowGeometry::new(0, 0, self.screen_size.w, self.screen_size.h);
        }

        // Multiple windows: simple vertical split
        // First window takes left half, others stack on right
        let half_width = self.screen_size.w / 2;
        
        if window_index == 0 {
            // Master window on left
            WindowGeometry::new(0, 0, half_width, self.screen_size.h)
        } else {
            // Stack windows on right
            let stack_height = self.screen_size.h / (total_windows - 1) as i32;
            let stack_index = (window_index - 1) as i32;
            let y = stack_index * stack_height;
            
            WindowGeometry::new(
                half_width,
                y,
                self.screen_size.w - half_width,
                stack_height,
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_window() {
        let layout = TilingLayout::new(1280, 800);
        let geom = layout.calculate_geometry(0, 1);
        assert_eq!(geom.location.x, 0);
        assert_eq!(geom.location.y, 0);
        assert_eq!(geom.size.w, 1280);
        assert_eq!(geom.size.h, 800);
    }

    #[test]
    fn test_point_contains() {
        let geom = WindowGeometry::new(100, 100, 200, 200);
        let inside = Point::<f64, Logical>::new(150.0, 150.0);
        let outside = Point::<f64, Logical>::new(50.0, 50.0);
        
        assert!(geom.contains_point(inside));
        assert!(!geom.contains_point(outside));
    }
}
