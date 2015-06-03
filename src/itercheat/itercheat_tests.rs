// cargo-deps: itertools
extern crate itertools;
use itertools::Itertools;

fn main() {
    use itertools::assert_equal as iae;

    iae((0..4).interleave(vec![8, 9]), vec![0, 8, 1, 9, 2, 3]);
    iae((0..0).interleave(vec![8, 9]), vec![8, 9]);

    println!("Ok.");
}
