/*
* MIT License
*
* Copyright (c) 2018 Clément SIBILLE
*
* Permission is hereby granted, free of charge, to any person obtaining a copy
* of this software and associated documentation files (the "Software"), to deal
* in the Software without restriction, including without limitation the rights
* to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
* copies of the Software, and to permit persons to whom the Software is
* furnished to do so, subject to the following conditions:
*
* The above copyright notice and this permission notice shall be included in all
* copies or substantial portions of the Software.
*
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
* FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
* AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
* LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
* OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
* SOFTWARE.
*/

use crate::number_traits::Zero;
use std::ops::{Add, Sub, Mul};

pub type Vector2<T> = [T; 2];
pub type Vector2i = Vector2<i32>;
pub type Vector2f = Vector2<f32>;

pub type Vector3<T> = [T; 3];
pub type Vector3i = Vector3<i32>;
pub type Vector3f = Vector3<f32>;

pub type Vector4<T> = [T; 4];
pub type Vector4i = Vector4<i32>;
pub type Vector4f = Vector4<f32>;

/// Adds two Vector2<T> together
///
/// # Examples
///
/// ```
/// use stones::vector::vec2_add;
///
/// let v1 = [5, 3];
/// let v2 = [12, -8];
/// assert_eq!(vec2_add(v1, v2), [17, -5]);
/// ```
pub fn vec2_add<T>(lhs: Vector2<T>, rhs: Vector2<T>) -> Vector2<T>
    where T: Copy + Add<Output=T>
{
    [
        lhs[0] + rhs[0],
        lhs[1] + rhs[1]
    ]
}

/// Adds two Vector3<T> together
///
/// # Examples
///
/// ```
/// use stones::vector::vec3_add;
///
/// let v1 = [5, 3, 7];
/// let v2 = [12, -8, -2];
/// assert_eq!(vec3_add(v1, v2), [17, -5, 5]);
/// ```
pub fn vec3_add<T>(lhs: Vector3<T>, rhs: Vector3<T>) -> Vector3<T>
    where T: Copy + Add<Output=T>
{
    [
        lhs[0] + rhs[0],
        lhs[1] + rhs[1],
        lhs[2] + rhs[2]
    ]
}

/// Adds two Vector4<T> together
///
/// # Examples
///
/// ```
/// use stones::vector::vec4_add;
///
/// let v1 = [5, 3, 7, 2];
/// let v2 = [12, -8, -2, 1];
/// assert_eq!(vec4_add(v1, v2), [17, -5, 5, 3]);
/// ```
pub fn vec4_add<T>(lhs: Vector4<T>, rhs: Vector4<T>) -> Vector4<T>
    where T: Copy + Add<Output=T>
{
    [
        lhs[0] + rhs[0],
        lhs[1] + rhs[1],
        lhs[2] + rhs[2],
        lhs[3] + rhs[3]
    ]
}

/// Subtracts a Vector2<T> from another
///
/// # Examples
///
/// ```
/// use stones::vector::vec2_sub;
///
/// let v1 = [5, 3];
/// let v2 = [12, -8];
/// assert_eq!(vec2_sub(v1, v2), [-7, 11]);
/// ```
pub fn vec2_sub<T>(lhs: Vector2<T>, rhs: Vector2<T>) -> Vector2<T>
    where T: Copy + Sub<Output=T>
{
    [
        lhs[0] - rhs[0],
        lhs[1] - rhs[1]
    ]
}


/// Subtracts a Vector3<T> from another
///
/// # Examples
///
/// ```
/// use stones::vector::vec3_sub;
///
/// let v1 = [5, 3, 7];
/// let v2 = [12, -8, -2];
/// assert_eq!(vec3_sub(v1, v2), [-7, 11, 9]);
/// ```
pub fn vec3_sub<T>(lhs: Vector3<T>, rhs: Vector3<T>) -> Vector3<T>
    where T: Copy + Sub<Output=T>
{
    [
        lhs[0] - rhs[0],
        lhs[1] - rhs[1],
        lhs[2] - rhs[2]
    ]
}

/// Subtracts a Vector4<T> from another
///
/// # Examples
///
/// ```
/// use stones::vector::vec4_sub;
///
/// let v1 = [5, 3, 7, 2];
/// let v2 = [12, -8, -2, 1];
/// assert_eq!(vec4_sub(v1, v2), [-7, 11, 9, 1]);
/// ```
pub fn vec4_sub<T>(lhs: Vector4<T>, rhs: Vector4<T>) -> Vector4<T>
    where T: Copy + Sub<Output=T>
{
    [
        lhs[0] - rhs[0],
        lhs[1] - rhs[1],
        lhs[2] - rhs[2],
        lhs[3] - rhs[3]
    ]
}


