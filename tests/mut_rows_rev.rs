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

    // Test that `mut_rows().rev()` is correct for `Trans<Mat>`
    #[quickcheck]
    fn mat((nrows, ncols): (uint, uint), col: uint) -> TestResult {
        enforce! {
            col < ncols,
        }

        test!({
            let mut m = setup::mat((ncols, nrows)).t();

            let n = m.nrows();

            m.mut_rows().rev().enumerate().all(|(i, r)| {
                let i = n - i - 1;

                r.at(col).unwrap() == &(col, i)
            })
        })
    }

    // Test that `mut_rows().rev()` is correct for `Trans<MutView>`
    #[quickcheck]
    fn view_mut(
        start: (uint, uint),
        (nrows, ncols): (uint, uint),
        col: uint,
    ) -> TestResult {
        enforce! {
            col < ncols,
        }

        let size = (start.0 + ncols, start.1 + nrows);
        test!({
            let mut m = setup::mat(size);
            let mut v = try!(m.slice_from_mut(start)).t();
            let (start_row, start_col) = start;
            let n = v.nrows();

            v.mut_rows().rev().enumerate().all(|(i, r)| {
                let i = n - i - 1;

                r.at(col).unwrap() == &(start_row + col, start_col + i)
            })
        })
    }
}

// Test that `mut_rows().rev()` is correct for `Mat`
#[quickcheck]
fn mat((nrows, ncols): (uint, uint), col: uint) -> TestResult {
    enforce! {
        col < ncols,
    }

    test!({
        let mut m = setup::mat((nrows, ncols));
        let n = m.nrows();

        m.mut_rows().rev().enumerate().all(|(i, r)| {
            let i = n - i - 1;

            r.at(col).unwrap() == &(i, col)
        })
    })
}

// Test that `mut_rows().rev()` is correct for `MutView`
#[quickcheck]
fn view_mut(
    start: (uint, uint),
    (nrows, ncols): (uint, uint),
    col: uint,
) -> TestResult {
    enforce! {
        col < ncols,
    }

    let size = (start.0 + nrows, start.1 + ncols);
    test!({
        let mut m = setup::mat(size);
        let mut v = try!(m.slice_from_mut(start));
        let (start_row, start_col) = start;
        let n = v.nrows();

        v.mut_rows().rev().enumerate().all(|(i, r)| {
            let i = n - i - 1;

            r.at(col).unwrap() == &(start_row + i, start_col + col)
        })
    })
}
