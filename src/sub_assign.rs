use onezero::One;

use blas::{Axpy, MutVector, ToBlasInt, Vector};
use strided;
use traits::{
    SubAssign, Collection, Matrix, MatrixCols, MatrixMutCols, MatrixMutRows, MatrixRows
};
use {Col, Diag, Mat, MutView, Row, Trans, View};

macro_rules! sub_assign {
    ($($lhs:ty $rhs:ty),+,) => {$(
        impl<T, L> SubAssign<T> for $lhs where
            T: Axpy + Neg<T> + One,
            L: MutVector<T>,
        {
            fn sub_assign(&mut self, rhs: &T) {
                vs(&mut self.0, rhs)
            }
        }

        impl<T, L, R> SubAssign<$rhs> for $lhs where
            T: Axpy + Neg<T> + One,
            L: MutVector<T>,
            R: Vector<T>,
        {
            fn sub_assign(&mut self, rhs: &$rhs) {
                vv(&mut self.0, &rhs.0)
            }
        })+
    }
}

sub_assign!{
    Col<L> Col<R>,
    Row<L> Row<R>,
}

impl<'a, T> SubAssign<T> for Diag<strided::MutSlice<'a, T>> where T: Axpy + Neg<T> + One {
    fn sub_assign(&mut self, rhs: &T) {
        vs(&mut self.0, rhs)
    }
}

impl<T> SubAssign<T> for Mat<T> where T: Axpy + Neg<T> + One {
    fn sub_assign(&mut self, rhs: &T) {
        let axpy = Axpy::axpy(None::<T>);
        let n = Collection::len(&self.data).to_blasint();
        let alpha = {
            let _1: T = One::one();
            -_1
        };
        let x = rhs;
        let incx = 0;
        let y = self.data.as_mut_ptr();
        let incy = 1;

        unsafe { axpy(&n, &alpha, x, &incx, y, &incy) }
    }
}

impl<T> SubAssign<T> for Trans<Mat<T>> where T: Axpy + Neg<T> + One {
    fn sub_assign(&mut self, rhs: &T) {
        self.0.sub_assign(rhs)
    }
}

impl<T> SubAssign<Mat<T>> for Mat<T> where T: Axpy + Neg<T> + One {
    fn sub_assign(&mut self, rhs: &Mat<T>) {
        assert_eq!(self.size(), rhs.size());

        let axpy = Axpy::axpy(None::<T>);
        let n = Collection::len(&self.data).to_blasint();
        let alpha = {
            let _1: T = One::one();
            -_1
        };
        let x = rhs.data.as_ptr();
        let incx = 1;
        let y = self.data.as_mut_ptr();
        let incy = 1;

        unsafe { axpy(&n, &alpha, x, &incx, y, &incy) }
    }
}

impl<T> SubAssign<Trans<Mat<T>>> for Trans<Mat<T>> where T: Axpy + Neg<T> + One {
    fn sub_assign(&mut self, rhs: &Trans<Mat<T>>) {
        self.0.sub_assign(&rhs.0)
    }
}

impl<T> SubAssign<Trans<Mat<T>>> for Mat<T> where T: Axpy + Neg<T> + One {
    fn sub_assign(&mut self, rhs: &Trans<Mat<T>>) {
        assert_eq!(self.size(), rhs.size());

        if self.nrows() < self.ncols() {
            for (mut lhs, rhs) in self.mut_rows().zip(rhs.rows()) {
                lhs.sub_assign(&rhs)
            }
        } else {
            for (mut lhs, rhs) in self.mut_cols().zip(rhs.cols()) {
                lhs.sub_assign(&rhs)
            }
        }
    }
}

impl<T> SubAssign<Mat<T>> for Trans<Mat<T>> where T: Axpy + Neg<T> + One {
    fn sub_assign(&mut self, rhs: &Mat<T>) {
        assert_eq!(self.size(), rhs.size());

        if self.nrows() < self.ncols() {
            for (mut lhs, rhs) in self.mut_rows().zip(rhs.rows()) {
                lhs.sub_assign(&rhs)
            }
        } else {
            for (mut lhs, rhs) in self.mut_cols().zip(rhs.cols()) {
                lhs.sub_assign(&rhs)
            }
        }
    }
}

