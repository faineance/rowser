use std::{
    f32::consts::PI as PI32,
    io::{self, Write},
};
const MAX_FONT_SIZE: f32 = 2000.0;

pub struct ViewState {
    pub mouse_position: (f32, f32),
    pub font_size: f32,
    pub zoom: f32,
    pub angle: f32,
    pub text: String,
    pub scroll_offset: f32,
}

impl Default for ViewState {
    fn default() -> Self {
        ViewState {
            mouse_position: (0.0, 0.0),
            font_size: 18.0,
            zoom: 1.0,
            angle: 0.0,
            text: "".to_string(),
            scroll_offset: 0.,
        }
    }
}

impl ViewState {
    pub fn rotate(&mut self, y: f32) {
        if y > 0.0 {
            self.angle += 0.02 * PI32;
        } else {
            self.angle -= 0.02 * PI32;
        }
        if (self.angle % (PI32 * 2.0)).abs() < 0.01 {
            self.angle = 0.0;
        }
        print!("\r                            \r");
        print!("transform-angle -> {:.2} * π", self.angle / PI32);
        let _ = io::stdout().flush();
    }
    pub fn zoom(&mut self, y: f32) {
        let old_zoom = self.zoom;
        // increase/decrease zoom
        if y > 0.0 {
            self.zoom += 0.1;
        } else {
            self.zoom -= 0.1;
        }
        self.zoom = self.zoom.min(1.0).max(0.1);
        if (self.zoom - old_zoom).abs() > 1e-2 {
            print!("\r                            \r");
            print!("transform-zoom -> {:.1}", self.zoom);
            let _ = io::stdout().flush();
        }
    }
    pub fn scale(&mut self, y: f32) {
        // increase/decrease font size
        let old_size = self.font_size;
        let mut size = self.font_size;
        if y > 0.0 {
            size += (size / 4.0).max(2.0)
        } else {
            size *= 4.0 / 5.0
        };
        self.font_size = size.max(1.0).min(MAX_FONT_SIZE);
        if (self.font_size - old_size).abs() > 1e-2 {
            print!("\r                            \r");
            print!("font-size -> {:.1}", self.font_size);
            let _ = io::stdout().flush();
        }
    }
    pub fn scroll(&mut self, scroll: f32) {
        
        self.scroll_offset = scroll
    }
    // pub fn click(&mut self, y: f32)
}
