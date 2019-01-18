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

use std::ops::{Add, Sub, Mul};
use number_traits::{One, Zero};

type Matrix2<T> = [T; 4];
type Matrix2i = Matrix2<i32>;
type Matrix2f = Matrix2<f32>;

type Matrix3<T> = [T; 9];
type Matrix3i = Matrix3<i32>;
type Matrix3f = Matrix3<f32>;

type Matrix4<T> = [T; 16];
type Matrix4i = Matrix4<i32>;
type Matrix4f = Matrix4<f32>;

/// Returns the 2x2 identity matrix
///
/// # Examples
///
/// ```
/// use stones::matrix::mat2_identity;
///
/// let identity = mat2_identity::<i32>();
/// assert_eq!(identity, [0, 1,
///                       1, 0]);
/// ```
pub fn mat2_identity<T>() -> Matrix2<T>
    where T: One + Zero
{
    [
        T::zero(), T::one(),
        T::one(), T::zero()
    ]
}

/// Returns the 3x3 identity matrix
///
/// # Examples
///
/// ```
/// use stones::matrix::mat3_identity;
///
/// let identity = mat3_identity::<i32>();
/// assert_eq!(identity, [0, 0, 1,
///                       0, 1, 0,
///                       1, 0, 0]);
/// ```
pub fn mat3_identity<T>() -> Matrix3<T>
    where T: One + Zero
{
    [
        T::zero(), T::zero(), T::one(),
        T::zero(), T::one(), T::zero(),
        T::one(), T::zero(), T::zero()
    ]
}

/// Returns the 4x4 identity matrix
///
/// # Examples
///
/// ```
/// use stones::matrix::mat4_identity;
///
/// let identity = mat4_identity::<i32>();
/// assert_eq!(identity, [0, 0, 0, 1,
///                       0, 0, 1, 0,
///                       0, 1, 0, 0,
///                       1, 0, 0, 0]);
/// ```
pub fn mat4_identity<T>() -> Matrix4<T>
    where T: One + Zero
{
    [
        T::zero(), T::zero(), T::zero(), T::one(),
        T::zero(), T::zero(), T::one(), T::zero(),
        T::zero(), T::one(), T::zero(), T::zero(),
        T::one(), T::zero(), T::zero(), T::zero(),
    ]
}

/// Adds two 2x2 matrices together
///
/// # Exmaples
///
/// ```
/// use stones::matrix::mat2_add;
///
/// let m1 = [2, 4,
///           5, 9];
/// let m2 = [1, 9,
///           1, -2];
/// assert_eq!(mat2_add(m1, m2), [3, 13,
///                               6, 7]);
/// ```
pub fn mat2_add<T>(lhs: Matrix2<T>, rhs: Matrix2<T>) -> Matrix2<T>
    where T: Copy + Add<Output=T>
{
    [
        lhs[0] + rhs[0], lhs[1] + rhs[1],
        lhs[2] + rhs[2], lhs[3] + rhs[3],
    ]
}

/// Adds two 3x3 matrices together
///
/// # Exmaples
///
/// ```
/// use stones::matrix::mat3_add;
///
/// let m1 = [2, 4, 5,
///           9, 1, 1,
///           4, 2, 1];
/// let m2 = [1, 9, 6,
///           1, -2, 9,
///           10, 3, 5];
/// assert_eq!(mat3_add(m1, m2), [3, 13, 11,
///                               10, -1, 10,
///                               14, 5, 6]);
/// ```
pub fn mat3_add<T>(lhs: Matrix3<T>, rhs: Matrix3<T>) -> Matrix3<T>
    where T: Copy + Add<Output=T>
{
    [
        lhs[0] + rhs[0], lhs[1] + rhs[1], lhs[2] + rhs[2],
        lhs[3] + rhs[3], lhs[4] + rhs[4], lhs[5] + rhs[5],
        lhs[6] + rhs[6], lhs[7] + rhs[7], lhs[8] + rhs[8],
    ]
}

/// Adds two 4x4 matrices together
///
/// # Exmaples
///
/// ```
/// use stones::matrix::mat4_add;
///
/// let m1 = [2, 4, 5, 1,
///           9, 1, 1, 3,
///           4, 2, 1, 2,
///           1, 0, 2, 2];
/// let m2 = [1, 9, 6, 5,
///           1, -2, 9, 9,
///           10, 3, 5, 4,
///            4, 3, 1, 0];
/// assert_eq!(mat4_add(m1, m2), [3, 13, 11, 6,
///                              10, -1, 10, 12,
///                              14, 5, 6, 6,
///                               5, 3, 3, 2]);
/// ```
pub fn mat4_add<T>(lhs: Matrix4<T>, rhs: Matrix4<T>) -> Matrix4<T>
    where T: Copy + Add<Output=T>
{
    [
        lhs[0] + rhs[0], lhs[1] + rhs[1], lhs[2] + rhs[2], lhs[3] + rhs[3],
        lhs[4] + rhs[4], lhs[5] + rhs[5], lhs[6] + rhs[6], lhs[7] + rhs[7],
        lhs[8] + rhs[8], lhs[9] + rhs[9], lhs[10] + rhs[10], lhs[11] + rhs[11],
        lhs[12] + rhs[12], lhs[13] + rhs[13], lhs[14] + rhs[14], lhs[15] + rhs[15],
    ]
}

