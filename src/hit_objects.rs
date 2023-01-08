pub struct Circle {
    point: Point,
    time: Time,
}

impl Circle {
    pub fn new(x: i32, y: i32, time: u32) -> Self {
        Self {
            point: Point { x, y },
            time: Time { milliseconds: time },
        }
    }
    pub fn velocity(&self, other: Self) -> f32 {
        let distance = self.point.distance(other.point);
        let time = std::cmp::max(self.time, other.time) - std::cmp::min(self.time, other.time);
        let time = time.seconds() as f32;
        distance / time
    }
}

pub struct Slider {
    point: Point,
    time: Time,
    curve_type: CurveType,
    curve_points: Vec<Point>,
    slides: u32,
    length: f32,
}

enum CurveType {
    Bezier,
    Catmull,
    Linear,
    Perfect,
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn distance(self, other: Self) -> f32 {
        let difference = self - other;
        let distance = (difference.x.pow(2) + difference.y.pow(2)) as f32;
        distance.sqrt()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Time {
    milliseconds: u32,
}

impl Time {
    pub fn ms(&self) -> u32 {
        self.milliseconds
    }

    pub fn seconds(&self) -> f32 {
        self.milliseconds as f32 / 1000.0
    }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Add for Time {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            milliseconds: self.milliseconds + rhs.milliseconds,
        }
    }
}

impl std::ops::Sub for Time {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            milliseconds: self.milliseconds - rhs.milliseconds,
        }
    }
}
