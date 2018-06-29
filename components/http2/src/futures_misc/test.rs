use futures::executor::Notify;
use futures::executor::NotifyHandle;

pub fn notify_noop() -> NotifyHandle {
    struct Noop;

    impl Notify for Noop {
        fn notify(&self, _id: usize) {}
    }

    const NOOP: &'static Noop = &Noop;

    NotifyHandle::from(NOOP)
}
