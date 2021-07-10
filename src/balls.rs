use crate::grfx::canvas::Canvas;
use crate::grfx::color;
use crate::grfx::color::Color;
use crate::grfx::render::Render2D;
use crate::math::vector::FVec2D;
use crate::math::vector::Point2D;
use rand::Rng;
use std::time::Duration;
use winit_input_helper::WinitInputHelper;
pub struct Drawable {
    title: String,
    width: u32,
    height: u32,
    selected_circle: Option<usize>,
    selected_line: LineSelection,
    circles: Vec<Circle>,
    lines: Vec<LineSegment>,
}

impl Drawable {
    pub fn new(title: String, width: u32, height: u32) -> Self {
        Self {
            title,
            width,
            height,
            selected_circle: None,
            selected_line: LineSelection::None,
            circles: Vec::new(),
            lines: Vec::new(),
        }
    }
}

impl Render2D for Drawable {
    fn setup(&mut self, _canvas: &mut Canvas) -> bool {
        let mut rand = rand::thread_rng();
        let widths = 0..self.width() as i32;
        let heights = 0..self.height() as i32;
        for _ in 0..=100 {
            let circle = Circle::new(
                FVec2D::new(
                    rand.gen_range(widths.clone()) as f32,
                    rand.gen_range(heights.clone()) as f32,
                ),
                8.0,
                FVec2D::new(0.0, 0.0),
                color::RED,
            );
            self.circles.push(circle);
        }
        self.lines.push(LineSegment::new(
            FVec2D::new(30.0, 30.0),
            FVec2D::new(300.0, 30.0),
            10.0,
        ));
        self.lines.push(LineSegment::new(
            FVec2D::new(30.0, 50.0),
            FVec2D::new(300.0, 50.0),
            10.0,
        ));
        self.lines.push(LineSegment::new(
            FVec2D::new(30.0, 80.0),
            FVec2D::new(300.0, 80.0),
            10.0,
        ));
        self.lines.push(LineSegment::new(
            FVec2D::new(30.0, 120.0),
            FVec2D::new(300.0, 120.0),
            10.0,
        ));
        return true;
    }
    fn update(&mut self, canvas: &mut Canvas, input: &WinitInputHelper, delta_t: Duration) -> bool {
        canvas.clear(color::BLACK);

        let mut colliding_circles = Vec::<(usize, usize, bool)>::new();
        let mut fake_balls = Vec::<Circle>::new();

        let width = self.width() as f32;
        let height = self.height() as f32;

        let circles = &mut self.circles;
        if input.mouse_pressed(0) || input.mouse_pressed(1) {
            self.selected_line = LineSelection::None;
            self.selected_circle = None;
            for (i, circle) in circles.iter_mut().enumerate() {
                if let Some((x, y)) = input.mouse() {
                    let clicked_point = FVec2D::new(x, y).to_i32();
                    if circle.hits(clicked_point, circle.radius) {
                        self.selected_circle = Some(i);
                    }
                }
            }

            for (i, line) in self.lines.iter_mut().enumerate() {
                if let Some((x, y)) = input.mouse() {
                    let clicked_point = FVec2D::new(x, y).to_i32();
                    let circle =
                        Circle::new(line.start, line.radius, FVec2D::new(0.0, 0.0), color::WHITE);
                    if circle.hits(clicked_point, circle.radius) {
                        self.selected_line = LineSelection::Head(i);
                    }
                    let circle =
                        Circle::new(line.end, line.radius, FVec2D::new(0.0, 0.0), color::WHITE);
                    if circle.hits(clicked_point, circle.radius) {
                        self.selected_line = LineSelection::Tail(i);
                    }
                }
            }
        }

        // drag with left click
        if input.mouse_held(0) {
            if let Some((x, y)) = input.mouse() {
                let selected_point = FVec2D::new(x, y);
                if let Some(index) = self.selected_circle {
                    circles[index].center = selected_point;
                }
                if let LineSelection::Head(i) = self.selected_line {
                    self.lines[i].start = selected_point;
                }
                if let LineSelection::Tail(i) = self.selected_line {
                    self.lines[i].end = selected_point;
                }
            }
        }
        if input.mouse_held(1) {
            if let Some(index) = self.selected_circle {
                if let Some((x, y)) = input.mouse() {
                    let selected_point = FVec2D::new(x, y).to_i32();
                    canvas.line_between(
                        selected_point,
                        circles[index].center.to_i32(),
                        color::BLUE,
                    );
                }
            }
        }
        if input.mouse_released(0) {
            self.selected_circle = None;
            self.selected_line = LineSelection::None;
        }

        // give push with dragging and dropping right click
        if input.mouse_released(1) {
            if let Some(index) = self.selected_circle {
                if let Some((x, y)) = input.mouse() {
                    let selected_point = FVec2D::new(x.abs(), y);
                    circles[index].speed = (circles[index].center - selected_point) * 5.0;
                }
            }
        }
        let simulation_updates = 4;
        let max_simulation_steps = 15;
        let sim_elapsed_time = delta_t.as_secs_f32() / simulation_updates as f32;
        for _ in 0..simulation_updates {
            for circle in circles.iter_mut() {
                circle.sim_time_remaining = sim_elapsed_time;
            }
            for _ in 0..max_simulation_steps {
                for (_i, circle) in circles.iter_mut().enumerate() {
                    if circle.sim_time_remaining > 0.0 {
                        // cache current center
                        circle.prev_center = circle.center;

                        circle.acceletation = -circle.speed * 0.8 + FVec2D::new(0.0, 100.0); // drag force + gravity
                        circle.speed += circle.acceletation * circle.sim_time_remaining;
                        circle.center += circle.speed * circle.sim_time_remaining;
                        if circle.center.x < 0.0 {
                            circle.center.x += width;
                        }
                        if circle.center.y < 0.0 {
                            circle.center.y += height;
                        }
                        if circle.center.x > width {
                            circle.center.x -= width;
                        }
                        if circle.center.y > height {
                            circle.center.y -= height;
                        }
                        if circle.speed.length() <= 0.01 {
                            circle.speed = FVec2D::new(0.0, 0.0);
                        }
                    }
                }
                // check for static collisions
                for i in 0..circles.len() {
                    // check collisions with edges
                    for edge in self.lines.iter_mut() {
                        let line_segment = edge.end - edge.start;
                        let edge_to_circle_vec = circles[i].center - edge.start;
                        let segment_length = line_segment.squared_length();

                        let t = segment_length.min(FVec2D::dot(edge_to_circle_vec, line_segment))
                            / segment_length;
                        let t = t.max(0.0);

                        let closest_point = edge.start + line_segment * t;

                        let distance = (circles[i].center - closest_point).length();

                        // colliding with edge
                        if distance <= circles[i].radius + edge.radius {
                            let mut fake_circle = Circle::new(
                                closest_point,
                                edge.radius,
                                -circles[i].speed,
                                circles[i].color,
                            );

                            //ereduce the mass a little
                            fake_circle.mass *= 0.8;
                            fake_balls.push(fake_circle);
                            colliding_circles.push((i, fake_balls.len() - 1, true)); //  hack store fake circles indexes

                            let overlap = distance - circles[i].radius - fake_circle.radius;
                            let circle_center = circles[i].center;
                            circles[i].center -=
                                (circle_center - fake_circle.center).unit_vector() * overlap;
                        }
                    }
                    for j in 0..circles.len() {
                        if i != j {
                            // make sure circles don't run into each other
                            if circles_overlap(&circles[i], &circles[j]) {
                                let distance_vec = (circles[i].center - circles[j].center).to_f32();
                                let overlap = 0.5
                                    * (distance_vec.length()
                                        - circles[i].radius
                                        - circles[j].radius);
                                circles[i].center -= distance_vec.unit_vector() * overlap;
                                circles[j].center += distance_vec.unit_vector() * overlap;
                                colliding_circles.push((i, j, false));
                            }
                        }
                    }
                    let intended_speed = circles[i].speed.length();
                    let _intended_distance = intended_speed * circles[i].sim_time_remaining;
                    let actual_distance = (circles[i].center - circles[i].prev_center).length();
                    let actual_time = actual_distance / intended_speed;

                    circles[i].sim_time_remaining -= actual_time;
                }
                // handle colliding circle. If they are hit reflect their speed and make them move accordingly
                // The hit ball hits in the direction tangent of the colision while the hitter moves direction of the normal vector
                for pair in &colliding_circles {
                    let first = circles[pair.0];

                    let second = if pair.2 {
                        fake_balls[pair.1]
                    } else {
                        circles[pair.1]
                    };

                    let distance = second.center - first.center;
                    let normal = distance.unit_vector();
                    let tangental = normal.perpendicular();

                    let tan_speed1 = FVec2D::dot(first.speed, tangental);
                    let tan_speed2 = FVec2D::dot(second.speed, tangental);

                    let norm_speed1 = FVec2D::dot(first.speed, normal);
                    let norm_speed2 = FVec2D::dot(second.speed, normal);

                    // conservation of momentum in 1D
                    // elastic collisions https://en.wikipedia.org/wiki/Elastic_collision
                    //https://www.youtube.com/watch?v=LPzyNOHY3A4&t=1077s&ab_channel=javidx9
                    let m1 = ((norm_speed1 * (first.mass - second.mass))
                        + 2.0 * second.mass * norm_speed2)
                        / (first.mass + second.mass);
                    let m2 = ((norm_speed2 * (second.mass - first.mass))
                        + 2.0 * first.mass * norm_speed1)
                        / (first.mass + second.mass);

                    // Update with new speeds and all circles are in original vector
                    if !pair.2 {
                        circles[pair.0].speed = tangental * tan_speed1 + normal * m1;
                        circles[pair.1].speed = tangental * tan_speed2 + normal * m2;
                    } else {
                        // second circle is fake ball
                        circles[pair.0].speed = tangental * tan_speed1 + normal * m1;
                        // fake_balls[pair.1].speed = tangental * tan_speed2 + normal * m2;
                    }
                }
                colliding_circles.clear();
                fake_balls.clear();
            }
        }

        // draw circles
        for circle in circles.iter() {
            canvas.filled_circle(circle.center.to_i32(), circle.radius as i32, circle.color);
        }

        // draw line segments
        for line in self.lines.iter() {
            canvas.filled_circle(line.start.to_i32(), line.radius as i32, color::WHITE);
            canvas.filled_circle(line.end.to_i32(), line.radius as i32, color::WHITE);

            // one line on bottom of the circles
            let normal = (line.end - line.start).perpendicular().unit_vector();
            let line_start = (normal * line.radius) + line.start;
            let line_end = (normal * line.radius) + line.end;
            canvas.line_between(line_start.to_i32(), line_end.to_i32(), color::WHITE);

            // another line on top of circle
            let line_start = -(normal * line.radius) + line.start;
            let line_end = -(normal * line.radius) + line.end;
            canvas.line_between(line_start.to_i32(), line_end.to_i32(), color::WHITE);
        }

        return true;
    }

