use super::*;
use image::{Rgb, Rgba};

macro_rules! def_linear_compositor {
    (
        $name:ident ($aa:ident, $ba:ident => $ca:ident, $ax:ident, $bx:ident)
        {$($rest1:tt)+} {$($rest2:tt)+}
    ) => {
        impl Compositor<Rgba<u8>> for $name {
            #[allow(unused_variables)]
            fn composite(&self, a: &Rgba<u8>, b: &Rgba<u8>, alpha: f64) -> Rgba<u8> {
                let $aa = a.0[3] as f64 / std::u8::MAX as f64;
                let $ba = b.0[3] as f64 / std::u8::MAX as f64 * alpha;
                $($rest1)+
                Rgba([
                    (a.0[0] as f64 * $ax + b.0[0] as f64 * $bx).round() as u8,
                    (a.0[1] as f64 * $ax + b.0[1] as f64 * $bx).round() as u8,
                    (a.0[2] as f64 * $ax + b.0[2] as f64 * $bx).round() as u8,
                    ($ca * std::u8::MAX as f64).round() as u8,
                ])
            }
        }

        impl Compositor<Rgb<u8>> for $name {
            #[allow(unused_variables)]
            fn composite(&self, a: &Rgb<u8>, b: &Rgb<u8>, alpha: f64) -> Rgb<u8> {
                let $aa = 1.0;
                let $ba = alpha;
                $($rest2)+
                Rgb([
                    (a.0[0] as f64 * $ax + b.0[0] as f64 * $bx).round() as u8,
                    (a.0[1] as f64 * $ax + b.0[1] as f64 * $bx).round() as u8,
                    (a.0[2] as f64 * $ax + b.0[2] as f64 * $bx).round() as u8,
                ])
            }
        }
    };
}

def_linear_compositor! {
    Clear(a, b => c, ax, bx) {
        let c = 0.0;
        let ax = 0.0;
        let bx = 0.0;
    } {
        let ax = 0.0;
        let bx = 0.0;
    }
}

def_linear_compositor! {
    Src(a, b => c, ax, bx) {
        let c = b;
        let ax = 0.0;
        let bx = b;
    } {
        let ax = 0.0;
        let bx = b;
    }
}

def_linear_compositor! {
    Dst(a, b => c, ax, bx) {
        let c = a;
        let ax = a;
        let bx = 0.0;
    } {
        let ax = a;
        let bx = 0.0;
    }
}

def_linear_compositor! {
    SrcOver(a, b => c, ax, bx) {
        let c = a + b - a * b;
        let ax = (a * (1.0 - b)) / c;
        let bx = b / c;
    } {
        let ax = a * (1.0 - b);
        let bx = b;
    }
}

def_linear_compositor! {
    SrcIn(a, b => c, ax, bx) {
        let c = a * b;
        let ax = 0.0;
        let bx = 1.0;
    } {
        let ax = 0.0;
        let bx = 1.0;
    }
}

def_linear_compositor! {
    SrcOut(a, b => c, ax, bx) {
        let c = (1.0 - a) * b;
        let ax = 0.0;
        let bx = 1.0;
    } {
        let ax = 0.0;
        let bx = 1.0;
    }
}

def_linear_compositor! {
    SrcAtop(a, b => c, ax, bx) {
        let c = b;
        let ax = 1.0 - b;
        let bx = b;
    } {
        let ax = 1.0 - b;
        let bx = b;
    }
}

def_linear_compositor! {
    DstOver(a, b => c, ax, bx) {
        let c = a + b - a * b;
        let ax = a / c;
        let bx = ((1.0 - a) * b) / c;
    } {
        let ax = a;
        let bx = (1.0 - a) * b;
    }
}

def_linear_compositor! {
    DstIn(a, b => c, ax, bx) {
        let c = a * b;
        let ax = 1.0;
        let bx = 0.0;
    } {
        let ax = 1.0;
        let bx = 0.0;
    }
}

def_linear_compositor! {
    DstOut(a, b => c, ax, bx) {
        let c = a * (1.0 - b);
        let ax = 1.0;
        let bx = 0.0;
    } {
        let ax = 1.0;
        let bx = 0.0;
    }
}

