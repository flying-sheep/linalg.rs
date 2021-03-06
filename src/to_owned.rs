use std::mem;

use blas::copy::Copy;
use traits::{Matrix, MatrixCols, MatrixRows, ToOwned};
use {ToBlasint, Col, ColVec, Mat, MutCol, MutRow, MutView, Row, RowVec, Trans, View};

fn to<'a, T>(s: ::raw::strided::Slice<'a, T>) -> Box<[T]> where T: Copy {
    let n = s.len();

    if n == 0 { return box [] }

    let copy = Copy::copy(None::<T>);
    let x = s.data;
    let incx = s.stride.to_blasint();
    let mut data = Vec::with_capacity(n);
    let y = data.as_mut_ptr();
    let incy = 1;

    unsafe {
        copy(&n.to_blasint(), x, &incx, y, &incy);
        data.set_len(n)
    }

    data.into_boxed_slice()
}

impl<'a, T> ToOwned<ColVec<T>> for Col<'a, T> where T: Copy {
    fn to_owned(&self) -> ColVec<T> {
        ColVec(to((self.0).0))
    }
}

impl<T> ToOwned<ColVec<T>> for ColVec<T> where T: Clone {
    fn to_owned(&self) -> ColVec<T> {
        self.clone()
    }
}

impl<T> ToOwned<Mat<T>> for Mat<T> where T: Clone {
    fn to_owned(&self) -> Mat<T> {
        self.clone()
    }
}

impl<'a, T> ToOwned<ColVec<T>> for MutCol<'a, T> where T: Copy {
    fn to_owned(&self) -> ColVec<T> {
        ColVec(to((self.0).0))
    }
}

impl<'a, T> ToOwned<RowVec<T>> for MutRow<'a, T> where T: Copy {
    fn to_owned(&self) -> RowVec<T> {
        RowVec(to((self.0).0))
    }
}

impl<'a, T> ToOwned<Mat<T>> for MutView<'a, T> where T: Copy {
    fn to_owned(&self) -> Mat<T> {
        unsafe { mem::transmute::<_, &View<_>>(self) }.to_owned()
    }
}

impl<'a, T> ToOwned<RowVec<T>> for Row<'a, T> where T: Copy {
    fn to_owned(&self) -> RowVec<T> {
        RowVec(to((self.0).0))
    }
}

impl<T> ToOwned<RowVec<T>> for RowVec<T> where T: Clone {
    fn to_owned(&self) -> RowVec<T> {
        self.clone()
    }
}

impl<T> ToOwned<Mat<T>> for Trans<Mat<T>> where T: Copy {
    fn to_owned(&self) -> Mat<T> {
        Trans(self.0.as_view()).to_owned()
    }
}

impl<'a, T> ToOwned<Mat<T>> for Trans<MutView<'a, T>> where T: Copy {
    fn to_owned(&self) -> Mat<T> {
        unsafe { mem::transmute::<_, &Trans<View<_>>>(self) }.to_owned()
    }
}

impl<'a, T> ToOwned<Mat<T>> for Trans<View<'a, T>> where T: Copy {
    fn to_owned(&self) -> Mat<T> {
        let (nrows, ncols) = (self.nrows(), self.ncols());

        let n = nrows * ncols;

        let data = if n == 0 {
            let b: Box<[_]> = box [];

            b
        } else {
            let mut data = Vec::with_capacity(n);

            let copy = Copy::copy(None::<T>);

            if nrows < ncols {
                let n = ncols.to_blasint();
                let incy = nrows.to_blasint();

                for (i, row) in self.rows().enumerate() {
                    let slice = (row.0).0;

                    let x = slice.data;
                    let incx = slice.stride.to_blasint();
                    let y = unsafe { data.as_mut_ptr().offset(i as int) };

                    unsafe { copy(&n, x, &incx, y, &incy) }
                }
            } else {
                let n = self.nrows().to_blasint();
                let incy = 1;

                for (i, col) in self.cols().enumerate() {
                    let slice = (col.0).0;

                    let x = slice.data;
                    let incx = slice.stride.to_blasint();
                    let y = unsafe { data.as_mut_ptr().offset((i * self.nrows()) as int) };

                    unsafe { copy(&n, x, &incx, y, &incy) }
                }
            }

            unsafe { data.set_len(n) }

            data.into_boxed_slice()
        };

        Mat {
            data: data,
            ncols: ncols,
            nrows: nrows,
        }
    }
}

impl<'a, T> ToOwned<Mat<T>> for View<'a, T> where T: Copy {
    fn to_owned(&self) -> Mat<T> {
        let (nrows, ncols) = (self.nrows(), self.ncols());

        let n = nrows * ncols;

        let data = if n == 0 {
            let b: Box<[_]> = box [];

            b
        } else {
            let mut data = Vec::with_capacity(n);

            let copy = Copy::copy(None::<T>);

            if nrows < ncols {
                let n = ncols.to_blasint();
                let incy = nrows.to_blasint();

                for (i, row) in self.rows().enumerate() {
                    let slice = (row.0).0;

                    let x = slice.data;
                    let incx = slice.stride.to_blasint();
                    let y = unsafe { data.as_mut_ptr().offset(i as int) };

                    unsafe { copy(&n, x, &incx, y, &incy) }
                }
            } else {
                let n = self.nrows().to_blasint();
                let incy = 1;

                for (i, col) in self.cols().enumerate() {
                    let slice = (col.0).0;

                    let x = slice.data;
                    let incx = slice.stride.to_blasint();
                    let y = unsafe { data.as_mut_ptr().offset((i * self.nrows()) as int) };

                    unsafe { copy(&n, x, &incx, y, &incy) }
                }
            }

            unsafe { data.set_len(n) }

            data.into_boxed_slice()
        };

        Mat {
            data: data,
            ncols: ncols,
            nrows: nrows,
        }
    }
}