    fn title(&mut self) -> String {
        self.title.clone()
    }
    fn height(&mut self) -> u32 {
        self.height
    }
    fn width(&mut self) -> u32 {
        self.width
    }
}

fn circles_overlap(c1: &Circle, c2: &Circle) -> bool {
    let vec = (c2.center - c1.center).to_f32();
    vec.squared_length() <= (c1.radius + c2.radius) * (c1.radius + c2.radius)
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Circle {
    pub center: FVec2D,
    pub radius: f32,
    pub speed: FVec2D,
    pub sim_time_remaining: f32,
    pub prev_center: FVec2D,
    pub mass: f32,
    pub acceletation: FVec2D,
    pub color: color::Color,
    pub selected: bool,
}

impl Circle {
    pub fn hits(&self, current: Point2D, width: f32) -> bool {
        let delta_vec = current.to_f32() - self.center;
        let distance_from_center = self.radius * self.radius - delta_vec.squared_length();
        distance_from_center < width * width && distance_from_center > 0.0
    }
    pub fn new(center: FVec2D, radius: f32, speed: FVec2D, color: Color) -> Self {
        Self {
            center,
            radius,
            speed,
            mass: radius * 10.0,
            acceletation: FVec2D::new(0.0, 0.0),
            color,
            selected: false,
            sim_time_remaining: 0.0,
            prev_center: center,
        }
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        canvas.filled_circle(self.center.to_i32(), self.radius as i32, self.color);
    }
}
enum LineSelection {
    None,
    Head(usize),
    Tail(usize),
}
struct LineSegment {
    pub start: FVec2D,
    pub end: FVec2D,
    pub radius: f32,
}

impl LineSegment {
    pub fn new(start: FVec2D, end: FVec2D, radius: f32) -> Self {
        Self { start, end, radius }
    }
}
