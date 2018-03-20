use futures::Async;
use futures::Poll;
use futures::future::Future;
use futures::stream::Stream;
use futures::sync::mpsc::UnboundedSender;

use void::Void;

use solicit::StreamId;

use stream_part::HttpPartStream;
use stream_part::HttpStreamPartContent;

use error::ErrorCode;

use super::*;

/// Poll the stream and enqueues frames
pub struct PumpStreamToWriteLoop<T: Types> {
    // TODO: this is not thread-safe
    pub to_write_tx: UnboundedSender<T::ToWriteMessage>,
    pub stream_id: StreamId,
    pub out_window: window_size::StreamOutWindowReceiver,
    pub stream: HttpPartStream,
}

impl<T: Types> Future for PumpStreamToWriteLoop<T> {
    type Item = ();
    type Error = Void;

    fn poll(&mut self) -> Poll<(), Void> {
        loop {
            match self.out_window.poll() {
                Ok(Async::NotReady) => return Ok(Async::NotReady),
                Ok(Async::Ready(())) => {}
                Err(window_size::StreamDead::Conn) => {
                    warn!("conn dead");
                    return Ok(Async::Ready(()));
                }
                Err(window_size::StreamDead::Stream) => {
                    warn!("stream {} dead", self.stream_id);
                    return Ok(Async::Ready(()));
                }
            }

            let part_opt = match self.stream.poll() {
                Ok(Async::NotReady) => return Ok(Async::NotReady),
                Ok(Async::Ready(r)) => r,
                Err(e) => {
                    warn!("stream error: {:?}", e);
                    let stream_end = CommonToWriteMessage::StreamEnd(self.stream_id, ErrorCode::InternalError);
                    if let Err(e) = self.to_write_tx.unbounded_send(stream_end.into()) {
                        warn!(
                            "failed to write to channel, probably connection is closed: {:?}",
                            e
                        );
                    }
                    break;
                }
            };

            match part_opt {
                Some(part) => {
                    match &part.content {
                        &HttpStreamPartContent::Data(ref d) => {
                            self.out_window.decrease(d.len());
                        }
                        &HttpStreamPartContent::Headers(_) => {}
                    }

                    let msg = CommonToWriteMessage::StreamEnqueue(self.stream_id, part);
                    if let Err(e) = self.to_write_tx.unbounded_send(msg.into()) {
                        warn!(
                            "failed to write to channel, probably connection is closed: {:?}",
                            e
                        );
                        break;
                    }

                    continue;
                }
                None => {
                    let msg = CommonToWriteMessage::StreamEnd(self.stream_id, ErrorCode::NoError);
                    if let Err(e) = self.to_write_tx.unbounded_send(msg.into()) {
                        warn!(
                            "failed to write to channel, probably connection is closed: {:?}",
                            e
                        );
                        break;
                    }

                    break;
                }
            }
        }

        Ok(Async::Ready(()))
    }
}
