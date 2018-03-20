use std::sync::Arc;

use futures::Async;
use futures::Poll;

use futures::stream::Stream;
use futures::task::Task;
use futures::task;

use void::Void;

use super::atomic_int_box::*;

const NOTHING: U2 = U2::V0;
const SIGNALLED: U2 = U2::V1;
const SENDER_DEAD: U2 = U2::V2;

struct Shared {
    state: AtomicU2OrBox<Task>,
}

pub struct Sender {
    shared: Arc<Shared>,
}

pub struct Receiver {
    shared: Arc<Shared>,
}

pub fn signal() -> (Sender, Receiver) {
    let shared = Arc::new(Shared {
        state: AtomicU2OrBox::from_u2(NOTHING),
    });

    (
        Sender {
            shared: shared.clone(),
        },
        Receiver { shared: shared },
    )
}

impl Drop for Sender {
    fn drop(&mut self) {
        if let DecodedBox::Box(task) = self.shared.state.swap_u2(SENDER_DEAD) {
            task.notify();
        }
    }
}

impl Sender {
    pub fn signal(&self) {
        if let DecodedRef::U2(SIGNALLED) = self.shared.state.load() {
            return;
        }

        match self.shared.state.swap_u2(SIGNALLED) {
            DecodedBox::Box(task) => task.notify(),
            DecodedBox::U2(NOTHING) => {}
            DecodedBox::U2(SIGNALLED) => {}
            DecodedBox::U2(_) => unreachable!(),
        }
    }
}

pub struct SenderDead;

impl Receiver {
    fn poll_ready(&self) -> Poll<(), SenderDead> {
        loop {
            let l = self.shared.state.load();
            match l {
                DecodedRef::U2(NOTHING) | DecodedRef::Ptr(_) => {
                    if let Ok(_) = self.shared
                        .state
                        .compare_exchange(l, DecodedBox::Box(Box::new(task::current())))
                    {
                        return Ok(Async::NotReady);
                    }
                }
                DecodedRef::U2(SIGNALLED) => {
                    if let Ok(_) = self.shared
                        .state
                        .compare_exchange(l, DecodedBox::U2(NOTHING))
                    {
                        return Ok(Async::Ready(()));
                    }
                }
                DecodedRef::U2(SENDER_DEAD) => {
                    return Err(SenderDead);
                }
                _ => unreachable!(),
            }
        }
    }
}

impl Stream for Receiver {
    type Item = ();
    type Error = Void;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        match self.poll_ready() {
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Ok(Async::Ready(())) => Ok(Async::Ready(Some(()))),
            Err(SenderDead) => Ok(Async::Ready(None)),
        }
    }
}

#[cfg(test)]
mod test {

    use futures::executor;

    use futures_misc::test::*;

    use super::*;

    #[test]
    fn test() {
        let (s, r) = signal();

        let mut r = executor::spawn(r);

        assert_eq!(Ok(Async::NotReady), r.poll_stream_notify(&notify_noop(), 1));
        assert_eq!(Ok(Async::NotReady), r.poll_stream_notify(&notify_noop(), 1));

        s.signal();

        assert_eq!(
            Ok(Async::Ready(Some(()))),
            r.poll_stream_notify(&notify_noop(), 1)
        );
        assert_eq!(Ok(Async::NotReady), r.poll_stream_notify(&notify_noop(), 1));

        s.signal();
        s.signal();

        assert_eq!(
            Ok(Async::Ready(Some(()))),
            r.poll_stream_notify(&notify_noop(), 1)
        );
        assert_eq!(Ok(Async::NotReady), r.poll_stream_notify(&notify_noop(), 1));

        drop(s);

        assert_eq!(
            Ok(Async::Ready(None)),
            r.poll_stream_notify(&notify_noop(), 1)
        );
    }

}