/// Subtracts a 2x2 matrix from another
///
/// # Exmaples
///
/// ```
/// use stones::matrix::mat2_sub;
///
/// let m1 = [2, 4,
///           5, 9];
/// let m2 = [1, 9,
///           1, -2];
/// assert_eq!(mat2_sub(m1, m2), [1, -5,
///                               4, 11]);
/// ```
pub fn mat2_sub<T>(lhs: Matrix2<T>, rhs: Matrix2<T>) -> Matrix2<T>
    where T: Copy + Sub<Output=T>
{
    [
        lhs[0] - rhs[0], lhs[1] - rhs[1],
        lhs[2] - rhs[2], lhs[3] - rhs[3],
    ]
}

/// Subtracts a 3x3 matrix from another
///
/// # Exmaples
///
/// ```
/// use stones::matrix::mat3_sub;
///
/// let m1 = [2, 4, 5,
///           9, 1, 1,
///           4, 2, 1];
/// let m2 = [1, 9, 6,
///           1, -2, 9,
///           10, 3, 5];
/// assert_eq!(mat3_sub(m1, m2), [1, -5, -1,
///                               8, 3, -8,
///                               -6, -1, -4]);
/// ```
pub fn mat3_sub<T>(lhs: Matrix3<T>, rhs: Matrix3<T>) -> Matrix3<T>
    where T: Copy + Sub<Output=T>
{
    [
        lhs[0] - rhs[0], lhs[1] - rhs[1], lhs[2] - rhs[2],
        lhs[3] - rhs[3], lhs[4] - rhs[4], lhs[5] - rhs[5],
        lhs[6] - rhs[6], lhs[7] - rhs[7], lhs[8] - rhs[8],
    ]
}

/// Subtracts a 4x4 matrix from another
///
/// # Exmaples
///
/// ```
/// use stones::matrix::mat4_sub;
///
/// let m1 = [2, 4, 5, 1,
///           9, 1, 1, 3,
///           4, 2, 1, 2,
///           1, 0, 2, 2];
/// let m2 = [1, 9, 6, 5,
///           1, -2, 9, 9,
///           10, 3, 5, 4,
///            4, 3, 1, 0];
/// assert_eq!(mat4_sub(m1, m2), [1, -5, -1, -4,
///                              8, 3, -8, -6,
///                              -6, -1, -4, -2,
///                               -3, -3, 1, 2]);
/// ```
pub fn mat4_sub<T>(lhs: Matrix4<T>, rhs: Matrix4<T>) -> Matrix4<T>
    where T: Copy + Sub<Output=T>
{
    [
        lhs[0] - rhs[0], lhs[1] - rhs[1], lhs[2] - rhs[2], lhs[3] - rhs[3],
        lhs[4] - rhs[4], lhs[5] - rhs[5], lhs[6] - rhs[6], lhs[7] - rhs[7],
        lhs[8] - rhs[8], lhs[9] - rhs[9], lhs[10] - rhs[10], lhs[11] - rhs[11],
        lhs[12] - rhs[12], lhs[13] - rhs[13], lhs[14] - rhs[14], lhs[15] - rhs[15],
    ]
}

/// Multiplies a 2x2 matrix by a scalar
///
/// # Examples
///
/// ```
/// use stones::matrix::mat2_mul;
///
/// let m = [1, 2,
///          3, 4];
/// let scalar = 2;
/// assert_eq!(mat2_mul(m, scalar), [2, 4,
///                                  6, 8]);
/// ```
pub fn mat2_mul<T>(lhs: Matrix2<T>, rhs: T) -> Matrix2<T>
    where T: Copy + Mul<Output=T>
{
    [
        lhs[0] * rhs, lhs[1] * rhs,
        lhs[2] * rhs, lhs[3] * rhs
    ]
}

/// Multiplies a 3x3 matrix by a scalar
///
/// # Examples
///
/// ```
/// use stones::matrix::mat3_mul;
///
/// let m = [1, 2, 3,
///          4, 5, 6,
///          7, 8, 9];
/// let scalar = 2;
/// assert_eq!(mat3_mul(m, scalar), [2, 4, 6,
///                                  8, 10, 12,
///                                  14, 16, 18]);
/// ```
pub fn mat3_mul<T>(lhs: Matrix3<T>, rhs: T) -> Matrix3<T>
    where T: Copy + Mul<Output=T>
{
    [
        lhs[0] * rhs, lhs[1] * rhs, lhs[2] * rhs,
        lhs[3] * rhs, lhs[4] * rhs, lhs[5] * rhs,
        lhs[6] * rhs, lhs[7] * rhs, lhs[8] * rhs
    ]
}

/// Multiplies a 4x4 matrix by a scalar
///
/// # Examples
///
/// ```
/// use stones::matrix::mat4_mul;
///
/// let m = [1, 2, 3, 4,
///          5, 6, 7, 8,
///          9, 10, 11, 12,
///          13, 14, 15, 16];
/// let scalar = 2;
/// assert_eq!(mat4_mul(m, scalar), [2, 4, 6, 8,
///                                  10, 12, 14, 16,
///                                  18, 20, 22, 24,
///                                  26, 28, 30, 32]);
/// ```
pub fn mat4_mul<T>(lhs: Matrix4<T>, rhs: T) -> Matrix4<T>
    where T: Copy + Mul<Output=T>
{
    [
        lhs[0] * rhs, lhs[1] * rhs, lhs[2] * rhs, lhs[3] * rhs,
        lhs[4] * rhs, lhs[5] * rhs, lhs[6] * rhs, lhs[7] * rhs,
        lhs[8] * rhs, lhs[9] * rhs, lhs[10] * rhs, lhs[11] * rhs,
        lhs[12] * rhs, lhs[13] * rhs, lhs[14] * rhs, lhs[15] * rhs,
    ]
}