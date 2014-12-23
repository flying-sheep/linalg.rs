use {Col, ColVec, Mat, MutCol, MutRow, MutView, Row, RowVec, Scaled, Trans, View};

// col
impl<'a, T> Mul<T, Scaled<T, Col<'a, T>>> for Col<'a, T> {
    fn mul(self, rhs: T) -> Scaled<T, Col<'a, T>> {
        Scaled(rhs, self)
    }
}

impl<'a, T> Mul<Col<'a, T>, Scaled<T, Col<'a, T>>> for T {
    fn mul(self, rhs: Col<'a, T>) -> Scaled<T, Col<'a, T>> {
        rhs * self
    }
}

impl<'a, T> Mul<T, Scaled<T, Col<'a, T>>> for &'a ColVec<T> {
    fn mul(self, rhs: T) -> Scaled<T, Col<'a, T>> {
        Scaled(rhs, self.as_col())
    }
}

impl<'a, T> Mul<&'a ColVec<T>, Scaled<T, Col<'a, T>>> for T {
    fn mul(self, rhs: &'a ColVec<T>) -> Scaled<T, Col<'a, T>> {
        rhs * self
    }
}

impl<'a, 'b, T> Mul<T, Scaled<T, Col<'a, T>>> for &'a MutCol<'b, T> {
    fn mul(self, rhs: T) -> Scaled<T, Col<'a, T>> {
        Scaled(rhs, self.as_col())
    }
}

impl<'a, 'b, T> Mul<&'a MutCol<'b, T>, Scaled<T, Col<'a, T>>> for T {
    fn mul(self, rhs: &'a MutCol<'b, T>) -> Scaled<T, Col<'a, T>> {
        rhs * self
    }
}

// mat
impl<'a, T> Mul<T, Scaled<T, View<'a, T>>> for &'a Mat<T> {
    fn mul(self, rhs: T) -> Scaled<T, View<'a, T>> {
        Scaled(rhs, self.as_view())
    }
}

impl<'a, T> Mul<&'a Mat<T>, Scaled<T, View<'a, T>>> for T {
    fn mul(self, rhs: &'a Mat<T>) -> Scaled<T, View<'a, T>> {
        rhs * self
    }
}

impl<'a, 'b, T> Mul<T, Scaled<T, View<'a, T>>> for &'a MutView<'b, T> {
    fn mul(self, rhs: T) -> Scaled<T, View<'a, T>> {
        Scaled(rhs, self.as_view())
    }
}

impl<'a, 'b, T> Mul<&'a MutView<'b, T>, Scaled<T, View<'a, T>>> for T {
    fn mul(self, rhs: &'a MutView<'b, T>) -> Scaled<T, View<'a, T>> {
        rhs * self
    }
}

impl<'a, T> Mul<T, Scaled<T, Trans<View<'a, T>>>> for &'a Trans<Mat<T>> {
    fn mul(self, rhs: T) -> Scaled<T, Trans<View<'a, T>>> {
        Scaled(rhs, Trans(self.0.as_view()))
    }
}

impl<'a, T> Mul<&'a Trans<Mat<T>>, Scaled<T, Trans<View<'a, T>>>> for T {
    fn mul(self, rhs: &'a Trans<Mat<T>>) -> Scaled<T, Trans<View<'a, T>>> {
        rhs * self
    }
}

impl<'a, 'b, T> Mul<T, Scaled<T, Trans<View<'a, T>>>> for &'a Trans<MutView<'b, T>> {
    fn mul(self, rhs: T) -> Scaled<T, Trans<View<'a, T>>> {
        Scaled(rhs, Trans(self.0.as_view()))
    }
}

impl<'a, 'b, T> Mul<&'a Trans<MutView<'b, T>>, Scaled<T, Trans<View<'a, T>>>> for T {
    fn mul(self, rhs: &'a Trans<MutView<'b, T>>) -> Scaled<T, Trans<View<'a, T>>> {
        rhs * self
    }
}

impl<'a, T> Mul<T, Scaled<T, Trans<View<'a, T>>>> for Trans<View<'a, T>> {
    fn mul(self, rhs: T) -> Scaled<T, Trans<View<'a, T>>> {
        Scaled(rhs, self)
    }
}

impl<'a, T> Mul<Trans<View<'a, T>>, Scaled<T, Trans<View<'a, T>>>> for T {
    fn mul(self, rhs: Trans<View<'a, T>>) -> Scaled<T, Trans<View<'a, T>>> {
        rhs * self
    }
}
impl<'a, T> Mul<T, Scaled<T, View<'a, T>>> for View<'a, T> {
    fn mul(self, rhs: T) -> Scaled<T, View<'a, T>> {
        Scaled(rhs, self)
    }
}

impl<'a, T> Mul<View<'a, T>, Scaled<T, View<'a, T>>> for T {
    fn mul(self, rhs: View<'a, T>) -> Scaled<T, View<'a, T>> {
        rhs * self
    }
}

// row
impl<'a, T> Mul<T, Scaled<T, Row<'a, T>>> for Row<'a, T> {
    fn mul(self, rhs: T) -> Scaled<T, Row<'a, T>> {
        Scaled(rhs, self)
    }
}

impl<'a, T> Mul<Row<'a, T>, Scaled<T, Row<'a, T>>> for T {
    fn mul(self, rhs: Row<'a, T>) -> Scaled<T, Row<'a, T>> {
        rhs * self
    }
}

impl<'a, T> Mul<T, Scaled<T, Row<'a, T>>> for &'a RowVec<T> {
    fn mul(self, rhs: T) -> Scaled<T, Row<'a, T>> {
        Scaled(rhs, self.as_row())
    }
}

impl<'a, T> Mul<&'a RowVec<T>, Scaled<T, Row<'a, T>>> for T {
    fn mul(self, rhs: &'a RowVec<T>) -> Scaled<T, Row<'a, T>> {
        rhs * self
    }
}

impl<'a, 'b, T> Mul<T, Scaled<T, Row<'a, T>>> for &'a MutRow<'b, T> {
    fn mul(self, rhs: T) -> Scaled<T, Row<'a, T>> {
        Scaled(rhs, self.as_row())
    }
}

impl<'a, 'b, T> Mul<&'a MutRow<'b, T>, Scaled<T, Row<'a, T>>> for T {
    fn mul(self, rhs: &'a MutRow<'b, T>) -> Scaled<T, Row<'a, T>> {
        rhs * self
    }
}
