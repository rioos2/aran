// Copyright 2018 The Rio Advancement Inc

pub mod error;
#[cfg(any(test, feature = "long_benchmarks"))]
pub mod tests;

use futures::sync::mpsc;
use futures::{Async, Future, Poll, Stream};
use node::runtime::ExternalMessage;

/// This kind of events is used to schedule execution in next event-loop ticks
/// Usable to make flat logic and remove recursions.
#[derive(Debug)]
pub enum InternalEvent {
    /// Shutdown the node.
    EntitlementTimeout,
    Shutdown,
}

#[derive(Debug)]
pub enum Event {
    Api(ExternalMessage),
    Internal(InternalEvent),
}

pub trait EventHandler {
    fn handle_event(&mut self, event: Event);
}

#[derive(Debug)]
pub struct HandlerPart<H: EventHandler> {
    pub handler: H,
    pub internal_rx: mpsc::Receiver<InternalEvent>,
    pub api_rx: mpsc::Receiver<ExternalMessage>,
}

impl<H: EventHandler + 'static> HandlerPart<H> {
    pub fn run(self) -> Box<Future<Item = (), Error = ()>> {
        let mut handler = self.handler;

        let fut = EventsAggregator::new(self.internal_rx, self.api_rx).for_each(move |event| {
            handler.handle_event(event);
            Ok(())
        });

        tobox(fut)
    }
}

impl Into<Event> for ExternalMessage {
    fn into(self) -> Event {
        Event::Api(self)
    }
}

impl Into<Event> for InternalEvent {
    fn into(self) -> Event {
        Event::Internal(self)
    }
}

/// Receives internal and api events and invokes `handle_event` method of handler.
/// If one of these streams closes, the aggregator stream completes immediately.
#[derive(Debug)]
pub struct EventsAggregator<S1, S2>
where
    S1: Stream,
    S2: Stream,
{
    done: bool,
    internal: S1,
    api: S2,
}

impl<S1, S2> EventsAggregator<S1, S2>
where
    S1: Stream,
    S2: Stream,
{
    pub fn new(internal: S1, api: S2) -> EventsAggregator<S1, S2> {
        EventsAggregator {
            done: false,
            internal: internal,
            api: api,
        }
    }
}

impl<S1, S2> Stream for EventsAggregator<S1, S2>
where
    S1: Stream<Item = InternalEvent>,
    S2: Stream<Item = ExternalMessage, Error = S1::Error>,
{
    type Item = Event;
    type Error = S1::Error;

    fn poll(&mut self) -> Poll<Option<Event>, Self::Error> {
        if self.done {
            Ok(Async::Ready(None))
        } else {
            match self.internal.poll()? {
                Async::Ready(Some(item)) => {
                    return Ok(Async::Ready(Some(Event::Internal(item))));
                }
                Async::Ready(None) => {
                    self.done = true;
                    return Ok(Async::Ready(None));
                }
                Async::NotReady => {}
            };
            match self.api.poll()? {
                Async::Ready(Some(item)) => {
                    return Ok(Async::Ready(Some(Event::Api(item))));
                }
                Async::Ready(None) => {
                    self.done = true;
                    return Ok(Async::Ready(None));
                }
                Async::NotReady => {}
            };

            Ok(Async::NotReady)
        }
    }
}

pub fn tobox<F: Future + 'static>(f: F) -> Box<Future<Item = (), Error = F::Error>> {
    Box::new(f.map(drop))
}
