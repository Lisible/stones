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

/// Trait for getting the 0 value of the type implementing the trait
pub trait Zero {
    fn zero() -> Self;
}

impl Zero for i32 {
    fn zero() -> Self {
        0i32
    }
}

impl Zero for i64 {
    fn zero() -> Self {
        0i64
    }
}

impl Zero for f32 {
    fn zero() -> Self {
        0f32
    }
}

impl Zero for f64 {
    fn zero() -> Self {
        0f64
    }
}


/// Trait for getting the 1 value of the type implementing the trait
pub trait One {
    fn one() -> Self;
}

impl One for i32 {
    fn one() -> Self {
        1i32
    }
}

impl One for i64 {
    fn one() -> Self {
        1i64
    }
}

impl One for f32 {
    fn one() -> Self {
        1f32
    }
}

impl One for f64 {
    fn one() -> Self {
        1f64
    }
}