use nanachi::{
    affine::AugmentedMatrix,
    bezier::{Bezier2, Bezier3},
    draw, geometry,
    image::{ImageBuffer, Luma, Rgb},
    k_curve,
    path::Path,
    point::Point,
    position_color, primitives,
};
use std::f64::consts::PI;

fn main() {
    let (width, height) = (512, 512);
    let mut img = ImageBuffer::from_pixel(width, height, Rgb([250u8, 250, 250]));

    draw::draw_hori_with_antialias(
        &mut img,
        (256, 256 + 200),
        PI * 0.1,
        &position_color::Constant::new(Rgb([200, 250, 250])),
    );
    draw::draw_hori_with_antialias(
        &mut img,
        (256 - 200, 256),
        PI * 0.6,
        &position_color::Constant::new(Rgb([200, 200, 250])),
    );
    draw::draw_hori_with_antialias(
        &mut img,
        (256, 256 - 200),
        PI * 1.1,
        &position_color::Constant::new(Rgb([250, 200, 250])),
    );
    draw::draw_hori_with_antialias(
        &mut img,
        (256 + 200, 256),
        PI * 1.6,
        &position_color::Constant::new(Rgb([250, 200, 200])),
    );

    draw::draw_line(&mut img, (10, 10), (100, 100), Rgb([250, 10, 10]));
    let mut path: Path = vec![
        (20f64, 20f64),
        (20., 50.),
        (50., 50.),
        (50., 20.),
        (20., 20.),
    ]
    .into_iter()
    .map(|x| x.into())
    .collect::<Vec<Point>>()
    .into();
    path.transform_mut(&AugmentedMatrix::new().rotate(-0.1));
    draw::draw_path(
        &mut img,
        &path.into() as &Vec<Point>,
        Rgb([10, 10, 250]),
        1.0,
    );

    let mut path = primitives::triangle(100.);
    path.transform_mut(&AugmentedMatrix::new().rotate(0.0).translate(100., 100.));
    let mut path2 = primitives::triangle(100.);
    path2.transform_mut(
        &AugmentedMatrix::new()
            .scale(0.5, 0.5)
            .rotate(0.0)
            .translate(100., 100.),
    );
    let pc = position_color::BlockCheck::new(Rgb([200, 200, 200]), Rgb([100, 100, 100]), 10.0);
    draw::draw_fill(
        &mut img,
        &vec![
            &path.clone().into() as &Vec<Point>,
            &path2.clone().into() as &Vec<Point>,
        ],
        &pc,
    );
    draw::draw_path(
        &mut img,
        &path.into() as &Vec<Point>,
        Rgb([10, 250, 10]),
        2.0,
    );

    let mut path = primitives::triangle(100.);
    path.transform_mut(&AugmentedMatrix::new().rotate(0.2).translate(100., 100.));
    draw::draw_path(
        &mut img,
        &path.into() as &Vec<Point>,
        Rgb([10, 250, 10]),
        2.0,
    );

    let mut path = primitives::triangle(100.);
    path.transform_mut(&AugmentedMatrix::new().rotate(0.4).translate(100., 100.));
    draw::draw_path(
        &mut img,
        &path.into() as &Vec<Point>,
        Rgb([10, 250, 10]),
        2.0,
    );

    let time = std::time::Instant::now();
    draw::draw_path(
        &mut img,
        &k_curve::k_curve(
            vec![
                Point(0.2, 0.2),
                Point(0.8, 0.2),
                Point(0.8, 0.8),
                Point(0.2, 0.8),
            ],
            true,
            0,
        )
        .as_lines_points(8)
        .iter()
        .map(|x| (x.0 * width as f64, x.1 * height as f64))
        .collect::<Vec<_>>(),
        Rgb([200, 0, 0]),
        2.0,
    );
    draw::draw_path(
        &mut img,
        &k_curve::k_curve(
            vec![
                Point(0.2, 0.2),
                Point(0.8, 0.2),
                Point(0.8, 0.8),
                Point(0.2, 0.8),
            ],
            true,
            1,
        )
        .as_lines_points(8)
        .iter()
        .map(|x| (x.0 * width as f64, x.1 * height as f64))
        .collect::<Vec<_>>(),
        Rgb([0, 200, 0]),
        2.0,
    );
    draw::draw_path(
        &mut img,
        &k_curve::k_curve(
            vec![
                Point(0.2, 0.2),
                Point(0.8, 0.2),
                Point(0.8, 0.8),
                Point(0.2, 0.8),
            ],
            true,
            2,
        )
        .as_lines_points(8)
        .iter()
        .map(|x| (x.0 * width as f64, x.1 * height as f64))
        .collect::<Vec<_>>(),
        Rgb([200, 200, 0]),
        2.0,
    );
    println!("{:?}", time.elapsed());

    let time = std::time::Instant::now();
    nanachi::draw_path::draw_line(&mut img, (100, 100), (50, 100), Rgb([20, 20, 200]));
    nanachi::draw_path::draw_line(&mut img, (100, 100), (50, 50), Rgb([20, 20, 200]));
    nanachi::draw_path::draw_line(&mut img, (100, 100), (100, 50), Rgb([20, 20, 200]));
    nanachi::draw_path::draw_line(&mut img, (100, 100), (150, 50), Rgb([20, 20, 200]));
    nanachi::draw_path::draw_line(&mut img, (100, 100), (150, 100), Rgb([20, 20, 200]));
    nanachi::draw_path::draw_line(&mut img, (100, 100), (150, 150), Rgb([20, 20, 200]));
    nanachi::draw_path::draw_line(&mut img, (100, 100), (100, 150), Rgb([20, 20, 200]));
    nanachi::draw_path::draw_line(&mut img, (100, 100), (50, 150), Rgb([20, 20, 200]));
    nanachi::draw_path::draw_path(
        &mut img,
        &[
            (100, 100),
            (100, 400),
            (300, 450),
            (450, 300),
            (400, 100),
            (100, 100),
        ],
        Rgb([20, 200, 200]),
    );
    println!("{:?}", time.elapsed());

    {
        let mut b = ImageBuffer::from_pixel(img.width(), img.height(), Luma([0u8]));
        nanachi::draw_path::draw_arc(
            &mut b,
            Point(300.0, 200.0),
            90.0,
            0.0,
            std::f64::consts::PI * 2.0,
        );
        nanachi::draw_path::draw_arc(
            &mut b,
            Point(300.0, 200.0),
            50.0,
            0.0,
            std::f64::consts::PI * 2.0,
        );
        nanachi::draw_path::draw_arc(&mut b, Point(300.0, 200.0), 45.0, 1.0, 3.0);
        nanachi::draw_path::draw_arc(
            &mut b,
            Point(300.0, 200.0),
            10.0,
            0.0,
            std::f64::consts::PI * 2.0,
        );
        nanachi::draw_path::copy_within(&mut img, &b, Rgb([200, 200, 20]));
    }
    {
        use nanachi::path2::{Path, PathAnchor};
        let path = Path::new(
            vec![
                PathAnchor::Point(Point(100.0, 200.0)),
                PathAnchor::Point(Point(150.0, 150.0)),
                PathAnchor::Point(Point(200.0, 200.0)),
                PathAnchor::Point(Point(300.0, 250.0)),
                PathAnchor::Arc {
                    center: Point(200.0, 300.0),
                    radius: 50.0,
                    angle1: 0.0,
                    angle2: 3.14,
                },
            ],
            false,
        );
        nanachi::draw_path::draw_path2(&mut img, &path, Rgb([200, 100, 0]));
        let path2 = Path::new(
            vec![
                PathAnchor::Point(Point(105.0, 205.0)),
                PathAnchor::Point(Point(205.0, 205.0)),
                PathAnchor::Arc {
                    center: Point(205.0, 305.0),
                    radius: 50.0,
                    angle1: 0.0,
                    angle2: 3.14,
                },
            ],
            true,
        );
        let edges = path2.edges();
        nanachi::draw_path::draw_path_edge(&mut img, &edges, Rgb([150, 150, 0]));

        let paths = path.edge_path(10.0);
        println!("{:?}", paths);
        println!(
            "!!!{:?}",
            paths.iter().flat_map(|p| p.edges()).collect::<Vec<_>>()
        );
        nanachi::fill_path2::draw_fill(
            &mut img,
            &paths.iter().flat_map(|p| p.edges()).collect(),
            &position_color::Constant::new(Rgb([200, 100, 250])),
        );
        // for path in paths {
        //     nanachi::draw_path::draw_path2(&mut img, &path, Rgb([100, 0, 200]));
        // }
    }
    {
        let t = std::time::Instant::now();
        use nanachi::path2::{Path, PathAnchor};
        let path = Path::new(
            vec![
                PathAnchor::Point(Point(100.0, 450.0)),
                PathAnchor::Point(Point(140.0, 460.0)),
                PathAnchor::Point(Point(160.0, 460.0)),
                PathAnchor::Point(Point(200.0, 450.0)),
                PathAnchor::Point(Point(210.0, 410.0)),
                PathAnchor::Point(Point(210.0, 390.0)),
                PathAnchor::Point(Point(200.0, 350.0)),
                PathAnchor::Point(Point(160.0, 340.0)),
                PathAnchor::Point(Point(140.0, 340.0)),
                PathAnchor::Point(Point(100.0, 350.0)),
                PathAnchor::Point(Point(90.0, 390.0)),
                PathAnchor::Point(Point(90.0, 410.0)),
            ],
            true,
        );

        nanachi::fill_path2::draw_fill(
            &mut img,
            &path.edges(),
            &position_color::Constant::new(Rgb([200, 100, 20])),
        );

        let path = Path::new(
            vec![
                PathAnchor::Point(Point(300.5, 350.5)),
                PathAnchor::Point(Point(350.5, 351.5)),
                PathAnchor::Point(Point(349.5, 400.5)),
                PathAnchor::Point(Point(299.5, 399.5)),
            ]
            .into_iter()
            .rev()
            .collect(),
            true,
        );

        nanachi::fill_path2::draw_fill(
            &mut img,
            &path.edges(),
            &position_color::Constant::new(Rgb([100, 200, 20])),
        );
        println!("{:?}", t.elapsed());
    }

    {
        use nanachi::path2::{Path, PathAnchor};
        let path = Path::new(
            vec![
                PathAnchor::Arc {
                    // 方向(法線)に注意
                    center: Point(430.0, 430.0),
                    radius: 50.0,
                    angle1: PI * 0.1,
                    angle2: PI * 1.6,
                }, //.flip(),
            ],
            true,
        );
        let mut e = path.edges();
        println!("{:?}", path.edges());
        //e.remove(0);
        nanachi::fill_path2::draw_fill(
            &mut img,
            &e,
            &position_color::Constant::new(Rgb([200, 100, 250])),
        );
    }

    let res = img.save("./symbols.png");
    println!("save: {:?}", res);
}
