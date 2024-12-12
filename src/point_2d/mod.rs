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

mod macros;
pub mod points;

use std::ops::{Deref, DerefMut};

/// 所有类似的二维点类型都应当实现该 trait.
pub trait Point2D<T> {
    /// 获取 x 坐标的引用。
    fn rx(&self) -> &T;
    /// 获取 y 坐标的引用。
    fn ry(&self) -> &T;
    /// 转换为内建 [`Point`] 类型。
    fn into_point(self) -> Point<T>;
    /// 通过内建 [`Point`] 类型构造。
    fn from_point(point_2d: Point<T>) -> Self;
    /// 使用 x, y 坐标的引用构造内建 [`Point`] 类型。
    fn as_point(&self) -> Point<&T> {
        Point {
            x: self.rx(),
            y: self.ry(),
        }
    }
}

impl<T> Point2D<T> for (T, T) {
    fn rx(&self) -> &T {
        &self.0
    }

    fn ry(&self) -> &T {
        &self.1
    }

    fn into_point(self) -> Point<T> {
        Point {
            x: self.0,
            y: self.1,
        }
    }

    fn from_point(point_2d: Point<T>) -> Self {
        (point_2d.x, point_2d.y)
    }
}
impl<T> Point2D<T> for [T; 2] {
    fn rx(&self) -> &T {
        &self[0]
    }

    fn ry(&self) -> &T {
        &self[1]
    }

    fn into_point(self) -> Point<T> {
        let [x, y] = self;
        Point { x, y }
    }

    fn from_point(point_2d: Point<T>) -> Self {
        [point_2d.x, point_2d.y]
    }
}
/// # [`Point`]
/// 内建 `Point` 类型，用作二维点类型之间转换的的桥梁。
///
/// 你可以先调用 [`Point2D::into_point`] 将一个点类型转换为 `Point`，
/// 再调用 [`Point::into_point_2d`] 将 `Point` 转换为另一个点类型。
///
/// 这些函数都是各自的成员函数，可以方便地采用链式调用风格。
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
impl<T> Point2D<T> for Point<T> {
    fn rx(&self) -> &T {
        &self.x
    }

    fn ry(&self) -> &T {
        &self.y
    }

    fn into_point(self) -> Point<T> {
        self
    }

    fn from_point(point_2d: Point<T>) -> Self {
        point_2d
    }
}
impl<T> Point<&T> {
    pub fn copied(&self) -> Point<T>
    where
        T: Copy,
    {
        Point {
            x: *self.x,
            y: *self.y,
        }
    }
    pub fn cloned(&self) -> Point<T>
    where
        T: Clone,
    {
        Point {
            x: self.x.clone(),
            y: self.y.clone(),
        }
    }
}

impl<T> Point<&mut T> {
    pub fn copied(&self) -> Point<T>
    where
        T: Copy,
    {
        Point {
            x: *self.x,
            y: *self.y,
        }
    }
    pub fn cloned(&self) -> Point<T>
    where
        T: Clone,
    {
        Point {
            x: self.x.clone(),
            y: self.y.clone(),
        }
    }
}
impl<T> Point<Option<T>> {
    pub fn unwrap(self) -> Option<Point<T>> {
        match self {
            Point {
                x: Some(x),
                y: Some(y),
            } => Some(Point { x, y }),
            _ => None,
        }
    }
}
impl<T, E> Point<Result<T, E>> {
    pub fn unwrap(self) -> Option<Point<T>> {
        match self {
            Point { x: Ok(x), y: Ok(y) } => Some(Point { x, y }),
            _ => None,
        }
    }
    pub fn unwrap_err(self) -> Option<Point<E>> {
        match self {
            Point {
                x: Err(x),
                y: Err(y),
            } => Some(Point { x, y }),
            _ => None,
        }
    }
}
impl<T, U> Point<(T, U)> {
    pub fn unzip(self) -> (Point<T>, Point<U>) {
        (
            Point {
                x: self.x.0,
                y: self.y.0,
            },
            Point {
                x: self.x.1,
                y: self.y.1,
            },
        )
    }
}
impl<T> Point<T> {
    pub const fn as_ref(&self) -> Point<&T> {
        Point {
            x: &self.x,
            y: &self.y,
        }
    }
    pub fn zip<U>(self, other: Point<U>) -> Point<(T, U)> {
        Point {
            x: (self.x, other.x),
            y: (self.y, other.y),
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
    pub fn as_deref_mut(&mut self) -> Point<&mut T::Target>
    where
        T: DerefMut,
    {
        Point {
            x: &mut self.x,
            y: &mut self.y,
        }
    }
    pub fn from_point_2d<I>(p: I) -> Self
    where
        I: Point2D<T>,
    {
        p.into_point()
    }
    pub fn into_point_2d<O>(self) -> O
    where
        O: Point2D<T>,
    {
        O::from_point(self)
    }
    pub fn as_point_2d<'a, O>(&'a self) -> O
    where
        O: Point2D<&'a T>,
    {
        O::from_point(self.as_ref())
    }
    pub fn map<U, F: Fn(T) -> U>(self, f: F) -> Point<U> {
        Point {
            x: f(self.x),
            y: f(self.y),
        }
    }
    pub const fn iter(&self) -> Iter<'_, T> {
        let Point { x, y } = self;
        Iter {
            inner: IterInner(Some(x), Some(y)),
        }
    }
}

struct IterInner<A>(Option<A>, Option<A>);
impl<A> IterInner<A> {
    fn next(&mut self) -> Option<A> {
        let next = self.0.take()?;
        std::mem::swap(&mut self.0, &mut self.1);
        Some(next)
    }
}
impl<A> Point2D<Option<A>> for IterInner<A> {
    fn rx(&self) -> &Option<A> {
        &self.0
    }

    fn ry(&self) -> &Option<A> {
        &self.1
    }

    fn into_point(self) -> Point<Option<A>> {
        Point {
            x: self.0,
            y: self.1,
        }
    }

    fn from_point(point_2d: Point<Option<A>>) -> Self {
        IterInner(point_2d.x, point_2d.y)
    }
}
pub struct Iter<'a, A: 'a> {
    inner: IterInner<&'a A>,
}
impl<'a, A> Iterator for Iter<'a, A> {
    type Item = &'a A;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}
pub struct IntoIter<A> {
    inner: IterInner<A>,
}
impl<A> Iterator for IntoIter<A> {
    type Item = A;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}
impl<T> IntoIterator for Point<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            inner: self.map(|a| Some(a)).into_point_2d(),
        }
    }
}
