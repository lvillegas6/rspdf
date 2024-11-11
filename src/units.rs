use std::num::FpCategory;
use std::ops::{Add, Div, Mul, Sub, AddAssign, DivAssign, MulAssign, SubAssign};
#[derive(Debug, Copy, Clone)]
pub struct Pt(pub f32);

#[derive(Debug, Copy, Clone)]
pub struct Px(pub f32);

impl Pt {
    pub fn into_px(self, dpi: f32) -> Px {
        Px(self.0 * dpi / 72.0)
    }
}

impl From<Px> for Pt {
    fn from(px: Px) -> Pt {
        Pt(px.0 * 0.74999943307122)
    }
}

impl Px {
    pub fn into_pt(self, dpi: f32) -> Pt {
        Pt(self.0 * 72.0 / dpi)
    }
}

impl From<Pt> for Px {
    fn from(pt: Pt) -> Px {
        Px(pt.0 * 1.3333343412075)
    }
}


macro_rules! impl_math_ops {
    ($type:ident) => {

        impl Add for $type {
            type Output = Self;
            fn add(self, other: Self) -> Self {
                Self(self.0 + other.0)
            }
        }

        impl Sub for $type {
            type Output = Self;
            fn sub(self, other: Self) -> Self {
                Self(self.0 - other.0)
            }
        }

        impl Div for $type {
            type Output = Self;
            fn div(self, other: Self) -> Self {
                Self(self.0 / other.0)
            }
        }

        impl Div<f32> for $type {
            type Output = Self;
            fn div(self, other: f32) -> Self {
                Self(self.0 / other)
            }
        }

        impl Mul for $type {
            type Output = Self;
            fn mul(self, other: Self) -> Self {
                Self(self.0 * other.0)
            }
        }

        impl Mul<f32> for $type {
            type Output = Self;
            fn mul(self, other: f32) -> Self {
                Self(self.0 * other)
            }
        }


        impl AddAssign for $type {
            fn add_assign(&mut self, other: Self) {
                self.0 += other.0;
            }
        }

        impl SubAssign for $type {
            fn sub_assign(&mut self, other: Self) {
                self.0 -= other.0;
            }
        }

        impl DivAssign for $type {
            fn div_assign(&mut self, other: Self) {
                self.0 /= other.0;
            }
        }

        impl DivAssign<f32> for $type {
            fn div_assign(&mut self, other: f32) {
                self.0 /= other;
            }
        }


        impl MulAssign for $type {
            fn mul_assign(&mut self, other: Self) {
                self.0 *= other.0;
            }
        }

        impl MulAssign<f32> for $type {
            fn mul_assign(&mut self, other: f32) {
                self.0 *= other;
            }
        }

        impl PartialEq for $type {
            fn eq(&self, other: &Self) -> bool {
                 if (self.0.classify() == FpCategory::Zero
                    || self.0.classify() == FpCategory::Normal)
                    && (other.0.classify() == FpCategory::Zero
                        || other.0.classify() == FpCategory::Normal)
                 {
                     return (self.0 * 1000.0).round() == (other.0 * 1000.0).round()
                 }

                false
            }
        }
    };
}

impl_math_ops!(Pt);
impl_math_ops!(Px);

#[cfg(test)]
mod tests {
    use super::{Px, Pt};
    #[test]
    fn it_convert_point_to_pixel() {
        let pt = Pt(10.0);
        let px: Px = pt.into();
        assert_eq!(px, Px(13.333343));
    }

    #[test]
    fn it_convert_pixel_to_point() {
        let px = Px(10.0);
        let pt: Pt = px.into();
        assert_eq!(pt, Pt(7.499994));
    }

    #[test]
    fn it_add_points() {
        let pt1 = Pt(10.0);
        let pt2 = Pt(20.0);
        let pt3 = pt1 + pt2;
        assert_eq!(pt3, Pt(30.0));
    }

    #[test]
    fn it_add_points_assign() {
        let mut pt1 = Pt(10.0);
        let pt2 = Pt(20.0);
        pt1 += pt2;
        assert_eq!(pt1, Pt(30.0));
    }

    #[test]
    fn it_sub_points() {
        let pt1 = Pt(10.0);
        let pt2 = Pt(20.0);
        let pt3 = pt1 - pt2;
        assert_eq!(pt3, Pt(-10.0));
    }

    #[test]
    fn it_sub_points_assign() {
        let mut pt1 = Pt(10.0);
        let pt2 = Pt(20.0);
        pt1 -= pt2;
        assert_eq!(pt1, Pt(-10.0));
    }

    #[test]
    fn it_mul_points() {
        let pt1 = Pt(10.0);
        let pt2 = Pt(20.0);
        let pt3 = pt1 * pt2;
        assert_eq!(pt3, Pt(200.0));
    }

