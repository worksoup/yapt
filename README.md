# YAPT

提供一些简单的类型以及其与类似类型之间的转换。

比如各个 crate 之间有各种各样的 Point2D 类型，它们都有一个 x, 一个 y, 但偏偏不能与彼此的 Point2D 互通。

本 crate 提供了一个 `Point<T>` 类型，与一个 `Point2D` trait.
提供了一个 `impl_point_2d!`, 用来方便实现 `Point2D` trait.

本 crate 为不同的 Point2D 类型实现 `Point2D` trait, 如此这些类型即可与 `Point` 相互转换。

例如 `PointA` 和 `PointB`, 可以通过

``` Rust
let pa = PointA::<u32>::default();
let pb: PointB<_> = pa.into_point().into_point_2d();
```

相互转换。但前提是本库为这两种 Point2D 实现 `Point2D` trait.