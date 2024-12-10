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
#![allow(unused_macros)]
#![allow(unused_imports)]
#![allow(dead_code)]
use crate::{Point, Point2D};

trait Construct<T> {
    fn new(point_2d: Point<T>) -> Self;
}

#[macro_export]
macro_rules! impl_construct_new {
    ($root:ident $(:: $idents:ident )*<$t:ident$(,$other:ident)*>$(,$modifiers:tt)?) => {
        impl<$t$(,$other)*> Construct<$t> for $root$(::$idents)*<$t$(,$other)*> {
            fn new(point_2d: Point<$t>) -> Self {
                Self::new($($modifiers)? point_2d.x,$($modifiers)? point_2d.y)
            }
        }
    };
}
#[macro_export]
macro_rules! impl_construct_tuple {
    ($root:ident $(:: $idents:ident )*<$t:ident$(,$other:ident)*>$(,$modifiers:tt)?) => {
        impl<$t$(,$other)*> Construct<$t> for $root$(::$idents)*<$t$(,$other)*> {
            fn new(point_2d: Point<$t>) -> Self {
                Self($($modifiers)? point_2d.x,$($modifiers)? point_2d.y)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_construct_struct {
    ($root:ident $(:: $idents:ident )*<$t:ident$(,$other:ident)*>$(,$modifiers:tt)?) => {
        impl<$t$(,$other)*> Construct<$t> for $root$(::$idents)*<$t$(,$other)*> {
            fn new(point_2d: Point<$t>) -> Self {
                Self{x:$($modifiers)? point_2d.x,y:$($modifiers)? point_2d.y}
            }
        }
    };
}

#[macro_export]
macro_rules! impl_point2d {
    ($root:ident $(:: $idents:ident )*<$t:ident$(,$other:ident)*>) => {
        impl<$t$(,$other)*> Point2D<$t> for $root$(::$idents)*<$t$(,$other)*> {
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
                <Self as Construct<T>>::new(point_2d)
            }
        }
    };
}

#[cfg(feature = "rxing")]
impl_point2d!(rxing::PointT<T>);
#[cfg(feature = "rxing")]
impl_construct_struct!(rxing::PointT<T>);

#[cfg(feature = "imageproc")]
impl_point2d!(imageproc::point::Point<T>);
#[cfg(feature = "imageproc")]
impl_construct_struct!(imageproc::point::Point<T>);

#[cfg(feature = "euclid")]
impl_point2d!(euclid::Point2D<T, U>);
#[cfg(feature = "euclid")]
impl_construct_new!(euclid::Point2D<T, U>);
