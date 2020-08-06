use crate::draw::blend_rgb;
use crate::geometry::{
    intersect_line_and_horizon, intersect_line_and_vertical, intersect_segment_and_horizon,
    intersect_segment_and_vertical,
};
use crate::point::Point;
use image::{ImageBuffer, Luma, Rgb};

pub fn draw_line<P: Into<Point> + Copy>(
    buf: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    p1: P,
    p2: P,
    pixel: Rgb<u8>,
) {
    let p1: Point = p1.into();
    let p2: Point = p2.into();
    let mut b = ImageBuffer::from_pixel(buf.width(), buf.height(), Luma([0u8]));
    draw_line_(&mut b, p1, p2);
    copy_within(buf, &b, pixel);
}

pub fn draw_arc(buf: &mut ImageBuffer<Luma<u8>, Vec<u8>>, c: Point, r: f64, a1: f64, a2: f64) {
    // TODO: normalize a1, a2

    let da = 1.0 / r; // TODO
    let mut a = a1;
    while a <= a2 {
        let (sin, cos) = a.sin_cos();
        safe_put_pixel(buf, (c.0 + cos * r) as i32, (c.1 - sin * r) as i32, 255);
        a += da;
    }
}

fn draw_line_(buf: &mut ImageBuffer<Luma<u8>, Vec<u8>>, p1: Point, p2: Point) {
    let (p1, p2) = if p1.0 > p2.0 || (p1.0 == p2.0 && p1.1 > p2.1) {
        (p2, p1)
    } else {
        (p1, p2)
    };

    let mut p = (p1.0 as i32, p1.1 as i32);
    let end = (p2.0 as i32, p2.1 as i32);
    safe_put_pixel(buf, p.0, p.1, 255);
    match dir(p1, p2) {
        Direction::Bottom => {
            for y in p1.1 as i32..p2.1 as i32 {
                safe_put_pixel(buf, p.0, y, 255);
            }
        }
        Direction::Right => {
            for x in p1.0 as i32..p2.0 as i32 {
                safe_put_pixel(buf, x, p.1, 255);
            }
        }
        Direction::TopRight => {
            let mut dx = intersect_line_and_horizon(p1, p2, p.1 as f64);
            let mut dy = intersect_line_and_vertical(p1, p2, (p.0 + 1) as f64);
            while p != end {
                if (dx - p.0 as f64) < (p.1 as f64 - dy) {
                    p.1 -= 1;
                    dx = intersect_line_and_horizon(p1, p2, p.1 as f64);
                } else {
                    p.0 += 1;
                    dy = intersect_line_and_vertical(p1, p2, (p.0 + 1) as f64);
                }
                safe_put_pixel(buf, p.0, p.1, 255);
            }
        }
        Direction::BottomRight => {
            let mut dx = intersect_line_and_horizon(p1, p2, (p.1 + 1) as f64);
            let mut dy = intersect_line_and_vertical(p1, p2, (p.0 + 1) as f64);
            while p != end {
                if (dx - p.0 as f64) < (dy - p.1 as f64) {
                    p.1 += 1;
                    dx = intersect_line_and_horizon(p1, p2, (p.1 + 1) as f64);
                } else {
                    p.0 += 1;
                    dy = intersect_line_and_vertical(p1, p2, (p.0 + 1) as f64);
                }
                safe_put_pixel(buf, p.0, p.1, 255);
            }
        }
        Direction::No => {}
        Direction::Top | Direction::Left | Direction::TopLeft | Direction::BottomLeft => {
            unreachable!()
        }
    }
}

fn safe_put_pixel(buf: &mut ImageBuffer<Luma<u8>, Vec<u8>>, x: i32, y: i32, value: u8) {
    if 0 <= x && x < buf.width() as i32 && 0 <= y && y < buf.height() as i32 {
        buf.put_pixel(x as u32, y as u32, Luma([value]));
    }
}

pub fn draw_path<P: Into<Point> + Copy>(
    buf: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    ps: &[P],
    pixel: Rgb<u8>,
) {
    assert!(!ps.is_empty());
    let mut b = ImageBuffer::from_pixel(buf.width(), buf.height(), Luma([0u8]));

    for pair in ps.windows(2) {
        draw_line_(&mut b, pair[0].into(), pair[1].into());
    }

    copy_within(buf, &b, pixel);
}

pub fn copy_within(
    buf: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    src: &ImageBuffer<Luma<u8>, Vec<u8>>,
    pixel: Rgb<u8>,
) {
    for (p1, p2) in buf.pixels_mut().zip(src.pixels()) {
        if p2.0[0] != 0 {
            *p1 = blend_rgb(*p1, pixel, p2.0[0] as f64 / std::u8::MAX as f64);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Top,
    Bottom,
    Left,
    Right,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    No,
}

pub fn dir(p1: Point, p2: Point) -> Direction {
    use Direction::*;
    if p1.0 == p2.0 {
        if p1.1 == p2.1 {
            No
        } else if p1.1 < p2.1 {
            Bottom
        } else {
            Top
        }
    } else if p1.0 < p2.0 {
        if p1.1 == p2.1 {
            Right
        } else if p1.1 < p2.1 {
            BottomRight
        } else {
            TopRight
        }
    } else {
        if p1.1 == p2.1 {
            Left
        } else if p1.1 < p2.1 {
            BottomLeft
        } else {
            TopLeft
        }
    }
}
