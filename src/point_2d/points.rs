// MIT License
//
// Copyright (c) 2024 worksoup <https://github.com/worksoup/>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use crate::{Point, Point2D};
macro_rules! impl_point2d {
    ($root:ident $(:: $idents:ident )*<$t:ident>) => {
        impl<$t> Point2D<$t> for $root$(::$idents)*<$t> {
            fn rx(&self) -> &$t {
                &self.x
            }

            fn ry(&self) -> &$t {
                &self.y
            }

            fn into_point_2d(self) -> Point<$t> {
                Point {
                    x: self.x,
                    y: self.y,
                }
            }

            fn from_point_2d(point_2d: Point<$t>) -> Self {
                Self {
                    x: point_2d.x,
                    y: point_2d.y,
                }
            }
        }
    };
}

#[cfg(feature = "rxing")]
impl_point2d!(rxing::PointT<T>);
#[cfg(feature = "imageproc")]
impl_point2d!(imageproc::point::Point<T>);