/// Multiplies a Vector2<T> by a scalar
///
/// # Examples
///
/// ```
/// use stones::vector::vec2_mul;
///
/// let v1 = [5, 3];
/// let scalar = 2;
/// assert_eq!(vec2_mul(v1, scalar), [10, 6]);
/// ```
pub fn vec2_mul<T>(lhs: Vector2<T>, rhs: T) -> Vector2<T>
    where T: Copy + Mul<Output=T>
{
    [
        lhs[0] * rhs,
        lhs[1] * rhs
    ]
}

/// Multiplies a Vector3<T> by a scalar
///
/// # Examples
///
/// ```
/// use stones::vector::vec3_mul;
///
/// let v1 = [5, 3, 7];
/// let scalar = 2;
/// assert_eq!(vec3_mul(v1, scalar), [10, 6, 14]);
/// ```
pub fn vec3_mul<T>(lhs: Vector3<T>, rhs: T) -> Vector3<T>
    where T: Copy + Mul<Output=T>
{
    [
        lhs[0] * rhs,
        lhs[1] * rhs,
        lhs[2] * rhs
    ]
}

/// Multiplies a Vector4<T> by a scalar
///
/// # Examples
///
/// ```
/// use stones::vector::vec4_mul;
///
/// let v1 = [5, 3, 7, 2];
/// let scalar = 2;
/// assert_eq!(vec4_mul(v1, scalar), [10, 6, 14, 4]);
/// ```
pub fn vec4_mul<T>(lhs: Vector4<T>, rhs: T) -> Vector4<T>
    where T: Copy + Mul<Output=T>
{
    [
        lhs[0] * rhs,
        lhs[1] * rhs,
        lhs[2] * rhs,
        lhs[3] * rhs
    ]
}

/// Calculates the dot product of two Vector2<T>
///
/// # Examples
///
/// ```
/// use stones::vector::vec2_dot;
///
/// let v1 = [0.6, -0.8];
/// let v2 = [0.0, 1.0];
/// assert_eq!(vec2_dot(v1, v2), -0.8);
/// ```
pub fn vec2_dot<T>(lhs: Vector2<T>, rhs: Vector2<T>) -> T
    where T: Zero + Copy + Mul<Output=T> + Add<Output=T>
{
    dot_product(lhs.iter(), rhs.iter())
}

/// Calculates the dot product of two Vector3<T>
///
/// # Examples
///
/// ```
/// use stones::vector::vec3_dot;
///
/// let v1 = [0.6, -0.8, 2.0];
/// let v2 = [0.0, 1.0, 1.0];
/// assert_eq!(vec3_dot(v1, v2), 1.2);
/// ```
pub fn vec3_dot<T>(lhs: Vector3<T>, rhs: Vector3<T>) -> T
    where T: Zero + Copy + Mul<Output=T> + Add<Output=T>
{
    dot_product(lhs.iter(), rhs.iter())
}

/// Calculates the dot product of two Vector2<T>
///
/// # Examples
///
/// ```
/// use stones::vector::vec4_dot;
///
/// let v1 = [0.6, -0.8, 2.1, 3.2];
/// let v2 = [0.0, 1.0, 1.5, 1.9];
/// assert_eq!(vec4_dot(v1, v2), 8.43);
/// ```
pub fn vec4_dot<T>(lhs: Vector4<T>, rhs: Vector4<T>) -> T
    where T: Zero + Copy + Mul<Output=T> + Add<Output=T>
{
    dot_product(lhs.iter(), rhs.iter())
}

/// Calculates the cross product of two Vector3<T>
///
/// # Examples
///
/// ```
/// use stones::vector::vec3_cross;
///
/// let v1 = [1, 0, 0];
/// let v2 = [0, 1, 0];
/// assert_eq!(vec3_cross(v1, v2), [0, 0, 1]);
/// ```
pub fn vec3_cross<T>(lhs: Vector3<T>, rhs: Vector3<T>) -> Vector3<T>
    where T: Copy + Mul<Output=T> + Sub<Output=T>
{
    [
        lhs[1] * rhs[2] - lhs[2] * rhs[1],
        lhs[2] * rhs[0] - lhs[0] * rhs[2],
        lhs[0] * rhs[1] - lhs[1] * rhs[0]
    ]
}


fn dot_product<T>(lhs: std::slice::Iter<T>, rhs: std::slice::Iter<T>) -> T 
    where T: Zero + Copy + Mul<Output=T> + Add<Output=T>
{
    lhs.zip(rhs)
        .fold(T::zero(), |acc, (&a, &b)| acc + a * b)
}