def_linear_compositor! {
    DstAtop(a, b => c, ax, bx) {
        let c = a;
        let ax = a;
        let bx = 1.0 - a;
    } {
        let ax = a;
        let bx = 1.0 - a;
    }
}

def_linear_compositor! {
    Xor(a, b => c, ax, bx) {
        let ax = a * (1.0 - b);
        let bx = (1.0 - a) * b;
        let c = a + b - 2.0 * a * b;
    } {
        let ax = a * (1.0 - b);
        let bx = (1.0 - a) * b;
    }
}

def_linear_compositor! {
    Add(a, b => c, ax, bx) {
        let c = (a + b).min(1.0);
        let ax = a / c;
        let bx = b / c;
    } {
        let ax = a;
        let bx = b;
    }
}

macro_rules! def_compositor {
    (
        $name:ident ($a:ident, $b:ident, $aa:ident, $ba:ident, $ca:ident)
        {$($rest1:tt)+} [$($rest2:expr,)+]
    ) => {
        impl Compositor<Rgba<u8>> for $name {
            #[allow(unused_variables)]
            fn composite(&self, $a: &Rgba<u8>, $b: &Rgba<u8>, alpha: f64) -> Rgba<u8> {
                let $aa = $a.0[3] as f64 / std::u8::MAX as f64;
                let $ba = $b.0[3] as f64 / std::u8::MAX as f64 * alpha;
                $($rest1)+
                Rgba([
                    $($rest2,)+
                    ($ca * std::u8::MAX as f64).round() as u8,
                ])
            }
        }

        impl Compositor<Rgb<u8>> for $name {
            #[allow(unused_variables)]
            fn composite(&self, $a: &Rgb<u8>, $b: &Rgb<u8>, alpha: f64) -> Rgb<u8> {
                let $aa = 1.0;
                let $ba = alpha;
                $($rest1)+
                Rgb([
                    $($rest2,)+
                ])
            }
        }
    };
}

def_compositor! {
    Darken(a, b, aa, ba, ca) {
        let ca = aa + ba - aa * ba;
        let ax = (aa * (1.0 - ba)) / ca;
        let bx = (ba * (1.0 - aa)) / ca;
        let cx = aa * ba / ca;
        let (a0, a1, a2, b0, b1, b2) = (a.0[0] as f64, a.0[1] as f64, a.0[2] as f64, b.0[0] as f64, b.0[1] as f64, b.0[2] as f64);
    } [
        (a0 * ax + b0 * bx + a.0[0].min(b.0[0]) as f64 * cx).round() as u8,
        (a1 * ax + b1 * bx + a.0[1].min(b.0[1]) as f64 * cx).round() as u8,
        (a2 * ax + b2 * bx + a.0[2].min(b.0[2]) as f64 * cx).round() as u8,
    ]
}

def_compositor! {
    Lighten(a, b, aa, ba, ca) {
        let ca = aa + ba - aa * ba;
        let ax = (aa * (1.0 - ba)) / ca;
        let bx = (ba * (1.0 - aa)) / ca;
        let cx = aa * ba / ca;
        let (a0, a1, a2, b0, b1, b2) = (a.0[0] as f64, a.0[1] as f64, a.0[2] as f64, b.0[0] as f64, b.0[1] as f64, b.0[2] as f64);
    } [
        (a0 * ax + b0 * bx + a.0[0].max(b.0[0]) as f64 * cx).round() as u8,
        (a1 * ax + b1 * bx + a.0[1].max(b.0[1]) as f64 * cx).round() as u8,
        (a2 * ax + b2 * bx + a.0[2].max(b.0[2]) as f64 * cx).round() as u8,
    ]
}

