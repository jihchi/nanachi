use crate::point::Point;
use crate::models::{Line, Arc, Ellipse, Quad};
use crate::matrix::Matrix2d;
use crate::path3::{Path, PathItem};
use std::f64::consts::PI;

pub fn path_transform(path: &Path, am: &Matrix2d) -> Path {
    let mut pis = Vec::with_capacity(path.0.len());
    for pi in path.0.iter() {
        pis.push(match pi {
            PathItem::Line(line) => PathItem::Line(Line(am.apply(line.0), am.apply(line.1))),
            PathItem::Arc(arc) =>
                PathItem::Ellipse(transform_ellipse(&Ellipse {
                    center: arc.center,
                    radius_x: arc.radius,
                    radius_y: arc.radius,
                    rotation: 0.0,
                    angle1: arc.angle1,
                    angle2: arc.angle2,
                }, am)),
            PathItem::Ellipse(ellipse) =>
                PathItem::Ellipse(transform_ellipse(ellipse, am)),
            PathItem::Quad(quad) => PathItem::Quad(Quad {
                start: am.apply(quad.start),
                end: am.apply(quad.end),
                control1: am.apply(quad.control1),
            }),
        });
    }
    Path::new(pis)
}

pub fn transform_ellipse(ellipse: &Ellipse, am: &Matrix2d) -> Ellipse {
    let eam = Matrix2d::new()
        .scale(ellipse.radius_x, ellipse.radius_y)
        .rotate(ellipse.rotation)
        .translate(ellipse.center.0, ellipse.center.1);
    let am = eam.then(&am);
    let center = Point(am.0[2], am.0[5]);
    // dbg!(am);
    let k = (am.0[1].atan2(am.0[4]) + am.0[3].atan2(am.0[0])).tan();
    let w = Point(am.0[0], am.0[3]).rotate(am.0[1].atan2(am.0[4])).0;
    let h = am.0[1].hypot(am.0[4]);
    // dbg!(k, w, h);
    if !k.is_normal() || k == 0.0 {
        return Ellipse {
            center,
            radius_x: w,
            radius_y: h,
            rotation: am.0[3].atan2(am.0[0]),
            angle1: ellipse.angle1,
            angle2: ellipse.angle2,
        }
    }
    let rotation = 1.0 / 2.0 * (2.0 * k / (1.0 - k.powi(2) - (h / w).powi(2))).atan();
    let radius_x = w * (1.0 - k / rotation.tan()).sqrt();
    let radius_y = w * (1.0 + k * rotation.tan()).sqrt();
    // dbg!(rotation, radius_x, radius_y);
    let rotation = rotation + am.0[4].atan2(am.0[1]);
    let am2 = Matrix2d::new()
        .scale(radius_x, radius_y)
        .rotate(rotation)
        .translate(center.0, center.1)
        .inverse();
    let mut angle1 = am2.apply(am.apply(Point::from_angle(ellipse.angle1))).atan2();
    let mut angle2 = am2.apply(am.apply(Point::from_angle(ellipse.angle2))).atan2();
    if ellipse.angle1 < ellipse.angle2 && angle1 >= angle2 {
        angle2 += PI * 2.0;
    }
    if ellipse.angle1 > ellipse.angle2 && angle1 <= angle2 {
        angle1 += PI * 2.0;
    }
    Ellipse {
        center,
        radius_x,
        radius_y,
        rotation,
        angle1,
        angle2,
    }
}

#[test]
fn test() {
    let am = Matrix2d::new().scale(3.0, 2.0).skew_y(0.5).rotate(0.1);
    // let k = (am.0[1] + am.0[2]) / (am.0[4] + am.0[5]) + (am.0[3] + am.0[5]) / (am.0[0] + am.0[2]);
    let k = (PI / 2.0 - am.0[4].atan2(am.0[1]) + am.0[3].atan2(am.0[0])).tan();
    let p = am.apply(Point(1.0, 0.0)).rotate(am.0[1].atan2(am.0[4])).0;
    let q = am.0[1].hypot(am.0[4]);
    dbg!(k, p, q);

    let k = 0.5f64;
    let p = 3.0f64;
    let q = 2.0;
    let rotation = 1.0 / 2.0 * (2.0 * k / (1.0 - k.powi(2) - (q / p).powi(2))).atan();
    let scale_x = p * (1.0 + k * rotation.tan()).sqrt();
    let scale_y = p * (1.0 - k / rotation.tan()).sqrt();
    dbg!(rotation, scale_x, scale_y);
}