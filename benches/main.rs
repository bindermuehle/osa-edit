#![feature(test)]
extern crate test;

//use osa_edit::edit_script_for_strings;
use test::Bencher;

#[bench]
fn bench(b: &mut Bencher) {
    let lines: Vec<&str> = include_str!("./sample.txt").split('\n').collect();
    b.iter(|| {
        let mut last_value = "";
        for line in &lines {
            // edit_script_for_strings(last_value, line, osa_edit::DEFAULT_OPTIONS);
            last_value = line;
        }
    });
}
