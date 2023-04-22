#![feature(test)]
extern crate test;

use osa_edit::get_script;
use test::Bencher;

#[bench]
fn bench_vec(b: &mut Bencher) {
    let lines: Vec<&str> = include_str!("./sample.txt").split('\n').collect();
    b.iter(|| {
        let mut last_value = "";
        for line in &lines {
            get_script::<osa_edit::vec::VecMatrix>(last_value, line, osa_edit::DEFAULT_OPTIONS);
            last_value = line;
        }
    });
}
#[bench]
fn bench_grid(b: &mut Bencher) {
    let lines: Vec<&str> = include_str!("./sample.txt").split('\n').collect();
    b.iter(|| {
        let mut last_value = "";
        for line in &lines {
            get_script::<osa_edit::grid::GridMatrix>(last_value, line, osa_edit::DEFAULT_OPTIONS);
            last_value = line;
        }
    });
}
