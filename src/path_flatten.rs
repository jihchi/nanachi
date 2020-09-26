use crate::models::Line;
use crate::path::{Path, PathItem};
use crate::point::Point;
use lyon_geom::{
    euclid::{default::Point2D, Angle},
    Arc, CubicBezierSegment, QuadraticBezierSegment,
};

enum FlattenState {
    Line(Point),
    Lines(Vec<Point>, usize),
    End,
}

pub struct Flatten<'a, I: Iterator<Item = &'a PathItem>> {
    tolerance: f64,
    last: Point,
    state: FlattenState,
    path_items: I,
}

impl<'a, I: Iterator<Item = &'a PathItem>> Flatten<'a, I> {
    pub fn new(it: I, tolerance: f64) -> Self {
        let mut flatten = Flatten {
            tolerance,
            last: Point(0.0, 0.0),
            state: FlattenState::End,
            path_items: it,
        };
        flatten.next_state();
        flatten
    }

    fn next_state(&mut self) {
        if let Some(pi) = self.path_items.next() {
            match pi {
                PathItem::Line(l) => {
                    self.last = l.0;
                    self.state = FlattenState::Line(l.1);
                }
                PathItem::Arc(arc) => {
                    let arc = Arc {
                        center: point_to_point2d(&arc.center),
                        radii: (arc.radius, arc.radius).into(),
                        start_angle: Angle::radians(arc.angle1),
                        sweep_angle: Angle::radians(arc.angle2 - arc.angle1),
                        x_rotation: Angle::radians(0.0),
                    };
                    let vec: Vec<_> = if arc.sweep_angle.radians.is_sign_positive() {
                        vec![arc.from()]
                            .into_iter()
                            .chain(arc.flattened(self.tolerance))
                            .map(|x| x.to_tuple().into())
                            .collect()
                    } else {
                        let mut vec: Vec<_> = vec![arc.to()]
                            .into_iter()
                            .chain(arc.flip().flattened(self.tolerance))
                            .map(|x| x.to_tuple().into())
                            .collect();
                        vec.reverse();
                        vec
                    };
                    self.last = vec[0];
                    self.state = FlattenState::Lines(vec, 1);
                }
                PathItem::Ellipse(ellipse) => {
                    let arc = Arc {
                        center: point_to_point2d(&ellipse.center),
                        radii: (ellipse.radius_x, ellipse.radius_y).into(),
                        start_angle: Angle::radians(ellipse.angle1),
                        sweep_angle: Angle::radians(ellipse.angle2 - ellipse.angle1),
                        x_rotation: Angle::radians(ellipse.rotation),
                    };
                    let vec: Vec<_> = if arc.sweep_angle.radians.is_sign_positive() {
                        vec![arc.from()]
                            .into_iter()
                            .chain(arc.flattened(self.tolerance))
                            .map(|x| x.to_tuple().into())
                            .collect()
                    } else {
                        let mut vec: Vec<_> = vec![arc.to()]
                            .into_iter()
                            .chain(arc.flip().flattened(self.tolerance))
                            .map(|x| x.to_tuple().into())
                            .collect();
                        vec.reverse();
                        vec
                    };
                    self.last = vec[0];
                    self.state = FlattenState::Lines(vec, 1);
                }
                PathItem::Quad(quad) => {
                    let it = QuadraticBezierSegment {
                        from: point_to_point2d(&quad.start),
                        ctrl: point_to_point2d(&quad.control1),
                        to: point_to_point2d(&quad.end),
                    }
                    .flattened(self.tolerance)
                    .map(|x| x.to_tuple().into());
                    self.last = quad.start;
                    self.state = FlattenState::Lines(it.collect(), 0)
                }
                PathItem::Cubic(cubic) => {
                    let it = CubicBezierSegment {
                        from: point_to_point2d(&cubic.start),
                        ctrl1: point_to_point2d(&cubic.control1),
                        ctrl2: point_to_point2d(&cubic.control2),
                        to: point_to_point2d(&cubic.end),
                    }
                    .flattened(self.tolerance)
                    .map(|x| x.to_tuple().into());
                    self.last = cubic.start;
                    self.state = FlattenState::Lines(it.collect(), 0);
                }
                PathItem::CloseAndJump | PathItem::Jump => {
                    self.next_state();
                }
            }
        } else {
            self.state = FlattenState::End;
        }
    }
}

impl<'a, I: Iterator<Item = &'a PathItem>> Iterator for Flatten<'a, I> {
    type Item = PathItem;

    fn next(&mut self) -> Option<Self::Item> {
        let mut state = FlattenState::End;
        std::mem::swap(&mut state, &mut self.state);
        match state {
            FlattenState::Line(p) => {
                let line = PathItem::Line(Line(self.last, p));
                self.next_state();
                Some(line)
            }
            FlattenState::Lines(ps, i) => {
                if ps.len() == i {
                    self.next_state();
                    self.next()
                } else {
                    let line = PathItem::Line(Line(self.last, ps[i]));
                    self.last = ps[i];
                    self.state = FlattenState::Lines(ps, i + 1);
                    Some(line)
                }
            }
            FlattenState::End => {
                None
            }
        }
    }
}

