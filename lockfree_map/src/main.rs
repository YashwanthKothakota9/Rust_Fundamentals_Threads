use dashmap::DashMap;
use once_cell::sync::Lazy;
use std::time::Duration;

static SHARED_MAP: Lazy<DashMap<u32, u32>> = Lazy::new(DashMap::new);

fn main() {
    for n in 0..5 {
        std::thread::spawn(move || loop {
            if let Some(mut v) = SHARED_MAP.get_mut(&n) {
                *v += 1;
            } else {
                SHARED_MAP.insert(n, n);
            }
        });
    }
    std::thread::sleep(Duration::from_secs(5));
    println!("{SHARED_MAP:#?}");
}
