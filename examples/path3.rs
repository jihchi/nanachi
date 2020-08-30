use nanachi::{
    image::{ImageBuffer, Rgb},
    path3::Path,
    path_builder::PathBuilder,
    point::Point,
    position_color,
    path_transform::path_transform,
    affine::AugmentedMatrix,
};
use std::f64::consts::PI;

fn main() {
    let (width, height) = (512, 512);
    let mut img = ImageBuffer::from_pixel(width, height, Rgb([250u8, 250, 250]));

    let path = PathBuilder::new()
    .move_to(Point(10.0, 10.0))
    .line_to(Point(300.0, 10.0))
    // .quad(Point(500.0, 300.0), Point(300.0, 300.0))
    // .quad(Point(300.0, 200.0), Point(300.0, 300.0))
    .quad(Point(700.0, 500.0), Point(300.0, 300.0))
    .arc(Point(200.0, 340.0), 100.0, 0.0, 3.14)
    .arc(Point(200.0, 340.0), 50.0, PI * 3.0, PI * 1.2)
    .line_to(Point(50.0, 250.0))
    .ellipse(Point(100.0, 150.0), 80.0, 50.0, 1.0, PI * 2.7, PI * 1.0)
    .close()
    .end();
    // let path = PathBuilder::new()
    //     .move_to(Point(100.0, 100.0))
    //     .line_to(Point(200.0, 100.0))
    //     .line_to(Point(200.0, 200.0))
    //     .line_to(Point(100.0, 200.0))
    //     .close().end();
    let am = AugmentedMatrix::new()
        .translate(-250.0, -250.0)
        .rotate(0.9)
        .scale(1.0, 0.6)
        .skew_x(-0.1)
        .translate(250.0, 250.0)
    ;
    let path = path_transform(&path, &am);
    {
        let pc = position_color::Constant::new(Rgb([250, 100, 100]));
        draw_fill(&mut img, &path, &pc, 1.0);
    }
    {
        let path = Path::new(nanachi::bold::path_bold1(&path, 1.0));
        let pc = position_color::Constant::new(Rgb([100, 100, 250]));
        draw_fill(&mut img, &path, &pc, 0.9);
    }

    let res = img.save("./path3.png");
    println!("save: {:?}", res);
}

fn draw_fill<X, C: nanachi::position_color::PositionColor<X>>(
    img: &mut ImageBuffer<X, Vec<u8>>,
    path: &Path,
    position_color: &C,
    alpha: f64,
) where
    X: image::Pixel<Subpixel = u8> + 'static,
{
    nanachi::fill_path2::draw_fill(
        img.width() as u32,
        img.height() as u32,
        path,
        &mut nanachi::writer::alpha_blend2(img, position_color, nanachi::writer::FillRule::NonZero, alpha),
    );
}
