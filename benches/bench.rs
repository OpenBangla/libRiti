#![feature(test)]

extern crate test;

use test::{Bencher, black_box};
use libRiti::phonetic::database::Database;

#[bench]
fn bench_a(b: &mut Bencher) {
    b.iter(|| {
        let database = Database::new();
        let res = database.search_dictionary("a");
        black_box(res);
    })
}

#[bench]
fn bench_aro(b: &mut Bencher) {
    b.iter(|| {
        let database = Database::new();
        let res = database.search_dictionary("arO");
        black_box(res);
    })
}

#[bench]
fn bench_bistari(b: &mut Bencher) {
    b.iter(|| {
        let database = Database::new();
        let res = database.search_dictionary("bistari");
        black_box(res);
    })
}
