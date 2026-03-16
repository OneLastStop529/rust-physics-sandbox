use macroquad::prelude::{
    Color, color_u8, draw_circle_lines, draw_line, draw_rectangle_lines, draw_triangle,
    screen_height, screen_width,
};

use crate::math::{Aabb, Vec2};

// The DebugCamera is responsible for converting world coordinates to screen coordinates and scaling lengths from world units to pixels. It defines a simple orthographic projection where the world origin (0, 0) is centered on the screen, and the y-axis is flipped so that positive y goes up. The camera also calculates a pixels-per-unit scale factor based on the screen size, which allows it to maintain a consistent view of the world regardless of the actual screen resolution.
pub struct DebugCamera {
    pixels_per_unit: f32,
    screen_center: Vec2,
}

impl DebugCamera {
    pub fn from_screen(screen_width: f32, screen_height: f32) -> Self {
        Self {
            pixels_per_unit: (screen_width / 28.0).min(screen_height / 16.0),
            screen_center: Vec2::new(screen_width * 0.5, screen_height * 0.5),
        }
    }

    pub fn world_to_screen(&self, point: Vec2) -> Vec2 {
        Vec2::new(
            self.screen_center.x + point.x * self.pixels_per_unit,
            self.screen_center.y - point.y * self.pixels_per_unit,
        )
    }

    pub fn scale_length(&self, world_units: f32) -> f32 {
        world_units * self.pixels_per_unit
    }
}

// The DebugRenderer provides methods for drawing basic primitives (lines, circles, rectangles, points) in world coordinates. It uses the DebugCamera to convert these world coordinates to screen coordinates and applies consistent styling (colors, line thickness) to make the debug visuals clear and distinguishable. The renderer is designed to be simple and efficient, allowing it to be used for visualizing physics simulations, collision shapes, and other debug information without affecting the performance of the main application.
pub struct DebugRenderer {
    camera: DebugCamera,
    axis_color: Color,
    circle_color: Color,
    contact_normal_color: Color,
    contact_point_color: Color,
    line_color: Color,
    point_color: Color,
    rect_color: Color,
}

impl DebugRenderer {
    pub fn new(camera: DebugCamera) -> Self {
        Self {
            camera,
            axis_color: color_u8!(96, 113, 145, 255),
            circle_color: color_u8!(255, 192, 92, 255),
            contact_normal_color: color_u8!(244, 143, 177, 255),
            contact_point_color: color_u8!(255, 111, 0, 255),
            line_color: color_u8!(44, 88, 132, 255),
            point_color: color_u8!(255, 96, 96, 255),
            rect_color: color_u8!(83, 217, 194, 255),
        }
    }

    pub fn line(&mut self, start: Vec2, end: Vec2) {
        let screen_start = self.camera.world_to_screen(start);
        let screen_end = self.camera.world_to_screen(end);
        let color = if start.x == 0.0 || start.y == 0.0 || end.x == 0.0 || end.y == 0.0 {
            self.axis_color
        } else {
            self.line_color
        };

        draw_line(
            screen_start.x,
            screen_start.y,
            screen_end.x,
            screen_end.y,
            1.0,
            color,
        );
    }

    pub fn circle(&mut self, center: Vec2, radius: f32) {
        let screen_center = self.camera.world_to_screen(center);

        draw_circle_lines(
            screen_center.x,
            screen_center.y,
            self.camera.scale_length(radius),
            2.0,
            self.circle_color,
        );
    }

    pub fn aabb(&mut self, aabb: Aabb) {
        let min_world = aabb.min();
        let max_world = aabb.max();
        let top_left = self
            .camera
            .world_to_screen(Vec2::new(min_world.x, max_world.y));
        let width = self.camera.scale_length(max_world.x - min_world.x);
        let height = self.camera.scale_length(max_world.y - min_world.y);

        draw_rectangle_lines(top_left.x, top_left.y, width, height, 2.0, self.rect_color);
    }

    pub fn point(&mut self, point: Vec2) {
        let screen_point = self.camera.world_to_screen(point);
        let size = 6.0;

        draw_line(
            screen_point.x - size,
            screen_point.y,
            screen_point.x + size,
            screen_point.y,
            2.0,
            self.point_color,
        );
        draw_line(
            screen_point.x,
            screen_point.y - size,
            screen_point.x,
            screen_point.y + size,
            2.0,
            self.point_color,
        );
    }

    pub fn contact_point(&mut self, point: Vec2) {
        let screen_point = self.camera.world_to_screen(point);

        draw_circle_lines(
            screen_point.x,
            screen_point.y,
            5.0,
            2.0,
            self.contact_point_color,
        );
    }

    pub fn contact_normal(&mut self, point: Vec2, normal: Vec2, length: f32) {
        let screen_start = self.camera.world_to_screen(point);
        let screen_end = self.camera.world_to_screen(point + normal * length);

        draw_line(
            screen_start.x,
            screen_start.y,
            screen_end.x,
            screen_end.y,
            2.0,
            self.contact_normal_color,
        );
    }

    #[allow(dead_code)]
    pub fn origin_marker(&mut self) {
        let origin = self.camera.world_to_screen(Vec2::new(0.0, 0.0));
        draw_triangle(
            macroquad::prelude::Vec2::new(origin.x, origin.y - 8.0),
            macroquad::prelude::Vec2::new(origin.x - 7.0, origin.y + 5.0),
            macroquad::prelude::Vec2::new(origin.x + 7.0, origin.y + 5.0),
            self.point_color,
        );
    }
}

#[allow(dead_code)]
pub fn current_screen_size() -> Vec2 {
    Vec2::new(screen_width(), screen_height())
}