pub fn path_flatten(path: &Path, tolerance: f64) -> Path {
    let mut pis = Vec::new();
    for pi in path.0.iter() {
        match pi {
            PathItem::Line(line) => {
                pis.push(PathItem::Line(line.clone()));
            }
            PathItem::Arc(arc) => {
                let arc = Arc {
                    center: point_to_point2d(&arc.center),
                    radii: (arc.radius, arc.radius).into(),
                    start_angle: Angle::radians(arc.angle1),
                    sweep_angle: Angle::radians(arc.angle2 - arc.angle1),
                    x_rotation: Angle::radians(0.0),
                };
                let vec: Vec<_> = if arc.sweep_angle.radians.is_sign_positive() {
                    vec![arc.from()]
                        .into_iter()
                        .chain(arc.flattened(tolerance))
                        .map(|x| x.to_tuple().into())
                        .collect()
                } else {
                    let mut vec: Vec<_> = vec![arc.to()]
                        .into_iter()
                        .chain(arc.flip().flattened(tolerance))
                        .map(|x| x.to_tuple().into())
                        .collect();
                    vec.reverse();
                    vec
                };
                let mut p = vec[0];
                for q in vec {
                    if p != q {
                        pis.push(PathItem::Line(Line(p, q)));
                        p = q;
                    }
                }
            }
            PathItem::Ellipse(ellipse) => {
                let arc = Arc {
                    center: point_to_point2d(&ellipse.center),
                    radii: (ellipse.radius_x, ellipse.radius_y).into(),
                    start_angle: Angle::radians(ellipse.angle1),
                    sweep_angle: Angle::radians(ellipse.angle2 - ellipse.angle1),
                    x_rotation: Angle::radians(ellipse.rotation),
                };
                let vec: Vec<_> = if arc.sweep_angle.radians.is_sign_positive() {
                    vec![arc.from()]
                        .into_iter()
                        .chain(arc.flattened(tolerance))
                        .map(|x| x.to_tuple().into())
                        .collect()
                } else {
                    let mut vec: Vec<_> = vec![arc.to()]
                        .into_iter()
                        .chain(arc.flip().flattened(tolerance))
                        .map(|x| x.to_tuple().into())
                        .collect();
                    vec.reverse();
                    vec
                };
                let mut p = vec[0];
                for q in vec {
                    if p != q {
                        pis.push(PathItem::Line(Line(p, q)));
                        p = q;
                    }
                }
            }
            PathItem::Quad(quad) => {
                let it = QuadraticBezierSegment {
                    from: point_to_point2d(&quad.start),
                    ctrl: point_to_point2d(&quad.control1),
                    to: point_to_point2d(&quad.end),
                }
                .flattened(tolerance)
                .map(|x| x.to_tuple().into());
                let mut p = quad.start;
                for q in it {
                    if p != q {
                        pis.push(PathItem::Line(Line(p, q)));
                        p = q;
                    }
                }
            }
            PathItem::Cubic(cubic) => {
                let it = CubicBezierSegment {
                    from: point_to_point2d(&cubic.start),
                    ctrl1: point_to_point2d(&cubic.control1),
                    ctrl2: point_to_point2d(&cubic.control2),
                    to: point_to_point2d(&cubic.end),
                }
                .flattened(tolerance)
                .map(|x| x.to_tuple().into());
                let mut p = cubic.start;
                for q in it {
                    if p != q {
                        pis.push(PathItem::Line(Line(p, q)));
                        p = q;
                    }
                }
            }
            PathItem::CloseAndJump => {
                pis.push(PathItem::CloseAndJump);
            }
            PathItem::Jump => {
                pis.push(PathItem::Jump);
            }
        }
    }
    Path(pis)
}

pub fn path_flatten_only_cubic(path: &Path, tolerance: f64) -> Path {
    let mut pis = Vec::new();
    for pi in path.0.iter() {
        match pi {
            PathItem::Cubic(cubic) => {
                let it = CubicBezierSegment {
                    from: point_to_point2d(&cubic.start),
                    ctrl1: point_to_point2d(&cubic.control1),
                    ctrl2: point_to_point2d(&cubic.control2),
                    to: point_to_point2d(&cubic.end),
                }
                .flattened(tolerance)
                .map(|x| x.to_tuple().into());
                let mut p = cubic.start;
                for q in it {
                    if p != q {
                        pis.push(PathItem::Line(Line(p, q)));
                        p = q;
                    }
                }
            }
            _ => {
                pis.push(pi.clone());
            }
        }
    }
    Path(pis)
}

fn point_to_point2d(p: &Point) -> Point2D<f64> {
    Point2D::new(p.0, p.1)
}
