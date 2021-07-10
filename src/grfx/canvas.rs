use crate::grfx::color;
use crate::grfx::color::Color;
use crate::math::vector::{FVec2D, Point2D};

#[allow(dead_code, unused_variables)]
pub struct Canvas {
    width: u32,
    height: u32,
    pub pixels: Vec<Color>,
}
impl Canvas {
    pub fn new(width: u32, height: u32) -> Self {
        let mut pixels = Vec::with_capacity((width * height) as usize);
        for _ in 0..pixels.capacity() {
            pixels.push(color::BLACK); // initialize to black pixels;
        }

        Self {
            width,
            height,
            pixels,
        }
    }

    pub fn clear(&mut self, color: Color) {
        for pixel in &mut self.pixels {
            *pixel = color;
        }
    }

    ///
    ///  Plots a single pixel at the given coordinates
    ///     x  -> X axis offset
    ///     y  -> y axis offset
    ///     color -> pixel color
    ///
    pub fn plot(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            let normalized_position = (y * self.width as i32 + x) as usize;
            if normalized_position < self.pixels.len() {
                self.pixels[normalized_position] = color;
            }
        }
    }

    pub fn line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: Color) {
        let origin = Point2D::new(x0, y0);
        let end = Point2D::new(x1, y1);

        self.line_between(origin, end, color);
    }
    ///
    /// Draws a line using Bresenham Algorthm
    /// origin: start point
    /// dest: final point
    /// color: Pixel color
    ///
    /// https://www.programmersought.com/article/60715259426/
    pub fn line_between(&mut self, origin: Point2D, dest: Point2D, color: Color) {
        let x0 = origin.x;
        let y0 = origin.y;

        let x1 = dest.x;
        let y1 = dest.y;

        let dx = if x1 >= x0 { x1 - x0 } else { x0 - x1 };
        let dy = if x1 >= x0 { y1 - y0 } else { y0 - y1 };

        // ensure we go from smaller to highest
        let mut x = if x1 >= x0 { x0 } else { x1 };
        let mut y = if x1 >= x0 { y0 } else { y1 };

        // veritical line
        if dx == 0 {
            for i in y0.min(y1)..y0.max(y1) {
                self.plot(x, i, color);
            }
            return;
        }
        // horizontal line
        if dy == 0 {
            for i in x0.min(x1)..x0.max(x1) {
                self.plot(i, y, color);
            }
            return;
        }
        // slope is less than 1
        if dy.abs() <= dx {
            let mut decision = 2 * dy.abs() - dx;
            self.plot(x, y, color);
            while x < x0.max(x1) {
                x += 1;
                if decision < 0 {
                    decision = decision + 2 * dy.abs();
                } else {
                    y += if dy >= 0 { 1 } else { -1 };
                    decision = decision + 2 * (dy.abs() - dx);
                }
                self.plot(x, y, color);
            }
        } else {
            // slopw is > 1 and dy positions are swapped
            let mut decision = 2 * dx - dy.abs();
            self.plot(x, y, color);
            while x < x0.max(x1) {
                y += if dy >= 0 { 1 } else { -1 };
                if decision < 0 {
                    decision = decision + 2 * dx;
                } else {
                    x += 1;
                    decision = decision + 2 * (dx - dy.abs());
                }
                self.plot(x, y, color);
            }
        }
    }

    ///
    /// Draws a hollow circle using Bresenham Algortim for circles
    /// origin: Center of circle
    /// Radius: Radius
    /// color: Pixel color
    /// https://iq.opengenus.org/bresenhams-circle-drawing-algorithm/
    ///
    pub fn circle(&mut self, origin: Point2D, radius: i32, color: Color) {
        let mut x = 0;
        let mut y = radius;
        let mut decision = 3 - 2 * y;
        while y >= x {
            self.plot(origin.x + x, origin.y + y, color);
            self.plot(origin.x - x, origin.y + y, color);
            self.plot(origin.x + x, origin.y - y, color);
            self.plot(origin.x - x, origin.y - y, color);
            self.plot(origin.x + y, origin.y + x, color);
            self.plot(origin.x - y, origin.y + x, color);
            self.plot(origin.x + y, origin.y - x, color);
            self.plot(origin.x - y, origin.y - x, color);

            x += 1;
            if decision > 0 {
                y -= 1;
                decision = decision + 4 * (x - y) + 10;
            } else {
                decision = decision + 4 * x + 6;
            }

            // break;
        }
    }

    ///
    /// Draws a hollow rectangle
    /// Takes:
    /// origin: toip left corner
    /// Width
    /// Height
    /// Color for pixels
    ///
    pub fn rectangle(&mut self, origin: Point2D, width: i32, height: i32, color: Color) {
        let top_right = Point2D::new(origin.x + width, origin.y);
        let bottom_left = Point2D::new(origin.x, origin.y + height);
        let bottom_right = Point2D::new(origin.x + width, origin.y + height);

        self.line_between(origin, top_right, color);
        self.line_between(origin, bottom_left, color);
        self.line_between(bottom_left, bottom_right, color);
        self.line_between(top_right, bottom_right, color);
    }

    ///
    /// Draws a hollow rectangle
    /// Takes:
    /// origin: toip left corner
    /// Width
    /// Height
    /// Color for pixels
    ///
    pub fn filled_rectangle(&mut self, origin: Point2D, width: i32, height: i32, color: Color) {
        for i in 0..=height {
            let right = Point2D::new(origin.x, origin.y + i);
            let left = Point2D::new(origin.x + width, origin.y + i);
            self.line_between(right, left, color);
        }
    }

    ///
    /// Draws a hollow rectangle
    /// Takes:
    /// v1 first point
    /// v2 second point
    /// v3 third point
    /// color: Color for the pixels
    ///
    pub fn triangle(&mut self, v1: Point2D, v2: Point2D, v3: Point2D, color: Color) {
        self.line_between(v1, v2, color);
        self.line_between(v2, v3, color);
        self.line_between(v3, v1, color);
    }

    ///
    /// Draw a given polygone based on the given vertices/points vector
    /// vertices -> Points to connect
    /// color -> color to paint them
    pub fn polygon(&mut self, vertices: &Vec<Point2D>, color: Color) {
        let len = vertices.len();
        if len >= 3 {
            let first = vertices[0];
            let last = vertices[len - 1];
            for i in 1..len {
                self.line_between(vertices[i - 1], vertices[i], color);
            }
            self.line_between(last, first, color);
        }
    }

    ///
    /// Draw a regular polygone based on the given start point, number of sides and length
    /// origin -> First point on the polygon
    /// sides -> Number of sides on the polygon
    /// length -> Length of each side of the polygon
    /// color -> Line color
    ///
    /// Not sure if there is a more efficient way of doing this but its similar to to the process of drawing a traingle.
    /// In this case we simply need to know that the exterior angles of a polygon are always 360 degrees or 2PI.
    ///
    /// We create a vector that runs from one point to the next.
    ///     To determine what the next point is we use the angle for each side (360/sides) and increment accordingly
    /// Then we simply draw a line between the previous point and the next one as they will always allign to close the polygon
    ///  due to the angle calculation.
    ///             
    ///
    ///   O ->   ******
    ///       *         *  <-- a
    ///      *            *
    /// F->  *            *   <---b
    ///      *            *
    /// e-->   *        *   <--- c
    ///         ******
    ///             ^
    ///              d
    ///
    ///    In this example we have an exampon so each line differs from the previous in exactly 60 degrees (pi/3)
    ///     Point O is at 0 degress
    ///     Point a is at 60 degress
    ///     Point b is at 120 degress
    ///     Point c is at 180 degress
    ///     Point d is at 240 degress
    ///     Point e is at 300 degress
    ///     Point f is at 360 degress
    ///
    pub fn regular_polygon(&mut self, origin: Point2D, sides: i32, length: i32, color: Color) {
        let delta_angle = 2.0 * std::f32::consts::PI / sides as f32;
        let mut current_angle = 0.0;
        let mut start = origin.to_f32();
        let mut next: FVec2D;

        for _ in 0..sides {
            next = start + FVec2D::from_polar(length as f32, current_angle);
            current_angle += delta_angle;
            self.line_between(start.to_i32(), next.to_i32(), color);
            start = next;
        }
    }

    ///
    /// Draws a filled circle
    /// Takes:
    /// origin: toip left corner
    /// Width
    /// Height
    /// Color for pixels
    ///  Helpful material to get this working:
    /// https://iq.opengenus.org/bresenhams-circle-drawing-algorithm/
    /// https://stackoverflow.com/questions/1201200/fast-algorithm-for-drawing-filled-circles
    /// https://github.com/OneLoneCoder/olcPixelGameEngine/blob/master/olcPixelGameEngine.h  (Javidx9  github)
    ///
    pub fn filled_circle(&mut self, origin: Point2D, radius: i32, color: Color) {
        let mut x = 0;
        let mut y = radius;
        let mut decision = 3 - 2 * y;
        while y >= x {
            self.line(
                origin.x + x,
                origin.y - y,
                origin.x + x,
                origin.y + y,
                color,
            );
            self.line(
                origin.x - x,
                origin.y - y,
                origin.x - x,
                origin.y + y,
                color,
            );
            self.line(
                origin.x - y,
                origin.y - x,
                origin.x + y,
                origin.y - x,
                color,
            );
            self.line(
                origin.x - y,
                origin.y + x,
                origin.x + y,
                origin.y + x,
                color,
            );

            x += 1;
            if decision > 0 {
                y -= 1;
                decision = decision + 4 * (x - y) + 10;
            } else {
                decision = decision + 4 * x + 6;
            }
        }
    }
}