    #[test]
    fn it_mul_points_assign() {
        let mut pt1 = Pt(10.0);
        let pt2 = Pt(20.0);
        pt1 *= pt2;
        assert_eq!(pt1, Pt(200.0));
    }

    #[test]
    fn it_mul_points_assign_f32() {
        let mut pt1 = Pt(10.0);
        let pt2 = 20.0;
        pt1 *= pt2;
        assert_eq!(pt1, Pt(200.0));
    }


    #[test]
    fn it_div_points() {
        let pt1 = Pt(10.0);
        let pt2 = Pt(20.0);
        let pt3 = pt1 / pt2;
        assert_eq!(pt3, Pt(0.5));
    }

    #[test]
    fn it_div_points_assign() {
        let mut pt1 = Pt(10.0);
        let pt2 = Pt(20.0);
        pt1 /= pt2;
        assert_eq!(pt1, Pt(0.5));
    }

    #[test]
    fn it_div_points_assign_f32() {
        let mut pt1 = Pt(10.0);
        let pt2 = 20.0;
        pt1 /= pt2;
        assert_eq!(pt1, Pt(0.5));
    }

    #[test]
    fn it_eq_points() {
        let pt1 = Pt(10.0);
        let pt2 = Pt(10.0);
        assert_eq!(pt1, pt2);
    }

    #[test]
    fn it_eq_points_with_rounding() {
        let pt1 = Pt(10.0001);
        let pt2 = Pt(10.0002);
        assert_eq!(pt1, pt2);
    }

    #[test]
    fn it_add_pixels() {
        let px1 = Px(10.0);
        let px2 = Px(20.0);
        let px3 = px1 + px2;
        assert_eq!(px3, Px(30.0));
    }

    #[test]
    fn it_add_pixels_assign() {
        let mut px1 = Px(10.0);
        let px2 = Px(20.0);
        px1 += px2;
        assert_eq!(px1, Px(30.0));
    }

    #[test]
    fn it_sub_pixels() {
        let px1 = Px(10.0);
        let px2 = Px(20.0);
        let px3 = px1 - px2;
        assert_eq!(px3, Px(-10.0));
    }

    #[test]
    fn it_sub_pixels_assign() {
        let mut px1 = Px(10.0);
        let px2 = Px(20.0);
        px1 -= px2;
        assert_eq!(px1, Px(-10.0));
    }

    #[test]
    fn it_mul_pixels() {
        let px1 = Px(10.0);
        let px2 = Px(20.0);
        let px3 = px1 * px2;
        assert_eq!(px3, Px(200.0));
    }

    #[test]
    fn it_mul_pixels_assign() {
        let mut px1 = Px(10.0);
        let px2 = Px(20.0);
        px1 *= px2;
        assert_eq!(px1, Px(200.0));
    }

    #[test]
    fn it_mul_pixels_assign_f32() {
        let mut px1 = Px(10.0);
        let px2 = 20.0;
        px1 *= px2;
        assert_eq!(px1, Px(200.0));
    }

    #[test]
    fn it_div_pixels() {
        let px1 = Px(10.0);
        let px2 = Px(20.0);
        let px3 = px1 / px2;
        assert_eq!(px3, Px(0.5));
    }

    #[test]
    fn it_div_pixels_assign() {
        let mut px1 = Px(10.0);
        let px2 = Px(20.0);
        px1 /= px2;
        assert_eq!(px1, Px(0.5));
    }

    #[test]
    fn it_div_pixels_assign_f32() {
        let mut px1 = Px(10.0);
        let px2 = 20.0;
        px1 /= px2;
        assert_eq!(px1, Px(0.5));
    }

    #[test]
    fn it_eq_pixels() {
        let px1 = Px(10.0);
        let px2 = Px(10.0);
        assert_eq!(px1, px2);
    }

    #[test]
    fn it_eq_pixels_with_rounding() {
        let px1 = Px(10.0001);
        let px2 = Px(10.0002);
        assert_eq!(px1, px2);
    }

    #[test]
    fn it_convert_point_to_pixel_with_dpi() {
        let pt = Pt(10.0);
        let px: Px = pt.into_px(96.0);
        assert_eq!(px, Px(13.333343));
    }

    #[test]
    fn it_convert_pixel_to_point_with_dpi() {
        let px = Px(10.0);
        let pt: Pt = px.into_pt(96.0);
        assert_eq!(pt, Pt(7.499994));
    }

    #[test]
    fn it_convert_pixel_to_point_with_dpi_160() {
        let px = Px(10.0);
        let pt: Pt = px.into_pt(160.0);
        assert_eq!(pt, Pt(4.5));
    }

    #[test]
    fn it_convert_point_to_pixel_with_dpi_160() {
        let pt = Pt(10.0);
        let px: Px = pt.into_px(160.0);
        assert_eq!(px, Px(22.2222222222));
    }

}