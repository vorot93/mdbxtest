use std::path::Path;

use libmdbx::{Environment, NoWriteMap, WriteFlags, Geometry};

pub fn get_key(n: u64) -> String {
    format!("key{}", n)
}

pub fn get_data(n: u64) -> String {
    format!("data{}", n)
}

pub fn setup_bench_db(dir: &Path, n: u64) -> Environment<NoWriteMap> {
    let env = Environment::new()
        .set_geometry(Geometry {
            size: Some(0..1024 * 1024 * 1024 * 16),
            growth_step: Some(1024 * 1024 * 1024),
            shrink_threshold: None,
            page_size: None,
        })
        .open(dir).unwrap();

    let txn = env.begin_rw_txn().unwrap();
    let db = txn.open_db(None).unwrap();
    for i in 0..n {
        txn.put(&db, &get_key(i), &get_data(i), WriteFlags::empty())
            .unwrap();
        if i % 10000 == 0 {
            println!("{} keys inserted", i);
        }
    }
    txn.commit().unwrap();
    env
}

fn main() {
    let dir = Path::new("test");
    let env = setup_bench_db(dir, 100_000_000);
    println!("Dir {:?}", dir);
}
