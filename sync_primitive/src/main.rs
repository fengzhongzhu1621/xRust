use sync_primitive::*;

fn main() {
    arc_example();
    arc_example2();

    mutex_example1();
    // mutex_example2_poison();
    mutex_example3_drop();

    atomic_example();
    atomic_example2();

    barrier_example();

    condvar_example();

    mpsc_example();
    sync_channel_example();

    once_example();

    rwlock_example();
}
