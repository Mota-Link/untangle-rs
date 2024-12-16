use crate::wasm4::{oval, rect, DRAW_COLORS};
use DrawPointsFlag::*;

#[derive(Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub type Points = Vec<Point>;

pub type PointsSpeed = Vec<(i8, i8)>;

impl Point {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn contain(&self, point: Point) -> bool {
        let Point { x, y } = point;
        let x_contain = (x - self.x).abs() <= 2;
        let y_contain = (y - self.y).abs() <= 2;
        x_contain && y_contain
    }

    pub fn move_to(&mut self, point: Point) {
        self.x = (point.x).min(158).max(1);
        self.y = (point.y).min(158).max(1);
    }

    pub fn draw(&self, bigger: bool) {
        if bigger {
            oval(self.x as i32 - 2, self.y as i32 - 2, 5, 5)
        } else {
            rect(self.x as i32 - 1, self.y as i32 - 1, 3, 3);
        }
    }
}

pub fn init_points(num_points: usize, shuffle: bool) -> Points {
    let mut points = Vec::with_capacity(num_points as usize);
    let radius = 60.0;
    let center_x = 80.0;
    let center_y = 80.0;

    let angle_step = 2.0 * core::f64::consts::PI / num_points as f64;
    for i in 0..num_points {
        let angle = angle_step * i as f64;
        let x = center_y + radius * angle.sin();
        let y = center_x + radius * angle.cos();
        points.push(Point::new(x as i32, y as i32));
    }

    if shuffle {
        fastrand::seed(unsafe { crate::GAME.seed });
        fastrand::shuffle(&mut points);
    }
    points
}

pub fn draw_points(points: &Points, flag: DrawPointsFlag) {
    for (i, point) in points.into_iter().enumerate() {
        let bigger = match flag {
            Hover(idx) if idx == i => true,
            Hoding(_, ref adjacent) if adjacent.contains(&i) => true,
            AllBig | AllBigLight => true,
            _ => false,
        };
        let colors = match flag {
            Hover(idx) | Hoding(idx, _) if idx == i => 0x31,
            Hoding(_, ref adjacent) if adjacent.contains(&i) => 0x32,
            AllLight | AllBigLight => 0x31,
            _ => 0x42,
        };
        unsafe { *DRAW_COLORS = colors };
        point.draw(bigger);
    }
}

pub enum DrawPointsFlag {
    Normal,
    Hoding(usize, Vec<usize>),
    Hover(usize),
    AllLight,
    AllBig,
    AllBigLight,
}

pub fn title_points() -> Points {
    init_points(9, false)
}

pub fn title_points_speed() -> PointsSpeed {
    vec![
        (-2, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
        (-1, -1),
        (-1, 2),
        (1, -1),
        (1, 2),
        (1, -1),
    ]
}