def_compositor! {
    Multiply(a, b, aa, ba, ca) {
        let ca = aa + ba - aa * ba;
        let ax = (aa * (1.0 - ba)) / ca;
        let bx = (ba * (1.0 - aa)) / ca;
        let cx = aa * ba / ca;
        let (a0, a1, a2, b0, b1, b2) = (a.0[0] as f64, a.0[1] as f64, a.0[2] as f64, b.0[0] as f64, b.0[1] as f64, b.0[2] as f64);
    } [
        (a0 * ax + b0 * bx + a0 * b0 / 255.0 * cx).round() as u8,
        (a1 * ax + b1 * bx + a1 * b1 / 255.0 * cx).round() as u8,
        (a2 * ax + b2 * bx + a2 * b2 / 255.0 * cx).round() as u8,
    ]
}

def_compositor! {
    Screen(a, b, aa, ba, ca) {
        let ca = aa + ba - aa * ba;
        let ax = (aa * (1.0 - ba)) / ca;
        let bx = (ba * (1.0 - aa)) / ca;
        let cx = aa * ba / ca;
        let (a0, a1, a2, b0, b1, b2) = (a.0[0] as f64, a.0[1] as f64, a.0[2] as f64, b.0[0] as f64, b.0[1] as f64, b.0[2] as f64);
    } [
        (a0 * ax + b0 * bx + (a0 + b0 - a0 * b0 / 255.0) * cx).round() as u8,
        (a1 * ax + b1 * bx + (a1 + b1 - a1 * b1 / 255.0) * cx).round() as u8,
        (a2 * ax + b2 * bx + (a2 + b2 - a2 * b2 / 255.0) * cx).round() as u8,
    ]
}

def_compositor! {
    Overlay(a, b, aa, ba, ca) {
        let ca = aa + ba - aa * ba;
        let ax = (aa * (1.0 - ba)) / ca;
        let bx = (ba * (1.0 - aa)) / ca;
        let cx = aa * ba / ca;
        let (a0, a1, a2, b0, b1, b2) = (a.0[0] as f64, a.0[1] as f64, a.0[2] as f64, b.0[0] as f64, b.0[1] as f64, b.0[2] as f64);
    } [
        (a0 * ax + b0 * bx + if a0 < 127.0 {2.0 * a0 * b0 / 255.0} else {255.0 - 2.0 * (255.0 - a0) * (255.0 - b0) / 255.0} * cx).round() as u8,
        (a1 * ax + b1 * bx + if a1 < 127.0 {2.0 * a1 * b1 / 255.0} else {255.0 - 2.0 * (255.0 - a1) * (255.0 - b1) / 255.0} * cx).round() as u8,
        (a2 * ax + b2 * bx + if a2 < 127.0 {2.0 * a2 * b2 / 255.0} else {255.0 - 2.0 * (255.0 - a2) * (255.0 - b2) / 255.0} * cx).round() as u8,
    ]
}

def_compositor! {
    HardLight(a, b, aa, ba, ca) {
        let ca = aa + ba - aa * ba;
        let ax = (aa * (1.0 - ba)) / ca;
        let bx = (ba * (1.0 - aa)) / ca;
        let cx = aa * ba / ca;
        let (a0, a1, a2, b0, b1, b2) = (a.0[0] as f64, a.0[1] as f64, a.0[2] as f64, b.0[0] as f64, b.0[1] as f64, b.0[2] as f64);
    } [
        (a0 * ax + b0 * bx + if b0 < 127.0 {2.0 * a0 * b0 / 255.0} else {255.0 - 2.0 * (255.0 - a0) * (255.0 - b0) / 255.0} * cx).round() as u8,
        (a1 * ax + b1 * bx + if b1 < 127.0 {2.0 * a1 * b1 / 255.0} else {255.0 - 2.0 * (255.0 - a1) * (255.0 - b1) / 255.0} * cx).round() as u8,
        (a2 * ax + b2 * bx + if b2 < 127.0 {2.0 * a2 * b2 / 255.0} else {255.0 - 2.0 * (255.0 - a2) * (255.0 - b2) / 255.0} * cx).round() as u8,
    ]
}

