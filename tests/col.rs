#![feature(globs, macro_rules, phase)]

extern crate linalg;
extern crate quickcheck;
#[phase(plugin)]
extern crate quickcheck_macros;

use linalg::prelude::*;
use quickcheck::TestResult;

mod setup;

mod trans {
    use linalg::prelude::*;
    use quickcheck::TestResult;

    use setup;

    // Test that `col(_)` is correct for `Trans<Mat>`
    #[quickcheck]
    fn mat((nrows, ncols): (uint, uint), (row, col): (uint, uint)) -> TestResult {
        enforce! {
            row < nrows,
            col < ncols,
        }

        test!({
            let m = setup::mat((ncols, nrows)).t();
            let c = try!(m.col(col));
            let &e = try!(c.at(row));

            e == (col, row)
        })
    }

    // Test that `col(_)` is correct for `Trans<View>`
    #[quickcheck]
    fn view(
        start: (uint, uint),
        (nrows, ncols): (uint, uint),
        (row, col): (uint, uint),
    ) -> TestResult {
        enforce! {
            row < nrows,
            col < ncols,
        }

        let size = (start.0 + ncols, start.1 + nrows);
        test!({
            let m = setup::mat(size);
            let v = try!(m.slice_from(start)).t();
            let c = try!(v.col(col));
            let &e = try!(c.at(row));
            let (start_row, start_col) = start;

            e == (start_row + col, start_col + row)
        })
    }

    // Test that `col(_)` is correct for `Trans<MutView>`
    #[quickcheck]
    fn view_mut(
        start: (uint, uint),
        (nrows, ncols): (uint, uint),
        (row, col): (uint, uint),
    ) -> TestResult {
        enforce! {
            row < nrows,
            col < ncols,
        }

        let size = (start.0 + ncols, start.1 + nrows);
        test!({
            let mut m = setup::mat(size);
            let v = try!(m.slice_from_mut(start)).t();
            let c = try!(v.col(col));
            let &e = try!(c.at(row));
            let (start_row, start_col) = start;

            e == (start_row + col, start_col + row)
        })
    }
}

// Test that `col(_)` is correct for `Mat`
#[quickcheck]
fn mat((nrows, ncols): (uint, uint), (row, col): (uint, uint)) -> TestResult {
    enforce! {
        row < nrows,
        col < ncols,
    }

    test!({
        let m = setup::mat((nrows, ncols));
        let c = try!(m.col(col));
        let &e = try!(c.at(row));

        e == (row, col)
    })
}

// Test that `col(_)` is correct for `View`
#[quickcheck]
fn view(
    start: (uint, uint),
    (nrows, ncols): (uint, uint),
    (row, col): (uint, uint),
) -> TestResult {
    enforce! {
        row < nrows,
        col < ncols,
    }

    let size = (start.0 + nrows, start.1 + ncols);
    test!({
        let m = setup::mat(size);
        let v = try!(m.slice_from(start));
        let c = try!(v.col(col));
        let &e = try!(c.at(row));
        let (start_row, start_col) = start;

        e == (start_row + row, start_col + col)
    })
}

// Test that `col(_)` is correct for `MutView`
#[quickcheck]
fn view_mut(
    start: (uint, uint),
    (nrows, ncols): (uint, uint),
    (row, col): (uint, uint),
) -> TestResult {
    enforce! {
        row < nrows,
        col < ncols,
    }

    let size = (start.0 + nrows, start.1 + ncols);
    test!({
        let mut m = setup::mat(size);
        let v = try!(m.slice_from_mut(start));
        let c = try!(v.col(col));
        let &e = try!(c.at(row));
        let (start_row, start_col) = start;

        e == (start_row + row, start_col + col)
    })
}