impl<'a, T> SubAssign<T> for MutView<'a, T> where T: Axpy + Neg<T> + One {
    fn sub_assign(&mut self, rhs: &T) {
        if self.nrows() < self.ncols() {
            for mut row in self.mut_rows() {
                row.sub_assign(rhs)
            }
        } else {
            for mut col in self.mut_cols() {
                col.sub_assign(rhs)
            }
        }
    }
}

impl<'a, T> SubAssign<T> for Trans<MutView<'a, T>> where T: Axpy + Neg<T> + One {
    fn sub_assign(&mut self, rhs: &T) {
        self.0.sub_assign(rhs)
    }
}

macro_rules! impls {
    ($($lhs:ty $rhs:ty),+,) => {$(
        impl<'a, T> SubAssign<$rhs> for $lhs where T: Axpy + Neg<T> + One {
            fn sub_assign(&mut self, rhs: &$rhs) {
                assert_eq!(self.size(), rhs.size());

                if self.nrows() < self.ncols() {
                    for (mut lhs, rhs) in self.mut_rows().zip(rhs.rows()) {
                        lhs.sub_assign(&rhs)
                    }
                } else {
                    for (mut lhs, rhs) in self.mut_cols().zip(rhs.cols()) {
                        lhs.sub_assign(&rhs)
                    }
                }
            }
        })+
     }
}

impls!{
    MutView<'a, T> Mat<T>,
    MutView<'a, T> Trans<Mat<T>>,
    Trans<MutView<'a, T>> Mat<T>,
    Trans<MutView<'a, T>> Trans<Mat<T>>,
}

