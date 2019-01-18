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