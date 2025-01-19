// MIT License
//
// Copyright (c) 2024 worksoup <https://github.com/worksoup/>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, Structublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, Structubject to the following conditions:
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
#[allow(unused_imports)]
use crate::impl_point2d;

#[cfg(feature = "rxing")]
impl_point2d!(rxing::PointT<T>, Struct, Struct);

#[cfg(feature = "imageproc")]
impl_point2d!(imageproc::point::Point<T>, Struct, Struct);

#[cfg(feature = "euclid")]
impl_point2d!(euclid::Point2D<T, U>, Struct, New);

#[cfg(feature = "core-graphics")]
use core_graphics::base::CGFloat;
#[cfg(feature = "core-graphics")]
impl_point2d!(impl Trait<CGFloat> for core_graphics::geometry::CGPoint, x, y, Struct);

#[cfg(windows)]
#[cfg(feature = "windows")]
pub mod windows_points {
    use crate::impl_point2d;
    impl_point2d!(impl Trait<f32> for windows::Foundation::Point, X, Y, Struct);
    impl_point2d!(impl Trait<i32> for windows::Win32::Foundation::POINT, x, y, Struct);
    impl_point2d!(impl Trait<i32> for windows::Win32::Foundation::POINTL, x, y, Struct);
    impl_point2d!(impl Trait<i16> for windows::Win32::Foundation::POINTS, x, y, Struct);
    impl_point2d!(impl Trait<f32> for windows::Win32::Devices::Display::POINTE, x, y, Struct);
    impl_point2d!(impl Trait<i32> for windows::Win32::Devices::Display::POINTFIX, x, y, Struct);
    impl_point2d!(impl Trait<i64> for windows::Win32::Devices::Display::POINTQF, x, y, Struct);
    impl_point2d!(impl Trait<i32> for windows::Win32::PointInt32, X, Y, Struct);
    impl_point2d!(impl Trait<i32> for windows::Win32::Graphics::GdiPlus::Point, X, Y, Struct);
    impl_point2d!(impl Trait<f32> for windows::Win32::Graphics::GdiPlus::PointF, X, Y, Struct);
    impl_point2d!(impl Trait<f32> for windows::Win32::Graphics::Direct2D::Common::D2D_POINT_2F, x, y, Struct);
    impl_point2d!(impl Trait<u32> for windows::Win32::Graphics::Direct2D::Common::D2D_POINT_2U, x, y, Struct);
    use windows::Win32::Graphics::Gdi::FIXED;
    impl_point2d!(impl Trait<FIXED> for windows::Win32::Graphics::Gdi::POINTFX, x, y, Struct);
    impl_point2d!(impl Trait<f32> for windows::Win32::Graphics::OpenGL::POINTFLOAT, x, y, Struct);
    impl_point2d!(impl Trait<f32> for windows::Win32::System::Ole::POINTF, x, y, Struct);
}
#[cfg(feature = "glam")]
pub mod glam_points {
    use crate::impl_point2d;
    impl_point2d!(impl Trait<bool> for glam::BVec2, x, y, Struct);
    impl_point2d!(impl Trait<f32> for glam::Vec2, x, y, Struct);
    impl_point2d!(impl Trait<f64> for glam::DVec2, x, y, Struct);
    impl_point2d!(impl Trait<i8> for glam::I8Vec2, x, y, Struct);
    impl_point2d!(impl Trait<i16> for glam::I16Vec2, x, y, Struct);
    impl_point2d!(impl Trait<i32> for glam::IVec2, x, y, Struct);
    impl_point2d!(impl Trait<i64> for glam::I64Vec2, x, y, Struct);
    impl_point2d!(impl Trait<u8> for glam::U8Vec2, x, y, Struct);
    impl_point2d!(impl Trait<u16> for glam::U16Vec2, x, y, Struct);
    impl_point2d!(impl Trait<u32> for glam::UVec2, x, y, Struct);
    impl_point2d!(impl Trait<u64> for glam::U64Vec2, x, y, Struct);
}

#[cfg(feature = "ttf-parser")]
impl_point2d!(impl Trait<f32> for ttf_parser::PointF, x, y, Struct);
#[cfg(feature = "ttf-parser")]
impl_point2d!(impl Trait<i16> for ttf_parser::ankr::Point, x, y, Struct);