macro_rules! view {
    ($($ty:ty),+) => {$(
        impl<'a, T> SubAssign<$ty> for Mat<T> where T: Axpy + Neg<T> + One {
            fn sub_assign(&mut self, rhs: &$ty) {
                assert_eq!(self.size(), rhs.size());

                if self.nrows() < self.ncols() {
                    for (mut lhs, rhs) in self.mut_rows().zip(rhs.rows()) {
                        lhs.sub_assign(&rhs)
                    }
                } else {
                    for (mut lhs, rhs) in self.mut_cols().zip(rhs.cols()) {
                        lhs.sub_assign(&rhs)
                    }
                }
            }
        }

        impl<'a, T> SubAssign<Trans<$ty>> for Mat<T> where T: Axpy + Neg<T> + One {
            fn sub_assign(&mut self, rhs: &Trans<$ty>) {
                assert_eq!(self.size(), rhs.size());

                if self.nrows() < self.ncols() {
                    for (mut lhs, rhs) in self.mut_rows().zip(rhs.rows()) {
                        lhs.sub_assign(&rhs)
                    }
                } else {
                    for (mut lhs, rhs) in self.mut_cols().zip(rhs.cols()) {
                        lhs.sub_assign(&rhs)
                    }
                }
            }
        }

        impl<'a, T> SubAssign<$ty> for Trans<Mat<T>> where T: Axpy + Neg<T> + One {
            fn sub_assign(&mut self, rhs: &$ty) {
                assert_eq!(self.size(), rhs.size());

                if self.nrows() < self.ncols() {
                    for (mut lhs, rhs) in self.mut_rows().zip(rhs.rows()) {
                        lhs.sub_assign(&rhs)
                    }
                } else {
                    for (mut lhs, rhs) in self.mut_cols().zip(rhs.cols()) {
                        lhs.sub_assign(&rhs)
                    }
                }
            }
        }

        impl<'a, T> SubAssign<Trans<$ty>> for Trans<Mat<T>> where T: Axpy + Neg<T> + One {
            fn sub_assign(&mut self, rhs: &Trans<$ty>) {
                assert_eq!(self.size(), rhs.size());

                if self.nrows() < self.ncols() {
                    for (mut lhs, rhs) in self.mut_rows().zip(rhs.rows()) {
                        lhs.sub_assign(&rhs)
                    }
                } else {
                    for (mut lhs, rhs) in self.mut_cols().zip(rhs.cols()) {
                        lhs.sub_assign(&rhs)
                    }
                }
            }
        }

        impl<'a, 'b, T> SubAssign<$ty> for MutView<'b, T> where T: Axpy + Neg<T> + One {
            fn sub_assign(&mut self, rhs: &$ty) {
                assert_eq!(self.size(), rhs.size());

                if self.nrows() < self.ncols() {
                    for (mut lhs, rhs) in self.mut_rows().zip(rhs.rows()) {
                        lhs.sub_assign(&rhs)
                    }
                } else {
                    for (mut lhs, rhs) in self.mut_cols().zip(rhs.cols()) {
                        lhs.sub_assign(&rhs)
                    }
                }
            }
        }

        impl<'a, 'b, T> SubAssign<Trans<$ty>> for MutView<'b, T> where T: Axpy + Neg<T> + One {
            fn sub_assign(&mut self, rhs: &Trans<$ty>) {
                assert_eq!(self.size(), rhs.size());

                if self.nrows() < self.ncols() {
                    for (mut lhs, rhs) in self.mut_rows().zip(rhs.rows()) {
                        lhs.sub_assign(&rhs)
                    }
                } else {
                    for (mut lhs, rhs) in self.mut_cols().zip(rhs.cols()) {
                        lhs.sub_assign(&rhs)
                    }
                }
            }
        }

        impl<'a, 'b, T> SubAssign<$ty> for Trans<MutView<'b, T>> where T: Axpy + Neg<T> + One {
            fn sub_assign(&mut self, rhs: &$ty) {
                assert_eq!(self.size(), rhs.size());

                if self.nrows() < self.ncols() {
                    for (mut lhs, rhs) in self.mut_rows().zip(rhs.rows()) {
                        lhs.sub_assign(&rhs)
                    }
                } else {
                    for (mut lhs, rhs) in self.mut_cols().zip(rhs.cols()) {
                        lhs.sub_assign(&rhs)
                    }
                }
            }
        }

        impl<'a, 'b, T> SubAssign<Trans<$ty>> for Trans<MutView<'b, T>> where
            T: Axpy + Neg<T> + One
        {
            fn sub_assign(&mut self, rhs: &Trans<$ty>) {
                assert_eq!(self.size(), rhs.size());

                if self.nrows() < self.ncols() {
                    for (mut lhs, rhs) in self.mut_rows().zip(rhs.rows()) {
                        lhs.sub_assign(&rhs)
                    }
                } else {
                    for (mut lhs, rhs) in self.mut_cols().zip(rhs.cols()) {
                        lhs.sub_assign(&rhs)
                    }
                }
            }
        })+
    }
}

view!(View<'a, T>, MutView<'a, T>)

fn vs<T, V: MutVector<T>>(lhs: &mut V, rhs: &T) where T: Axpy + Neg<T> + One {
    let axpy = Axpy::axpy(None::<T>);
    let n = Vector::len(lhs);
    let alpha = {
        let _1: T = One::one();
        -_1
    };
    let x = rhs;
    let incx = 0;
    let y = lhs.as_mut_ptr();
    let incy = lhs.stride();

    unsafe { axpy(&n, &alpha, x, &incx, y, &incy) }
}

fn vv<T, L: MutVector<T>, R: Vector<T>>(lhs: &mut L, rhs: &R) where T: Axpy + Neg<T> + One {
    assert_eq!(Collection::len(lhs), Collection::len(rhs))

    let axpy = Axpy::axpy(None::<T>);
    let n = Vector::len(lhs);
    let alpha = {
        let _1: T = One::one();
        -_1
    };
    let x = rhs.as_ptr();
    let incx = rhs.stride();
    let y = lhs.as_mut_ptr();
    let incy = lhs.stride();

    unsafe { axpy(&n, &alpha, x, &incx, y, &incy) }
}