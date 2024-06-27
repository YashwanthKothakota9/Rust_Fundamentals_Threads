fn main() {
    // Let's explicitly size our thread pool
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .unwrap();

    pool.spawn(|| println!("Hello from pool thread"));

    pool.scope(|scope| {
        for n in 0..20 {
            scope.spawn(move |_| {
                println!("Hello from thread {n}");
            });
        }
    });
    println!("Hello from main thread");
}

fn nested_pools() {
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .unwrap();

    // We're using a scope to ensure that we wait for everything to finish
    pool.scope(|scope| {
        for n in 0..4 {
            scope.spawn(move |_scope| {
                println!("Hello from top-level {n}");
                let pool = rayon::ThreadPoolBuilder::new()
                    .num_threads(4)
                    .build()
                    .unwrap();

                pool.scope(|scope| {
                    for inner_n in 0..4 {
                        scope.spawn(move |_scope| {
                            println!("Hello from inner {inner_n} (part of {n})");
                        });
                    }
                });

                println!("Goodbye from top-level {n}");
            });
        }
    });
}

fn rayon_broadcast() {
    // Let's explicitly size our thread pool
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .unwrap();

    pool.scope(|scope| {
        scope.spawn_broadcast(|_scope, broadcast_context| {
            // You can use scope just like the parent scope
            println!("Hello from broadcast thread {}", broadcast_context.index());
        });
    });
}

// fn test() {
//     println!("Hello from test thread");
// }

// fn main() {
//     // Let's explicitly size our thread pool
//     let pool = rayon::ThreadPoolBuilder::new()
//         .num_threads(4)
//         .build()
//         .unwrap();

//     pool.join(test, test);
// }
