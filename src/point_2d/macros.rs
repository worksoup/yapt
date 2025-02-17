#[macro_export]
macro_rules! impl_point2d {
    ($root:ident $(:: $idents:ident )*<$t:ident$(, $other:ident)*>,Tuple, $expr:tt$(, $xx:tt, $yy:tt)?) => {
        impl_point2d!($root$(::$idents)*<$t$(,$other)*>,0,1,$expr$(, $xx, $yy)?);
    };
    ($root:ident $(:: $idents:ident )*<$t:ident$(, $other:ident)*>,Struct, $expr:tt$(, $xx:tt, $yy:tt)?) => {
        impl_point2d!($root$(::$idents)*<$t$(,$other)*>,x,y,$expr$(, $xx, $yy)?);
    };
    (impl $(<$t1:ident$(, $other1:ident)*>)? Trait<$t2:ident> for $root:ident $(:: $idents:ident )*$(<$t3:ident$(, $other2:ident)*>)?$(where {$($case:tt)?*})?, $x:tt, $y:tt, $expr:tt$(, $xx:tt, $yy:tt)?) => {
        impl$(<$t1$(,$other1)*>)? $crate::point_2d::Point2D<$t2> for $root$(::$idents)*$(<$t3$(,$other2)*>)?$(where $($case)?*)?{
            #[inline]
            fn rx(&self) -> &$t2 {
                &self.$x
            }
            #[inline]
            fn ry(&self) -> &$t2 {
                &self.$y
            }
            #[inline]
            fn into_point(self) -> $crate::point_2d::Point<$t2> {
                $crate::point_2d::Point {
                    x: self.$x,
                    y: self.$y,
                }
            }
            #[inline]
            fn from_point(p: $crate::point_2d::Point<$t2>) -> Self {
                let xy = impl_point2d!($expr, $t2$(, $xx, $yy)?, $x, $y);
                xy(p)
            }
        }
    };
    ($root:ident $(:: $idents:ident )*<$t:ident$(, $other:ident)*>, $x:tt, $y:tt, $expr:tt$(, $xx:tt, $yy:tt)?) => {
        impl_point2d!(impl<$t$(, $other)*> Trait<$t> for $root $(:: $idents )*<$t$(, $other)*>, $x, $y, $expr$(, $xx, $yy)?);
    };
    (New, $t:ident$(, $xx:tt, $yy:tt)?)=>{
        |p: $crate::point_2d::Point<$t>| -> Self{
            Self::new(p.x, p.y)
        }
    };
    (Tuple, $t:ident, $xx:tt, $yy:tt$(, $x:tt, $y:tt)?)=>{
        |p: $crate::point_2d::Point<$t>| -> Self{
            Self(p.x, p.y)
        }
    };
    (Struct, $t:ident, $xx:tt, $yy:tt$(, $x:tt, $y:tt)?)=>{
        |p: $crate::point_2d::Point<$t>| -> Self{
            Self{$xx:p.x,$yy: p.y}
        }
    }
}
