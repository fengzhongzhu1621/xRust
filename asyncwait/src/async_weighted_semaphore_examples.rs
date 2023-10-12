use futures::poll;
use futures::pin_mut;

pub fn async_weighted_semaphore_example() {
    smol::block_on(async {
        let sem = async_weighted_semaphore::Semaphore::new(1);
        let a = sem.acquire(2);
        let b = sem.acquire(1);
        pin_mut!(a);
        pin_mut!(b);
        assert!(poll!(&mut a).is_pending());
        assert!(poll!(&mut b).is_pending());

        sem.release(1);
        assert!(poll!(&mut a).is_ready());
        assert!(poll!(&mut b).is_ready());
    });
}