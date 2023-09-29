use sync_primitive::*;

fn main() {
    arc_example();
    arc_example2();

    mutex_example1();
    // mutex_example2_poison();
    mutex_example3_drop();

    // atomic
    atomic_example();
    atomic_example2();
    portable_atomic_i128();
    portable_atomic_u128();
    portable_atomic_f32();
    portable_atomic_f64();
    atomic_float_example();
    atomig_example();
    atomicbox_examples();

    barrier_example();

    condvar_example();

    mpsc_example();
    sync_channel_example();

    once_example();

    rwlock_example();
}