def_compositor! {
    Dodge(a, b, aa, ba, ca) {
        let ca = aa + ba - aa * ba;
        let ax = (aa * (1.0 - ba)) / ca;
        let bx = (ba * (1.0 - aa)) / ca;
        let cx = aa * ba / ca;
        let (a0, a1, a2, b0, b1, b2) = (a.0[0] as f64, a.0[1] as f64, a.0[2] as f64, b.0[0] as f64, b.0[1] as f64, b.0[2] as f64);
    } [
        (a0 * ax + b0 * bx + if b0 < 255.0 {(a0 / (255.0 - b0)).min(1.0) * 255.0} else {255.0} * cx).round() as u8,
        (a1 * ax + b1 * bx + if b1 < 255.0 {(a1 / (255.0 - b1)).min(1.0) * 255.0} else {255.0} * cx).round() as u8,
        (a2 * ax + b2 * bx + if b2 < 255.0 {(a2 / (255.0 - b2)).min(1.0) * 255.0} else {255.0} * cx).round() as u8,
    ]
}

def_compositor! {
    Burn(a, b, aa, ba, ca) {
        let ca = aa + ba - aa * ba;
        let ax = (aa * (1.0 - ba)) / ca;
        let bx = (ba * (1.0 - aa)) / ca;
        let cx = aa * ba / ca;
        let (a0, a1, a2, b0, b1, b2) = (a.0[0] as f64, a.0[1] as f64, a.0[2] as f64, b.0[0] as f64, b.0[1] as f64, b.0[2] as f64);
    } [
        (a0 * ax + b0 * bx + if 0.0 < b0 {255.0 - ((255.0 - a0) / b0).min(1.0) * 255.0} else {0.0} * cx).round() as u8,
        (a1 * ax + b1 * bx + if 0.0 < b1 {255.0 - ((255.0 - a1) / b1).min(1.0) * 255.0} else {0.0} * cx).round() as u8,
        (a2 * ax + b2 * bx + if 0.0 < b2 {255.0 - ((255.0 - a2) / b2).min(1.0) * 255.0} else {0.0} * cx).round() as u8,
    ]
}


def_compositor! {
    SoftLight(a, b, aa, ba, ca) {
        let ca = aa + ba - aa * ba;
        let ax = (aa * (1.0 - ba)) / ca;
        let bx = (ba * (1.0 - aa)) / ca;
        let cx = aa * ba / ca;
        let (a0, a1, a2, b0, b1, b2) = (a.0[0] as f64, a.0[1] as f64, a.0[2] as f64, b.0[0] as f64, b.0[1] as f64, b.0[2] as f64);
        fn f(a: f64, b: f64) -> f64 {
            let a = a / 255.0;
            let b = b / 255.0;
            255.0 * if 0.5 < b {
                a - (1.0 - 2.0 * b) * a * (1.0 - a)
            } else {
                let g = if a < 0.25 {
                    ((16.0 * a - 12.0) * a + 4.0) * a
                } else {
                    a.sqrt()
                };
                a + (2.0 * b - 1.0) * (g - a)
            }
        }
    } [
        (a0 * ax + b0 * bx + f(a0, b0) * cx).round() as u8,
        (a1 * ax + b1 * bx + f(a1, b1) * cx).round() as u8,
        (a2 * ax + b2 * bx + f(a2, b2) * cx).round() as u8,
    ]
}

def_compositor! {
    Difference(a, b, aa, ba, ca) {
        let ca = aa + ba - aa * ba;
        let ax = (aa * (1.0 - ba)) / ca;
        let bx = (ba * (1.0 - aa)) / ca;
        let cx = aa * ba / ca;
        let (a0, a1, a2, b0, b1, b2) = (a.0[0] as f64, a.0[1] as f64, a.0[2] as f64, b.0[0] as f64, b.0[1] as f64, b.0[2] as f64);
    } [
        (a0 * ax + b0 * bx + (a0 - b0).abs() * cx).round() as u8,
        (a1 * ax + b1 * bx + (a1 - b1).abs() * cx).round() as u8,
        (a2 * ax + b2 * bx + (a2 - b2).abs() * cx).round() as u8,
    ]
}

