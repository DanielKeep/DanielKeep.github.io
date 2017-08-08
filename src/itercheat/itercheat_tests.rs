// cargo-deps: itertools
extern crate itertools;
use std::cmp::Ordering as O;
use itertools::Itertools;
use itertools::assert_equal as iae;
use itertools::MinMaxResult;

fn main() {
    iae((0..4).interleave(vec![8, 9]), vec![0, 8, 1, 9, 2, 3]);
    iae((0..0).interleave(vec![8, 9]), vec![8, 9]);

    assert_eq!((0..4).cmp(0..5), O::Less);
    assert_eq!((0..4).cmp(0..4), O::Equal);
    assert_eq!((0..5).cmp(0..4), O::Greater);

    {
        let a = vec![0.0f32].into_iter();
        let b = vec![0.0, 1.0].into_iter();
        assert_eq!(a.partial_cmp(b), Some(O::Less));
    }
    {
        let a = vec![0.0f32, 1.0].into_iter();
        let b = vec![0.0, 1.0].into_iter();
        assert_eq!(a.partial_cmp(b), Some(O::Equal));
    }
    {
        let a = vec![0.0f32, std::f32::NAN].into_iter();
        let b = vec![0.0, 1.0].into_iter();
        assert_eq!(a.partial_cmp(b), None);
    }

    {
        let s = String::from;
        let a = vec![s("c"), s("a")];
        assert_eq!(a.into_iter().minmax(), MinMaxResult::MinMax(s("a"), s("c")));
    }

    println!("Ok.");
}
