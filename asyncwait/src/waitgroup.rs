use awaitgroup::WaitGroup;

pub fn awaitgroup_example() {
    smol::block_on(async {
        let mut wg = WaitGroup::new();
        for _ in 0..5 {
            // Create a new worker.
            let worker = wg.worker();

            let _ = smol::spawn(async {
                // Do some work...

                // This task is done all of its work.
                worker.done();
            });
        }

        // Block until all other tasks have finished their work.
        wg.wait().await;
    });
}
