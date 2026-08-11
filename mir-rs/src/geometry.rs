/*
 * Copyright © Canonical Ltd.
 *
 * This program is free software: you can redistribute it and/or modify it
 * under the terms of the GNU General Public License version 2 or 3,
 * as published by the Free Software Foundation.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

//! Geometry primitives for window and output positioning.
//!
//! These types represent positions, sizes, and regions in the compositor's
//! coordinate space. All values are in logical pixels.
//!
//! The types are generic over a [`Scalar`] coordinate type. The default is
//! `i32`; use the `*F` type aliases (e.g. [`PointF`]) for `f32` variants.

use std::ops::{Add, Sub};

mod sealed {
    pub trait Sealed {}
    impl Sealed for i32 {}
    impl Sealed for f32 {}
}

/// Numeric coordinate type. Implemented for `i32` and `f32`.
///
/// This trait is sealed — external implementations are not allowed.
pub trait Scalar:
    sealed::Sealed + Copy + Default + PartialOrd + Add<Output = Self> + Sub<Output = Self>
{
}

impl Scalar for i32 {}
impl Scalar for f32 {}

/// A 2D point in logical pixel coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Point<T: Scalar = i32> {
    /// The x coordinate.
    pub x: T,
    /// The y coordinate.
    pub y: T,
}

impl<T: Scalar> Point<T> {
    /// Create a new point.
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl Eq for Point<i32> {}

impl std::hash::Hash for Point<i32> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl<T: Scalar> Add<Displacement<T>> for Point<T> {
    type Output = Point<T>;
    fn add(self, rhs: Displacement<T>) -> Point<T> {
        Point {
            x: self.x + rhs.dx,
            y: self.y + rhs.dy,
        }
    }
}

impl<T: Scalar> Sub<Point<T>> for Point<T> {
    type Output = Displacement<T>;
    fn sub(self, rhs: Point<T>) -> Displacement<T> {
        Displacement {
            dx: self.x - rhs.x,
            dy: self.y - rhs.y,
        }
    }
}

/// Floating-point variant of [`Point`] for sub-pixel precision.
pub type PointF = Point<f32>;

/// A 2D size in logical pixels.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Size<T: Scalar = i32> {
    /// The width in logical pixels.
    pub width: T,
    /// The height in logical pixels.
    pub height: T,
}

impl<T: Scalar> Size<T> {
    /// Create a new size.
    pub fn new(width: T, height: T) -> Self {
        Self { width, height }
    }
}

impl Eq for Size<i32> {}

impl std::hash::Hash for Size<i32> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.width.hash(state);
        self.height.hash(state);
    }
}

/// Floating-point variant of [`Size`].
pub type SizeF = Size<f32>;

/// A rectangle defined by its top-left corner and size.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Rectangle<T: Scalar = i32> {
    /// The top-left corner of the rectangle.
    pub top_left: Point<T>,
    /// The size of the rectangle.
    pub size: Size<T>,
}

impl<T: Scalar> Rectangle<T> {
    /// Create a new rectangle.
    pub fn new(top_left: Point<T>, size: Size<T>) -> Self {
        Self { top_left, size }
    }

    /// Check if a point is contained within this rectangle.
    pub fn contains(&self, point: Point<T>) -> bool {
        point.x >= self.top_left.x
            && point.x < self.top_left.x + self.size.width
            && point.y >= self.top_left.y
            && point.y < self.top_left.y + self.size.height
    }
}

impl Eq for Rectangle<i32> {}

impl std::hash::Hash for Rectangle<i32> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.top_left.hash(state);
        self.size.hash(state);
    }
}

/// Floating-point variant of [`Rectangle`].
pub type RectangleF = Rectangle<f32>;

/// A 2D displacement vector in logical pixels.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Displacement<T: Scalar = i32> {
    /// The horizontal displacement.
    pub dx: T,
    /// The vertical displacement.
    pub dy: T,
}

impl<T: Scalar> Displacement<T> {
    /// Create a new displacement.
    pub fn new(dx: T, dy: T) -> Self {
        Self { dx, dy }
    }
}

impl Eq for Displacement<i32> {}

impl std::hash::Hash for Displacement<i32> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.dx.hash(state);
        self.dy.hash(state);
    }
}

impl<T: Scalar> Add for Displacement<T> {
    type Output = Displacement<T>;
    fn add(self, rhs: Displacement<T>) -> Displacement<T> {
        Displacement {
            dx: self.dx + rhs.dx,
            dy: self.dy + rhs.dy,
        }
    }
}

impl<T: Scalar> Sub for Displacement<T> {
    type Output = Displacement<T>;
    fn sub(self, rhs: Displacement<T>) -> Displacement<T> {
        Displacement {
            dx: self.dx - rhs.dx,
            dy: self.dy - rhs.dy,
        }
    }
}

/// Floating-point variant of [`Displacement`].
pub type DisplacementF = Displacement<f32>;

// --- FFI conversions ---

impl From<mir_sys::ffi::Point> for Point<i32> {
    fn from(p: mir_sys::ffi::Point) -> Self {
        Self { x: p.x, y: p.y }
    }
}

impl From<Point<i32>> for mir_sys::ffi::Point {
    fn from(p: Point<i32>) -> Self {
        Self { x: p.x, y: p.y }
    }
}

impl From<mir_sys::ffi::Size> for Size<i32> {
    fn from(s: mir_sys::ffi::Size) -> Self {
        Self {
            width: s.width,
            height: s.height,
        }
    }
}

impl From<Size<i32>> for mir_sys::ffi::Size {
    fn from(s: Size<i32>) -> Self {
        Self {
            width: s.width,
            height: s.height,
        }
    }
}

impl From<mir_sys::ffi::Rectangle> for Rectangle<i32> {
    fn from(r: mir_sys::ffi::Rectangle) -> Self {
        Self {
            top_left: r.top_left.into(),
            size: r.size.into(),
        }
    }
}

impl From<Rectangle<i32>> for mir_sys::ffi::Rectangle {
    fn from(r: Rectangle<i32>) -> Self {
        Self {
            top_left: r.top_left.into(),
            size: r.size.into(),
        }
    }
}

impl From<mir_sys::ffi::Displacement> for Displacement<i32> {
    fn from(d: mir_sys::ffi::Displacement) -> Self {
        Self { dx: d.dx, dy: d.dy }
    }
}

impl From<Displacement<i32>> for mir_sys::ffi::Displacement {
    fn from(d: Displacement<i32>) -> Self {
        Self { dx: d.dx, dy: d.dy }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_plus_displacement_translates() {
        assert_eq!(
            Point::new(10, 20) + Displacement::new(5, -5),
            Point::new(15, 15)
        );
    }

    #[test]
    fn point_minus_point_is_a_displacement() {
        assert_eq!(
            Point::new(15, 15) - Point::new(10, 20),
            Displacement::new(5, -5)
        );
    }

    #[test]
    fn displacements_add_and_subtract() {
        assert_eq!(
            Displacement::new(1, 2) + Displacement::new(3, 4),
            Displacement::new(4, 6)
        );
        assert_eq!(
            Displacement::new(3, 4) - Displacement::new(1, 2),
            Displacement::new(2, 2)
        );
    }

    #[test]
    fn rectangle_contains_is_half_open() {
        let rect = Rectangle::new(Point::new(0, 0), Size::new(100, 50));
        assert!(rect.contains(Point::new(0, 0)));
        assert!(rect.contains(Point::new(99, 49)));
        // The bottom-right edge is exclusive.
        assert!(!rect.contains(Point::new(100, 49)));
        assert!(!rect.contains(Point::new(99, 50)));
        assert!(!rect.contains(Point::new(-1, 0)));
    }

    #[test]
    fn empty_rectangle_contains_nothing() {
        let rect = Rectangle::new(Point::new(10, 10), Size::new(0, 0));
        assert!(!rect.contains(Point::new(10, 10)));
    }

    #[test]
    fn float_variants_work() {
        let rect = RectangleF::new(PointF::new(0.0, 0.0), SizeF::new(1.5, 1.5));
        assert!(rect.contains(PointF::new(1.4, 1.4)));
        assert!(!rect.contains(PointF::new(1.5, 1.4)));
    }

    #[test]
    fn geometry_types_round_trip_through_ffi() {
        let point = Point::new(3, 4);
        assert_eq!(Point::from(mir_sys::ffi::Point::from(point)), point);

        let size = Size::new(800, 600);
        assert_eq!(Size::from(mir_sys::ffi::Size::from(size)), size);

        let displacement = Displacement::new(-1, 2);
        assert_eq!(
            Displacement::from(mir_sys::ffi::Displacement::from(displacement)),
            displacement
        );

        let rect = Rectangle::new(point, size);
        assert_eq!(Rectangle::from(mir_sys::ffi::Rectangle::from(rect)), rect);
    }
}