def_compositor! {
    Exclusion(a, b, aa, ba, ca) {
        let ca = aa + ba - aa * ba;
        let ax = (aa * (1.0 - ba)) / ca;
        let bx = (ba * (1.0 - aa)) / ca;
        let cx = aa * ba / ca;
        let (a0, a1, a2, b0, b1, b2) = (a.0[0] as f64, a.0[1] as f64, a.0[2] as f64, b.0[0] as f64, b.0[1] as f64, b.0[2] as f64);
    } [
        (a0 * ax + b0 * bx + (a0 + b0 - 2.0 * a0 * b0 / 255.0) * cx).round() as u8,
        (a1 * ax + b1 * bx + (a1 + b1 - 2.0 * a1 * b1 / 255.0) * cx).round() as u8,
        (a2 * ax + b2 * bx + (a2 + b2 - 2.0 * a2 * b2 / 255.0) * cx).round() as u8,
    ]
}

impl Compositor<Rgba<u8>> for Basic {
    fn composite(&self, a: &Rgba<u8>, b: &Rgba<u8>, alpha: f64) -> Rgba<u8> {
        use Basic::*;
        match self {
            Clear => Clear.composite(a, b, alpha),
            Src => Src.composite(a, b, alpha),
            Dst => Dst.composite(a, b, alpha),
            SrcOver => SrcOver.composite(a, b, alpha),
            SrcIn => SrcIn.composite(a, b, alpha),
            SrcOut => SrcOut.composite(a, b, alpha),
            SrcAtop => SrcAtop.composite(a, b, alpha),
            DstOver => DstOver.composite(a, b, alpha),
            DstIn => DstIn.composite(a, b, alpha),
            DstOut => DstOut.composite(a, b, alpha),
            DstAtop => DstAtop.composite(a, b, alpha),
            Xor => Xor.composite(a, b, alpha),
            Add => Add.composite(a, b, alpha),
            Darken => Darken.composite(a, b, alpha),
            Lighten => Lighten.composite(a, b, alpha),
            Multiply => Multiply.composite(a, b, alpha),
            Screen => Screen.composite(a, b, alpha),
            Overlay => Overlay.composite(a, b, alpha),
            HardLight => HardLight.composite(a, b, alpha),
            Dodge => Dodge.composite(a, b, alpha),
            Burn => Burn.composite(a, b, alpha),
            SoftLight => SoftLight.composite(a, b, alpha),
            Difference => Difference.composite(a, b, alpha),
            Exclusion => Exclusion.composite(a, b, alpha),
        }
    }
}

impl Compositor<Rgb<u8>> for Basic {
    fn composite(&self, a: &Rgb<u8>, b: &Rgb<u8>, alpha: f64) -> Rgb<u8> {
        use Basic::*;
        match self {
            Clear => Clear.composite(a, b, alpha),
            Src => Src.composite(a, b, alpha),
            Dst => Dst.composite(a, b, alpha),
            SrcOver => SrcOver.composite(a, b, alpha),
            SrcIn => SrcIn.composite(a, b, alpha),
            SrcOut => SrcOut.composite(a, b, alpha),
            SrcAtop => SrcAtop.composite(a, b, alpha),
            DstOver => DstOver.composite(a, b, alpha),
            DstIn => DstIn.composite(a, b, alpha),
            DstOut => DstOut.composite(a, b, alpha),
            DstAtop => DstAtop.composite(a, b, alpha),
            Xor => Xor.composite(a, b, alpha),
            Add => Add.composite(a, b, alpha),
            Darken => Darken.composite(a, b, alpha),
            Lighten => Lighten.composite(a, b, alpha),
            Multiply => Multiply.composite(a, b, alpha),
            Screen => Screen.composite(a, b, alpha),
            Overlay => Overlay.composite(a, b, alpha),
            HardLight => HardLight.composite(a, b, alpha),
            Dodge => Dodge.composite(a, b, alpha),
            Burn => Burn.composite(a, b, alpha),
            SoftLight => SoftLight.composite(a, b, alpha),
            Difference => Difference.composite(a, b, alpha),
            Exclusion => Exclusion.composite(a, b, alpha),
        }
    }
}
