use libmdbx::*;
use std::{path::Path, time::SystemTime};

pub fn get_key(n: u32) -> String {
    format!("key{}", hex::encode(n.to_be_bytes()))
}

pub fn get_data(n: u32) -> String {
    format!("data{}", hex::encode(n.to_be_bytes()))
}

pub fn setup_bench_db(dir: &Path, nums: &[(String, String)]) -> Environment<WriteMap> {
    let env = Environment::new()
        .set_geometry(Geometry {
            size: Some(0..1024 * 1024 * 1024 * 1024),
            growth_step: Some(1024 * 1024 * 1024 * 4),
            shrink_threshold: None,
            page_size: None,
        })
        .set_max_dbs(1)
        .set_flags(EnvironmentFlags {
            // no_rdahead: true,
            // liforeclaim: true,
            ..Default::default()
        })
        .open(dir)
        .unwrap();

    let mut txn = env.begin_rw_txn().unwrap();
    let mut db = txn
        .create_db(Some("hello"), DatabaseFlags::default())
        .unwrap();
    let mut cursor = txn.cursor(&db).unwrap();
    for (i, (key, value)) in nums.iter().enumerate() {
        cursor
            .put(key.as_bytes(), value.as_bytes(), WriteFlags::default())
            .unwrap();
        if i % 10000 == 0 {
            println!("{} keys inserted", i);
        }
    }
    txn.commit().unwrap();
    env
}

fn main() {
    let dir = tempfile::tempdir_in(Path::new(".")).unwrap();
    println!("Generating test set...");
    // let nums = std::iter::repeat_with(rand::random).take(100_000_000);
    let nums = 0..100_000_000;
    let kv = nums
        .into_iter()
        .map(|i| (get_key(i), get_data(i)))
        .collect::<Vec<_>>();
    let start = SystemTime::now();
    let _env = setup_bench_db(dir.path(), &kv);
    println!("Elapsed: {:?}", start.elapsed().unwrap());
    println!("Dir {:?}", dir);
}
