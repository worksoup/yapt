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

pub mod points;

use std::ops::Deref;

pub trait ConstructFromPoint<T> {
    fn new(point_2d: Point<T>) -> Self;
}

pub trait Point2D<T> {
    fn rx(&self) -> &T;
    fn ry(&self) -> &T;
    fn into_point_2d(self) -> Point<T>;
    fn from_point_2d(point_2d: Point<T>) -> Self;
    fn as_point_2d_ref(&self) -> Point<&T> {
        Point {
            x: self.rx(),
            y: self.ry(),
        }
    }
}
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T: Clone> Clone for Point<T> {
    fn clone(&self) -> Self {
        Self {
            x: self.x.clone(),
            y: self.y.clone(),
        }
    }
}
impl<T: Copy> Copy for Point<T> {}
impl<T> Point2D<T> for Point<T> {
    fn rx(&self) -> &T {
        &self.x
    }

    fn ry(&self) -> &T {
        &self.y
    }

    fn into_point_2d(self) -> Point<T> {
        self
    }

    fn from_point_2d(point_2d: Point<T>) -> Self {
        point_2d
    }
}
impl<T: Copy> Point<&T> {
    pub fn to_copied(&self) -> Point<T> {
        Point {
            x: *self.x,
            y: *self.y,
        }
    }
}

impl<T: Clone> Point<&T> {
    pub fn to_cloned(&self) -> Point<T> {
        Point {
            x: self.x.clone(),
            y: self.y.clone(),
        }
    }
}

impl<T> Point<T> {
    pub fn as_ref(&self) -> Point<&T> {
        Point {
            x: self.rx(),
            y: self.ry(),
        }
    }
    pub fn as_mut(&mut self) -> Point<&mut T> {
        Point {
            x: &mut self.x,
            y: &mut self.y,
        }
    }
    pub fn as_deref(&self) -> Point<&<T as Deref>::Target>
    where
        T: Deref,
    {
        Point {
            x: &self.x,
            y: &self.y,
        }
    }
}
