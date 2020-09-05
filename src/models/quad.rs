use crate::point::Point;

#[derive(Debug, Clone)]
pub struct Quad {
    pub start: Point,
    pub end: Point,
    pub control1: Point,
}

impl Quad {
    pub fn bound(&self) -> (f64, f64, f64, f64) {
        let x = (self.end.1 - self.control1.1) / (self.start.1 + self.end.1 - 2.0 * self.control1.1);
        let y = (self.end.0 - self.control1.0) / (self.start.0 + self.end.0 - 2.0 * self.control1.0);
        let (x_min, x_max) = if 0.0 <= y && y <= 1.0 {
            (
                self.start.0.min(self.end.0).min(self.pos(y).0),
                self.start.0.max(self.end.0).max(self.pos(y).0),
            )
        } else {
            (
                self.start.0.min(self.end.0),
                self.start.0.max(self.end.0),
            )
        };
        let (y_min, y_max) = if 0.0 <= x && x <= 1.0 {
            (
                self.start.1.min(self.end.1).min(self.pos(x).1),
                self.start.1.max(self.end.1).max(self.pos(x).1),
            )
        } else {
            (
                self.start.1.min(self.end.1),
                self.start.1.max(self.end.1),
            )
        };
        (x_min, x_max, y_min, y_max)
    }

    pub fn pos(&self, t: f64) -> Point {
        let it = 1.0 - t;
        self.start * it.powi(2) + self.control1 * t * it * 2.0 + self.end * t.powi(2)
    }

    pub fn separate(&self, t: f64) -> (Quad, Quad) {
        debug_assert!(0.0 <= t && t <= 1.0);
        let middle = self.pos(t);
        (
            Quad {
                start: self.start,
                end: middle,
                control1: (self.control1 - self.start) * t + self.start,
            },
            Quad {
                start: middle,
                end: self.end,
                control1: (self.control1 - self.end) * (1.0 - t) + self.end,
            },
        )
    }

    pub fn closest_t_to_control(&self) -> f64 {
        let v0 = self.control1 - self.start;
        let v1 = self.end - self.control1;

        let a = (v1 - v0).dot(&(v1 - v0));
        let b = 3.0 * (v1.dot(&v0) - v0.dot(&v0));
        let c = 3.0 * v0.dot(&v0) - v1.dot(&v0);
        let d = -1.0 * v0.dot(&v0);

        let p = -b / (3.0 * a);
        let q = p.powi(3) + (b * c - 3.0 * a * d) / (6.0 * a.powi(2));
        let r = c / (3.0 * a);

        let s = (q.powi(2) + (r - p.powi(2)).powi(3)).sqrt();
        (q + s).cbrt() + (q - s).cbrt() + p
    }
}
