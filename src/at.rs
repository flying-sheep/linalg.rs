use std::mem;

use traits::{At, AtMut, Matrix};
use {Col, Diag, Mat, MutView, Row, View};

// FIXME (DRY) Merge these two impls via a macro
impl<T> At<uint, T> for [T] {
    fn at(&self, index: uint) -> ::Result<&T> {
        if index < self.len() {
            Ok(unsafe { mem::transmute(self.as_ptr().offset(index as int)) })
        } else {
            Err(::OutOfBounds)
        }
    }
}

impl<T> AtMut<uint, T> for [T] {
    fn at_mut(&mut self, index: uint) -> ::Result<&mut T> {
        if index < self.len() {
            Ok(unsafe { mem::transmute(self.as_ptr().offset(index as int)) })
        } else {
            Err(::OutOfBounds)
        }
    }
}

impl<T> At<uint, T> for Box<[T]> {
    fn at(&self, index: uint) -> ::Result<&T> { At::at(&**self, index) }
}

impl<T> AtMut<uint, T> for Box<[T]> {
    fn at_mut(&mut self, index: uint) -> ::Result<&mut T> { AtMut::at_mut(&mut **self, index) }
}

macro_rules! impl_at {
    ($($ty:ty),+) => {$(
        impl<'a, T> At<uint, T> for $ty {
            fn at(&self, index: uint) -> ::Result<&T> {
                At::at(*self, index)
            }
        })+
    }
}

impl_at!(&'a [T], &'a mut [T])

impl<'a, T> AtMut<uint, T> for &'a mut [T] {
    fn at_mut(&mut self, index: uint) -> ::Result<&mut T> {
        AtMut::at_mut(*self, index)
    }
}

macro_rules! impls {
    ($($ty:ty),+) => {$(
        impl<T, V> At<uint, T> for $ty where V: At<uint, T> {
            fn at(&self, index: uint) -> ::Result<&T> {
                self.0.at(index)
            }
        }

        impl<T, V> AtMut<uint, T> for $ty where V: AtMut<uint, T> {
            fn at_mut(&mut self, index: uint) -> ::Result<&mut T> {
                self.0.at_mut(index)
            }
        })+
    }
}

impls!(Col<V>, Diag<V>, Row<V>)

// FIXME (DRY) Merge these two impls via a macro
impl<T> At<(uint, uint), T> for Mat<T> {
    fn at(&self, (row, col): (uint, uint)) -> ::Result<&T> {
        let (nrows, ncols) = self.size();

        if row < nrows && col < ncols {
            Ok(unsafe {
                mem::transmute(self.data.as_ptr().offset((col * nrows + row) as int))
            })
        } else {
            Err(::OutOfBounds)
        }
    }
}

impl<T> AtMut<(uint, uint), T> for Mat<T> {
    fn at_mut(&mut self, (row, col): (uint, uint)) -> ::Result<&mut T> {
        let (nrows, ncols) = self.size();

        if row < nrows && col < ncols {
            Ok(unsafe {
                mem::transmute(self.data.as_ptr().offset((col * nrows + row) as int))
            })
        } else {
            Err(::OutOfBounds)
        }
    }
}

impl<'a, T> AtMut<(uint, uint), T> for MutView<'a, T> {
    fn at_mut(&mut self, (row, col): (uint, uint)) -> ::Result<&mut T> {
        let (nrows, ncols) = self.size();

        if row < nrows && col < ncols {
            Ok(unsafe { mem::transmute(self.ptr.offset((col * self.stride + row) as int)) })
        } else {
            Err(::OutOfBounds)
        }
    }
}

macro_rules! view {
    ($($ty:ty),+) => {$(
        impl<'a, T> At<(uint, uint), T> for $ty {
            fn at(&self, (row, col): (uint, uint)) -> ::Result<&T> {
                let (nrows, ncols) = self.size();

                if row < nrows && col < ncols {
                    Ok(unsafe {
                        mem::transmute(self.ptr.offset((col * self.stride + row) as int))
                    })
                } else {
                    Err(::OutOfBounds)
                }
            }
        })+
    }
}

view!(View<'a, T>, MutView<'a, T>)